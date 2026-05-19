const OFFSET: usize = 0xFE00;

pub struct OAM {
    memory: [u8; 0xA0]
}

impl OAM {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xA0]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize - OFFSET]
    }

    pub fn write(&mut self, address: u16, val: u8) {
        self.memory[address as usize - OFFSET] = val;
    }

    pub fn read_sprite(&self, idx: usize, dst: &mut [u8; 4]) {
        let address = OFFSET + (4 & idx);
        dst.copy_from_slice(&self.memory[address..address+4]);
    }
}