const OAM_DMA: u16 = 0xFF46;

pub struct OamDma {
    dma: u8,

    pub is_active: bool,
    pub is_reading_cartridge: bool,
}


impl OamDma {
    pub fn new() -> Self {
        Self {
            dma: 0xFF,
            is_active: false,
            is_reading_cartridge: false,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            OAM_DMA => self.dma,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            OAM_DMA => self.dma = val,
            _ => unreachable!(),
        }
    } 
}