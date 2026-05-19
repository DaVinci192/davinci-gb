use std::collections::VecDeque;

pub mod fetcher;
pub mod oam_scanner;

use crate::core::bus::{oam::OAM, ppu::{fetcher::{Fetcher, FetcherCtx}, oam_scanner::{OamScanner, OamScannerCtx}}, vram::Vram, cram::Cram};



#[derive(Copy, Clone)]
pub struct Lcdc(pub u8);
impl Lcdc {
    fn sprite_height(&self) -> u8 {
        if self.0 & 0x04 != 0 { 16 } else { 8 }
    }

    fn bg_tilemap(&self) -> bool {
        self.0 & 0x08 != 0
    }

    fn window_tilemap(&self) -> bool {
        self.0 & 0x40 != 0
    }

    fn is_window_enabled(&self) -> bool {
        self.0 & 0x20 != 0
    }

    fn address_mode(&self) -> bool {
        self.0 & 0x10 != 0
    }

    // 0 = 8x8, 1 = 8x16
    fn obj_size(&self) -> bool {
        self.0 & 0x04 != 0
    }

    fn obj_enable(&self) -> bool {
        self.0 & 0x02 != 0
    }

    fn bg_obj_priority(&self) -> bool {
        self.0 & 0x01 != 0
    }
}


const LCDC: u16 = 0xFF40;
const STAT: u16 = 0xFF41;

const SCY: u16 = 0xFF42;
const SCX: u16 = 0xFF43;

const LY: u16 = 0xFF44;
const LYC: u16 = 0xFF45;

const BGP: u16 = 0xFF47;
const OBP0: u16 = 0xFF48;
const OBP1: u16 = 0xFF49;

const WY: u16 = 0xFF4A;
const WX: u16 = 0xFF4B;

const OBJ_BUFFER_LEN: usize = 10;
const TILEMAP_BASE: u16 = 0x9800;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PPU_MODE {
    HBLANK,
    VBLANK,
    OAM_SCAN,
    DRAWING,
}

pub struct PPU {
    lcdc: Lcdc,
    stat: u8,

    scy: u8,
    scx: u8,

    ly: u8,
    lyc: u8,

    bgp: u8,
    obp0: u8,
    obp1: u8,

    wy: u8,
    wx: u8,

    mode: PPU_MODE,
    dot: u16,

    // populated during OAM SCAN
    oam_scanner: OamScanner,

    // fetcher (mode 3)
    fetcher: Fetcher,

    // framebuffer
    framebuffer: [[u16; 160]; 144],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            lcdc: Lcdc(0x91),
            stat: 0x00,
            scy: 0x00,
            scx: 0x00,
            ly: 0x00,
            lyc: 0x00,
            bgp: 0xFC,
            obp0: 0x00,
            obp1: 0x00,
            wy: 0x00,
            wx: 0x00,

            mode: PPU_MODE::OAM_SCAN,
            dot: 0,

            oam_scanner: OamScanner::new(),
            fetcher: Fetcher::new(),

            framebuffer: [[0; 160]; 144]
        }
    }

    pub fn mode(&self) -> PPU_MODE { self.mode }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCDC => self.lcdc.0,
            STAT => self.stat,
            SCY => self.scy,
            SCX => self.scx,
            LY => self.ly,
            LYC => self.lyc,
            BGP => self.bgp,
            OBP0 => self.obp0,
            OBP1 => self.obp1,
            WY => self.wy,
            WX => self.wx,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            LCDC => self.lcdc = Lcdc(val),
            STAT => self.stat |= (val & 0b01111000), // only bits 6-3 are r/w
            SCY => self.scy = val,
            SCX => self.scx = val,
            LY => {}, // read-only
            LYC => self.lyc = val,
            BGP => self.bgp = val,
            OBP0 => self.obp0 = val,
            OBP1 => self.obp1 = val,
            WY => self.wy = val,
            WX => self.wx = val,
            _ => unreachable!(),
        }
    }
}


impl PPU {

    fn advance_scanline(&mut self) {
        self.ly += 1;

        if self.fetcher.is_window_mode() {
            self.fetcher.ly_window += 1;
        }

        self.fetcher.reset();
        self.ly += 1;
    }

    pub fn tick(&mut self, oam: &mut OAM, vram: &mut Vram, cram: &mut Cram) {
        match self.mode {
            PPU_MODE::OAM_SCAN => {
                let ctx = OamScannerCtx {
                    lcdc: self.lcdc,
                    ly: self.ly,
                    dot: self.dot,
                };

                self.oam_scanner.tick(oam, &ctx);

                if self.oam_scanner.finished { self.mode = PPU_MODE::DRAWING }
            },
            PPU_MODE::DRAWING => {
                let mut ctx = FetcherCtx {
                    lcdc: self.lcdc,
                    scx: self.scx,
                    scy: self.scy,
                    wx: self.wx,
                    wy: self.wy,
                    ly: self.ly,

                    framebuffer: &mut self.framebuffer,

                    cram: cram,
                    vram: vram,

                    obj_buffer: &mut self.oam_scanner.obj_buffer,
                    obj_buffer_len: self.oam_scanner.obj_buffer_idx+1,
                };

                self.fetcher.tick(&mut ctx);

                if self.fetcher.finished { 
                    self.mode = PPU_MODE::HBLANK;
                }

                self.dot += 1;
            },
            PPU_MODE::HBLANK => {
                if self.dot == 455 { 
                    if self.ly == 144 {
                        self.mode = PPU_MODE::VBLANK;
                        self.dot += 1;
                    } else {
                        self.dot = 0;
                        self.advance_scanline();
                        self.mode = PPU_MODE::OAM_SCAN;
                    }
                }
            },
            PPU_MODE::VBLANK => {

                self.dot += 1;
            }

        }

    }


}
