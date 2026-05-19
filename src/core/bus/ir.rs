const COMMS_PORT: u16 = 0xFF56;

pub struct IR {
    comm: u8
}

impl IR {
    pub fn new() -> Self {
        Self {
            comm: 0
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        debug_assert!(address == COMMS_PORT);

        self.comm
    }

    pub fn write(&mut self, address: u16, val: u8) {
        debug_assert!(address == COMMS_PORT);

        self.comm = val;
    }
}