use super::cpu::CPU;
use super::bus::Bus;
use super::cartridge::Cartridge;

pub struct Gameboy {
    pub cpu: CPU,
    pub bus: Bus,
    pub cartridge: Option<Cartridge>,
}

impl Gameboy {
    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        self.bus.tick_timer();
    }

    pub fn step_cycle(&mut self) {
        for _ in 0..4 {
            self.tick();
        }
    }

    pub fn step_instruction(&mut self) {
        loop {
            self.step_cycle();
            if self.cpu.step == 0 {
                break;
            }
        }
        // self.cpu.step_instruction(&mut self.bus);
    }

    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            cartridge: None,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.reset();

        for (i, b) in data.iter().enumerate() {
            self.bus.memory[i] = *b;
        }

        // self.cpu.a = 0x02;

        // self.bus.memory[0x0100] = 0x28;
        // self.bus.memory[0x0101] = 0x05;
        // self.bus.memory[0x0106] = 0x1F;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
    }
}


