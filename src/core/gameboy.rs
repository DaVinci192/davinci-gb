use super::cpu::CPU;

pub struct Gameboy {
    cpu: CPU,
}

impl Gameboy {
    pub fn cycle(&mut self) {
        self.cpu.cycle();
    }
}