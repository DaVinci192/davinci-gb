use crate::core::bus::{oam::OAM, ppu::Lcdc};



const OBJ_BUFFER_LEN: usize = 10;

pub struct OamScannerCtx {
    pub lcdc: Lcdc,
    pub ly: u8,
    pub dot: u16,
}

pub struct OamScanner {
    curr_dot: u8,

    obj_temp: [u8; 4],
    pub obj_buffer: [[u8; 4]; OBJ_BUFFER_LEN],
    pub obj_buffer_idx: usize,

    pub finished: bool
}

impl OamScanner {
    pub fn new() -> Self {
        Self {
            curr_dot: 0,

            obj_temp: [0; 4],
            obj_buffer: [[0; 4]; OBJ_BUFFER_LEN],
            obj_buffer_idx: 0,

            finished: false
        }
    }

    pub fn tick(&mut self, oam: &mut OAM, ctx: &OamScannerCtx) {
        match self.curr_dot {
            0 => { // dot 0: copy OAM data to internal memory
                if ctx.dot == 80 { 
                    self.finished = true;
                    return;
                }

                // this dot is 0, 2, 4, ..., 78
                // idx = dot / 2
                let sprite_idx = (ctx.dot / 2) as usize;
                oam.read_sprite(sprite_idx, &mut self.obj_temp);

                self.curr_dot += 1;
            },
            1 => { // dot 1: check if sprite should be added to buffer
                // if buffer full, don't do anything
                if self.obj_buffer_idx == OBJ_BUFFER_LEN { return; }

                // parse metadata for obj
                let y_pos = self.obj_temp[0];
                let x_pos = self.obj_temp[1];
                let height = ctx.lcdc.sprite_height();

                // add to buffer if conditions satisfied
                if (x_pos > 0) &&
                    (ctx.ly + 16 > y_pos) &&
                    ((ctx.ly + 16) < (y_pos + height)) {
                        self.obj_buffer[self.obj_buffer_idx].copy_from_slice(&mut self.obj_temp);
                        self.obj_buffer_idx += 1;
                    }

                self.curr_dot = 0;
            },
            _ => unreachable!(),
        }
    }
}