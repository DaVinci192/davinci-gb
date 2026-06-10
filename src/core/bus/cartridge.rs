// minimal cartridge with no MBC

const ROM_LO: u16 = 0x0000;
const ROM_HI: u16 = 0x7FFF;

const RAM_LO: u16 = 0xA000;
const RAM_HI: u16 = 0xBFFF;

pub struct Cartridge {
    rom: [u8; 0x8000],
    ram: [u8; 0x2000],
}


impl Cartridge {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x8000],
            ram: [0; 0x2000],
        }
    }

    pub fn read(&self, address: u16) -> u8  {
        match address {
            ROM_LO..=ROM_HI => self.rom[address as usize],
            RAM_LO..=RAM_HI => self.ram[(address - RAM_LO) as usize],
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            ROM_LO..=ROM_HI => {}, // read only - do nothing
            RAM_LO..=RAM_HI => self.ram[(address - RAM_LO) as usize] = val,
            _ => unreachable!(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, b) in rom.iter().enumerate() {
            self.rom[i] = *b;
        }
    }
}