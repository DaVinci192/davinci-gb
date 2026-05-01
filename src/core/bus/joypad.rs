pub struct Joypad {
    joyp: u8,

    input: u8,
    /* 
    7: Start
    6: Select
    5: B
    4: A
    3: Down
    2: Up
    1: Left
    0: Right
     */
}

/*
 BIT  |   Direction   |   Action
-----------------------------------
  0   |   Right       |   A
  1   |   Left        |   B
  2   |   Up          |   Select
  3   |   Down        |   Start
 */

impl Default for Joypad {
    fn default() -> Self {
        Self {
            joyp: 0,
            input: 0,
        }
    }
}


impl Joypad {
    pub fn read(&self, address: u16) -> u8 {
        debug_assert!(address == 0xFF00);

        self.joyp
    }

    pub fn write(&mut self, address: u16, val: u8) {
        debug_assert!(address == 0xFF00);

        let select_mask = val | !0x30;
        self.joyp &= select_mask;

        let select = (self.joyp & 0x30) >> 4;

        let nibble = match select {
            0b00 => ((self.input >> 4) & self.input) & 0x0F,
            0b01 => self.input & 0x0F,
            0b10 => (self.input >> 4) & 0x0F,
            0b11 => 0x0F,
            _ => unreachable!()
        };

        self.joyp &= nibble | 0xF0;
    }
}