pub struct Bus {
    memory: [u8; 0x10000],   
}

impl Bus {
    pub fn read8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write8(&mut self, address: u16, val: u8) {
        self.memory[address as usize] = val;
    }
}