use eframe::egui::accesskit::Color;


const BCPS: u16 = 0xFF68;
const BCPD: u16 = 0xFF69;
const OCPS: u16 = 0xFF6A;
const OCPD: u16 = 0xFF6B;

pub struct ColorSpec(pub u8);

impl ColorSpec {
    fn address(&self) -> u8 {
        self.0 & 0b00111111
    }

    fn auto_increment(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

pub struct Cram {
    bcps: ColorSpec, // background color palette spec
    bcpd: u8, // background color palette data
    bg_memory: [u8; 64],

    ocps: ColorSpec, // object color palette spec
    ocpd: u8, // object color palette data
    obj_memory: [u8; 64],

}

impl Cram {
    pub fn new() -> Self {
        Self {
            bcps: ColorSpec(0),
            bcpd: 0xFF,
            bg_memory: [0xFF; 64],

            ocps: ColorSpec(0),
            ocpd: 0xFF,
            obj_memory: [0xFF; 64],
        }
    }

    fn read_bg_color(&self) -> u8 {
        let address = self.bcps.address();

        self.bg_memory[address as usize]
    }

    pub fn read_bg_color_direct(&self, address: usize) -> u16 {
        let low = self.bg_memory[address];
        let high = self.bg_memory[address+1];

        (low as u16) | ((high as u16) << 8)
    }

    fn read_obj_color(&self) -> u8 {
        let address = self.ocps.address();

        self.obj_memory[address as usize]
    }

    pub fn read_obj_color_direct(&self, address: usize) -> u16 {
        let low = self.obj_memory[address];
        let high = self.obj_memory[address+1];

        (low as u16) | ((high as u16) << 8)
    }

    fn write_bg_color(&mut self, val: u8) {
        let address = self.bcps.address();

        self.bg_memory[address as usize] = val;

        if self.bcps.auto_increment() {
            self.bcps.0 += 1;
        }
    }

    fn write_obj_color(&mut self, val: u8) {
        let address = self.ocps.address();

        self.obj_memory[address as usize] = val;

        if self.ocps.auto_increment() {
            self.ocps.0 += 1;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            BCPS => self.bcps.0,
            BCPD => self.read_bg_color(),
            OCPS => self.ocps.0,
            OCPD => self.read_obj_color(),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            BCPS => self.bcps = ColorSpec(val),
            BCPD => self.write_bg_color(val),
            OCPS => self.ocps = ColorSpec(val),
            OCPD => self.write_obj_color(val),
            _ => unreachable!(),
        }
    }


}