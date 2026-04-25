use super::cpu::CPU;
use super::bus::Bus;

pub struct Gameboy {
    cpu: CPU,
    bus: Bus,
}

impl Gameboy {
    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.bus);
    }
}