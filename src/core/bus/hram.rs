const HRAM_LO: u16 = 0xFF80;
const HRAM_HI: u16 = 0xFFFE;

pub struct Hram {
    memory: [u8; 0x007F],
}

impl Hram {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x007F],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            HRAM_LO..=HRAM_HI => self.memory[(address - HRAM_LO) as usize],
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            HRAM_LO..=HRAM_HI => self.memory[(address - HRAM_LO) as usize] = val,
            _ => unreachable!()
        }
    }
}