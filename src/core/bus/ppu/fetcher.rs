use std::collections::VecDeque;

use crate::core::bus::{cram::Cram, ppu::OBJ_BUFFER_LEN, vram::Vram};
use super::Lcdc;

const TILEMAP_BASE: u16 = 0x9800;

enum FETCHER_STATE {
    FETCH_TILE_NO,
    FETCH_TILE_LOW,
    FETCH_TILE_HIGH,
    PUSH_TO_FIFO,
}

#[derive(PartialEq, Eq)]
enum BG_FETCHER_TILE_TYPE {
    BACKGROUND,
    WINDOW,
}

#[derive(Clone, Copy)]
pub struct TileAttr(pub u8);

impl TileAttr {
    fn y_flip(&self) -> bool {
        self.0 >> 6 != 0
    }

    fn x_flip(&self) -> bool {
        self.0 >> 5 != 0
    }

    fn bank(&self) -> u8 {
        (self.0 >> 3) & 0x01
    }

    fn cgb_palette(&self) -> u8 {
        self.0 & 0x07
    }

    fn priority(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

#[derive(Clone, Copy)]
pub struct Pixel(pub u8);

// Bit 7: priority bit
// Bit 6-4: palette
// Bit 3-2: unused
// Bit 0-1: color index

impl Pixel {
    fn color_index(&self) -> u8 {
        self.0 & 0x03
    }

    fn palette(&self) -> u8 {
        (self.0 >> 4) & 0x07
    }

    fn priority(&self) -> bool {
        self.0 & 0x80 != 0
    }

    fn to_cram_address(&self) -> usize {
        // 8 palettes, 4 colors/palette, 2 bytes/color
        let palette_base = self.palette() * 8;
        let color_offset = self.color_index() * 2;

        (palette_base + color_offset) as usize
    }
}

#[derive(Clone, Copy)]
pub struct SpritePixel(pub u16);

// Bit 8-11: sprite index in obj buffer
// lower 8 -> Pixel()

impl SpritePixel {
    fn idx(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    fn data(&self) -> Pixel {
        Pixel((self.0 & 0x00FF) as u8)
    }
}

pub struct FetcherCtx<'a, 'b, 'c> {
    pub lcdc: Lcdc,
    pub scx: u8,
    pub scy: u8,
    pub wx: u8,
    pub wy: u8,
    pub ly: u8,

    pub framebuffer: &'c mut [[u16; 160]; 144],

    pub cram: &'a mut Cram,
    pub vram: &'a mut Vram,
    pub obj_buffer: &'b [[u8; 4]; OBJ_BUFFER_LEN],
    pub obj_buffer_len: usize,
}

pub type FetchOp = fn(&mut Fetcher, &mut FetcherCtx);

// const STARTUP_PROGRAM: [FetchOp; 3] = [];
const BG_PROGRAM: [FetchOp; 2] = [Fetcher::bg_fetch, Fetcher::push_to_bg_fifo];
const SPRITE_PROGRAM: [FetchOp; 2] = [Fetcher::sprite_fetch, Fetcher::push_to_sprite_fifo];

pub struct Fetcher {
    curr_dot: u8,
    pub finished: bool,

    fetch_pc: usize,
    fetch_program: [FetchOp; 2],
    fetch_instruction: FetchOp,

    sprite_render_requested: bool,
    sprite_processing: bool,
    sprite_request_idx: usize,
    
    y_condition: bool,
    x_pixel_counter: u8,

    fetcher_x: u8, //  0 .. 31

    wind_x: u8, // 0 .. 31
    pub ly_window: u8, // like LY for window, 0 .. 144


    fetcher_tile_type: BG_FETCHER_TILE_TYPE,
    fetcher_tilemap_idx: u16,

    fetch_tile_idx: u8,
    fetch_tile_attributes: TileAttr,

    fetch_tile_low: u8,
    fetch_tile_high: u8,

    bg_fifo: VecDeque<Pixel>,
    sprite_fifo: VecDeque<SpritePixel>,
}


impl Fetcher {
    pub fn new() -> Self {
        Self {
            curr_dot: 0,
            finished: false,

            fetch_pc: 0,
            fetch_program: BG_PROGRAM,
            fetch_instruction: BG_PROGRAM[0],

            sprite_render_requested: false,
            sprite_processing: false,
            sprite_request_idx: 0,

            y_condition: false,
            x_pixel_counter: 0,

            fetcher_x: 0,

            wind_x: 0,
            ly_window: 0,

            fetcher_tile_type: BG_FETCHER_TILE_TYPE::BACKGROUND,
            fetcher_tilemap_idx: 0,

            fetch_tile_idx: 0,
            fetch_tile_attributes: TileAttr(0),

            fetch_tile_low: 0,
            fetch_tile_high: 0,

            bg_fifo: VecDeque::new(),
            sprite_fifo: VecDeque::new(),
        }
    }


    pub fn reset(&mut self) {
        self.curr_dot = 0;
        self.finished = false;

        self.fetch_pc = 0;
        self.fetch_program = BG_PROGRAM;

        self.sprite_render_requested = false;
        self.sprite_processing = false;
        self.sprite_request_idx = 0;

        self.y_condition = false;
        self.x_pixel_counter = 0;

        self.fetcher_x = 0;
        self.wind_x = 0;   

        self.ly_window = 0; // TODO this should derive from ly on init

        self.fetcher_tile_type = BG_FETCHER_TILE_TYPE::BACKGROUND;
        self.fetcher_tilemap_idx = 0;

        self.fetch_tile_idx = 0;
        // self.fetch_tile_attributes

        self.fetch_tile_low = 0;
        self.fetch_tile_high = 0;

        self.bg_fifo = VecDeque::new();
        self.sprite_fifo = VecDeque::new();
        
    }

    pub fn is_window_mode(&self) -> bool {
        self.fetcher_tile_type == BG_FETCHER_TILE_TYPE::WINDOW
    }

    fn is_window_tile(&mut self, ctx: &FetcherCtx) -> bool {
        (self.x_pixel_counter >= ctx.wx) && self.y_condition
    }

    fn push_to_framebuffer(&mut self, ctx: &mut FetcherCtx, pixel: Pixel, is_sprite: bool) {
        let address = pixel.to_cram_address();

        if !is_sprite {
            let color = ctx.cram.read_bg_color_direct(address);
            ctx.framebuffer[self.x_pixel_counter as usize][ctx.ly as usize] = color;
        } else {
            let color = ctx.cram.read_obj_color_direct(address);
            ctx.framebuffer[self.x_pixel_counter as usize][ctx.ly as usize] = color;
        }
    }

    fn render_pixel(&mut self, ctx: &mut FetcherCtx) {
        // if self.x_pixel_counter >= 160 { return; } // handled at end of function call when increment is called
        if self.bg_fifo.is_empty() { return; }

        // check if pixels need to be discarded
        let shift_len = ctx.scx % 8;
        if self.x_pixel_counter < shift_len {
            self.bg_fifo.pop_back();
            if !self.sprite_fifo.is_empty() { self.sprite_fifo.pop_back(); }
        }

        // sprite fifo and bg fifo contain at least 1 pixel
        if !self.sprite_fifo.is_empty() {
            let bg_pixel = self.bg_fifo.pop_back().unwrap();
            let sprite_pixel = self.sprite_fifo.pop_back().unwrap().data();

            if !ctx.lcdc.bg_obj_priority() {
                self.push_to_framebuffer(ctx, sprite_pixel, true);
            } else if !bg_pixel.priority() && !sprite_pixel.priority() {
                self.push_to_framebuffer(ctx, sprite_pixel, true);
            } else if bg_pixel.color_index() == 0 {
                self.push_to_framebuffer(ctx, sprite_pixel, true);
            } else {
                self.push_to_framebuffer(ctx, bg_pixel, false);
            }
        } else {
            let bg_pixel = self.bg_fifo.pop_back().unwrap();
            self.push_to_framebuffer(ctx, bg_pixel, false);
        }

        self.x_pixel_counter += 1;

        if self.x_pixel_counter >= 160 {
            self.finished = true;
        }
    }


    fn check_window(&mut self, ctx: &mut FetcherCtx) -> bool {
        self.is_window_tile(ctx) && ctx.lcdc.is_window_enabled()
    }

    fn switch_tilemap(&mut self, ctx: &mut FetcherCtx) {
        match self.curr_dot {
            1..6 => {
                self.curr_dot += 1;
            },
            6 => {
                self.fetcher_tile_type = BG_FETCHER_TILE_TYPE::WINDOW;
                self.curr_dot = 0;
            },
            _ => unreachable!(),
        }
    }

    pub fn bg_fetch(&mut self, ctx: &mut FetcherCtx) {
        match self.curr_dot {
            1 => { // fetch tile no. dot 1
                let mut tilemap_idx = TILEMAP_BASE;

                if (ctx.lcdc.bg_tilemap() && !self.is_window_tile(&ctx)) ||
                    (ctx.lcdc.window_tilemap() && self.is_window_tile(&ctx)) {
                    tilemap_idx |= 0x400; // set bit 10
                }

                if self.is_window_tile(&ctx) && ctx.lcdc.is_window_enabled() {
                    self.fetcher_tile_type = BG_FETCHER_TILE_TYPE::WINDOW;
                } else {
                    self.fetcher_tile_type = BG_FETCHER_TILE_TYPE::BACKGROUND;
                }

                match self.fetcher_tile_type {
                    BG_FETCHER_TILE_TYPE::BACKGROUND => {
                        let x_tile = ((ctx.scx >> 3) + self.fetcher_x) & 0x1F;
                        let y_pixel = (ctx.ly + ctx.scy) & 0xFF;
                        
                        let y_tile = (y_pixel >> 3) as u8;

                        tilemap_idx |= ((y_tile as u16 ) << 5) | (x_tile as u16);
                    }
                    BG_FETCHER_TILE_TYPE::WINDOW => {
                        let y_tile = (self.ly_window >> 3) as u8;

                        tilemap_idx |= ((y_tile as u16 ) << 5) | (self.wind_x as u16);
                    }
                }       

                self.fetcher_tilemap_idx = tilemap_idx;
                
                self.curr_dot += 1;
            },
            2 => { // fetch tile no. dot 2
                ctx.vram.switch_bank(0);
                self.fetch_tile_idx = ctx.vram.read(self.fetcher_tilemap_idx);

                ctx.vram.switch_bank(1);
                self.fetch_tile_attributes = TileAttr(ctx.vram.read(self.fetcher_tilemap_idx));

                ctx.vram.switch_bank(0);

                self.curr_dot += 1;
            },
            3 => { // fetch tile low dot 1
                let bank = self.fetch_tile_attributes.bank();
                ctx.vram.switch_bank(bank);

                self.curr_dot += 1;
            },
            4 => { // fetch tile low dot 2
                let address_mode = ctx.lcdc.address_mode();
                let line = if self.fetcher_tile_type == BG_FETCHER_TILE_TYPE::BACKGROUND { ctx.ly & 0x07 } else { self.ly_window & 0x07 };
                let offset: u8 = 0; // low
                let y_flip = self.fetch_tile_attributes.y_flip();

                self.fetch_tile_low = ctx.vram.read_tile(self.fetch_tile_idx, address_mode, line, offset, y_flip);

                self.curr_dot += 1;
            },
            5 => { // fetch tile high dot 1
                let bank = self.fetch_tile_attributes.bank();
                ctx.vram.switch_bank(bank);

                self.curr_dot += 1;
            },
            6 => { // fetch tile high dot 2
                let address_mode = ctx.lcdc.address_mode();
                let line = if self.fetcher_tile_type == BG_FETCHER_TILE_TYPE::BACKGROUND { ctx.ly & 0x07 } else { self.ly_window & 0x07 };
                let offset: u8 = 1; // high
                let y_flip = self.fetch_tile_attributes.y_flip();


                self.fetch_tile_high = ctx.vram.read_tile(self.fetch_tile_idx, address_mode, line, offset, y_flip);

                self.curr_dot = 0;

                if self.fetcher_tile_type == BG_FETCHER_TILE_TYPE::BACKGROUND {
                    self.fetcher_x += 1;
                } else {
                    self.wind_x += 1;
                }
            },
            _ => unreachable!(),
        }
    }


    pub fn sprite_fetch(&mut self, ctx: &mut FetcherCtx) {
        match self.curr_dot {
            1 => {
                self.curr_dot += 1;
                self.sprite_processing = true;
                self.sprite_render_requested = false;
            },
            2 => {
                let y_pos = ctx.obj_buffer[self.sprite_request_idx][0];

                if ctx.lcdc.obj_size() && (y_pos - ctx.ly > 8) {
                    self.fetch_tile_idx = ctx.obj_buffer[self.sprite_request_idx][2] + 1;
                } else {
                    self.fetch_tile_idx = ctx.obj_buffer[self.sprite_request_idx][2]
                }

                self.fetch_tile_attributes = TileAttr(ctx.obj_buffer[self.sprite_request_idx][3]);
                self.curr_dot += 1;
            },
            3 => {
                let bank = self.fetch_tile_attributes.bank();
                ctx.vram.switch_bank(bank);

                self.curr_dot += 1;
            },
            4 => {
                let y_pos = ctx.obj_buffer[self.sprite_request_idx][0];
                let mut line: u8 = 0;
                if ctx.lcdc.obj_size() && (y_pos - ctx.ly > 8) {
                    line = y_pos - ctx.ly - 8;
                } else {
                    line = y_pos - ctx.ly;
                } 

                let address_mode = true; // 8000 addressing
                let offset: u8 = 0; // low
                let y_flip = self.fetch_tile_attributes.y_flip();

                self.fetch_tile_low = ctx.vram.read_tile(self.fetch_tile_idx, address_mode, line, offset, y_flip);

                self.curr_dot += 1;
            },
            5 => {
                let bank = self.fetch_tile_attributes.bank();
                ctx.vram.switch_bank(bank);

                self.curr_dot += 1;
            },
            6 => {
                let y_pos = ctx.obj_buffer[self.sprite_request_idx][0];
                let mut line: u8 = 0;
                if ctx.lcdc.obj_size() && (y_pos - ctx.ly > 8) {
                    line = y_pos - ctx.ly - 8;
                } else {
                    line = y_pos - ctx.ly;
                } 

                let address_mode = true; // 8000 addressing
                let offset: u8 = 1; // high
                let y_flip = self.fetch_tile_attributes.y_flip();

                self.fetch_tile_low = ctx.vram.read_tile(self.fetch_tile_idx, address_mode, line, offset, y_flip);

                self.curr_dot = 0;
            },
            _ => unreachable!(),
        }

    }

    pub fn check_sprite(&mut self, ctx: &mut FetcherCtx) {
        for sprite_idx in 0..ctx.obj_buffer_len {
            let sprite = ctx.obj_buffer[sprite_idx];
            let x_pos = sprite[1];

            if x_pos == self.x_pixel_counter + 8 {
                self.sprite_render_requested = true;
                self.sprite_request_idx = sprite_idx;
            }
        }
    }

    fn build_bg_pixels(&mut self, ctx: &mut FetcherCtx) -> [Pixel; 8] {
        let x_flip = self.fetch_tile_attributes.x_flip();
        let priority = self.fetch_tile_attributes.priority();
        let palette = self.fetch_tile_attributes.cgb_palette();

        let mut pixel_base: u8 = 0x00;

        if priority { pixel_base |= 0x80; }
        pixel_base |= palette << 4;

        let mut pixels: [u8; 8] = [pixel_base; 8];


        if !x_flip {
            for idx in 0..8 {
                let low_bit = (self.fetch_tile_low >> idx) & 0x01;
                let high_bit = (self.fetch_tile_high >> idx) & 0x01;

                pixels[idx] |= low_bit | (high_bit << 1);
            }
        } else {
            for idx in (0..8).rev() {
                let low_bit = (self.fetch_tile_low >> idx) & 0x01;
                let high_bit = (self.fetch_tile_high >> idx) & 0x01;

                pixels[idx] |= low_bit | (high_bit << 1);

            }
        }

        let mut res: [Pixel; 8] = [Pixel(0); 8];
        for idx in 0..8 {
            res[idx] = Pixel(pixels[idx]);
        }

        res
    }

    // reuse build_bg_pixel logic
    fn build_sprite_pixels(&mut self, ctx: &mut FetcherCtx) -> [SpritePixel; 8] {
        let pixels = self.build_bg_pixels(ctx);

        let mut res: [SpritePixel; 8] = [SpritePixel(0); 8];
        for idx in 0..8 {
            res[idx] = SpritePixel(pixels[idx].0 as u16 | ((self.sprite_request_idx as u16) << 8))
        }

        res
    }

    pub fn push_to_bg_fifo(&mut self, ctx: &mut FetcherCtx) {
        match self.curr_dot  {
            1 => {
                if self.bg_fifo.is_empty() {
                    let pixels = self.build_bg_pixels(ctx);

                    for pixel in pixels {
                        self.bg_fifo.push_back(pixel);
                    }

                    self.fetch_pc = 0;
                } else {
                    self.fetch_pc = 1;
                }

                self.curr_dot = 0;
            }
            _ => unreachable!(),
        }
    }

    pub fn push_to_sprite_fifo(&mut self, ctx: &mut FetcherCtx) {
        match self.curr_dot {
            1 => {
                let pixels = self.build_sprite_pixels(ctx);

                // deal with overlapping pixels
                let fifo_len= self.sprite_fifo.len();
                
                for (idx, pixel) in pixels.into_iter().enumerate() {
                    // LSB is 0 in "pixel", LSB is at end of self.sprite_fifo
                    // pixels[idx] overlaps with sprite_fifo[8 - len(sprite_fifo)]
                    if (fifo_len > 0) && (idx < fifo_len) {
                        let existing_pixel = self.sprite_fifo[fifo_len - idx - 1];

                        // check if new pixel is higher priority
                        if pixel.idx() < existing_pixel.idx() {
                            self.sprite_fifo[fifo_len - idx - 1] = pixel;
                        }
                    }
                }

                // push remaining pixels
                if fifo_len < 8 {
                    for idx in fifo_len..8 {
                        self.sprite_fifo.push_back(pixels[idx]);
                    }
                }

                self.curr_dot = 0;
                self.fetch_pc = 0;
                self.fetch_program = BG_PROGRAM;
                self.sprite_processing = false;
            },
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, ctx: &mut FetcherCtx) {

        if self.sprite_render_requested && !self.bg_fifo.is_empty() {
            self.fetch_program = SPRITE_PROGRAM;
            self.fetch_pc = 0;

            self.curr_dot = 0;

        } 
        
        if self.curr_dot == 0 {
            self.fetch_instruction = self.fetch_program[self.fetch_pc];
            self.curr_dot = 1
        }
        
        if !self.sprite_processing {
            self.render_pixel(ctx);
            self.check_sprite(ctx);
        }

        (self.fetch_instruction)(self, ctx);

        
    }
        
    
    /* 
    if !sprite_requested { 
        bg_fetch; 
        push_to_bg_fifo
    } else {
        sprite_fetch
        push_to_sprite_fifo
    }
    
    render_pixel (consumes pixels in fifo, advances x_pixel_counter)
    check_sprite
    
     */

    
}