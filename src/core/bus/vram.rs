const VBK: u16 = 0xFF4F;
const VRAM_LO: u16 = 0x8000;
const VRAM_HI: u16 = 0x9FFF;

const ADDR_OFFSET: usize = 0x8000;
const VRAM_BANK_SEL: u8 = 0x01;

const BASE_8000: u16 = 0x8000;
const BASE_8800: u16 = 0x9000;

pub struct Vram {
    pub vbk: u8,
    
    memory: [[u8; 0x2000]; 2],
}

impl Vram {
    pub fn new() -> Self {
        Self {
            vbk: 0,

            memory: [
                [0; 0x2000],
                [0; 0x2000],
            ]
        }
    }

    pub fn switch_bank(&mut self, bank: u8) {
        debug_assert!((bank == 1) || (bank == 0));

        self.vbk = bank;
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            VBK => self.vbk = val,
            VRAM_LO..=VRAM_HI => {
                let bank = (self.vbk & VRAM_BANK_SEL) as usize;
                self.memory[bank][(address as usize) - ADDR_OFFSET] = val;
            },
            _ => unreachable!(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            VBK => self.vbk,
            VRAM_LO..=VRAM_HI => {
                let bank = (self.vbk & VRAM_BANK_SEL) as usize;
                self.memory[bank][(address as usize) - ADDR_OFFSET]
            },
            _ => unreachable!(),
        }
    }

    // line = 0 .. 7
    // offset = 0 => read low byte; offset = 1 => read high byte
    fn read_tile_8000(&self, idx: u8, line: u8, offset: u8, y_flip: bool) -> u8 {
        let tile_base = BASE_8000 + idx as u16;
        let tile_line = if !y_flip { line as u16 } else { (7 - line) as u16 };
        let address = tile_base + (tile_line * 2) + offset as u16 ;

        self.read(address)
    }

    fn read_tile_8800(&self, idx: u8, line: u8, offset: u8, y_flip: bool) -> u8 {
        let tile_base = BASE_8800.wrapping_add(idx as i8 as u16);
        let tile_line = if !y_flip { line as u16 } else { (7 - line) as u16 };
        let address = tile_base + (tile_line * 2) + offset as u16 ;

        self.read(address)
    }

    // address mode == 0 => 8800 ; address mode == 1 => 8000
    pub fn read_tile(&self, idx: u8, address_mode: bool, line: u8, offset: u8, y_flip: bool) -> u8 {
        if !address_mode { 
            self.read_tile_8800(idx, line, offset, y_flip)
        } else {
            self.read_tile_8000(idx, line, offset, y_flip)
        }
    }


}