
const WBK: u16 = 0xFF70;

const BANK0_LO: u16 = 0xC000;
const BANK0_HI: u16 = 0xCFFF;

// switchable banks
const BANKX_LO: u16 = 0xD000;
const BANKX_HI: u16 = 0xDFFF;

// echo ram, use is prohibited
const ERAM_LO: u16 = 0xE000;
const ERAM_HI: u16 = 0xFDFF;


pub struct Wram {
    // 32 KiB of RAM in CGB Mode
    // split into 8 banks of 4 KiB each
    memory: [[u8; 0x1000]; 8],

    // 0xFF70, WRAM Bank
    pub wbk: u8,
}


impl Wram {
    pub fn new() -> Self {
        Self {
            memory: [[0; 0x1000]; 8],
            wbk: 0,
        }
    }

    fn selected_bank(&self) -> usize {
        let bank = self.wbk & 0x07; // last 3 bits
        
        if bank == 0 { 
            return 1 
        } else {
            return bank as usize
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            WBK => self.wbk,
            BANK0_LO..=BANK0_HI => {
                let idx = (address - BANK0_LO) as usize;
                self.memory[0][idx]
            },
            BANKX_LO..=BANKX_HI => {
                let idx = (address - BANKX_LO) as usize;
                let bank = self.selected_bank();

                self.memory[bank][idx]
            },
            ERAM_LO..=ERAM_HI => 0xFF, // echo ram should not be used 
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            WBK => self.wbk = val,
            BANK0_LO..=BANK0_HI => {
                let idx = (address - BANK0_LO) as usize;
                self.memory[0][idx] = val;
            },
            BANKX_LO..=BANKX_HI => {
                let idx = (address - BANKX_LO) as usize;
                let bank = self.selected_bank();

                self.memory[bank][idx] = val;
            },
            ERAM_LO..=ERAM_HI => {}, // echo ram should not be used
            _ => unreachable!(),
        }
    }
}