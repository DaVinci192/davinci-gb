use crate::core::bus::cpu_bus_view::CpuBusView;

use super::cpu::CPU;
use super::bus::Bus;

pub struct Gameboy {
    pub cpu: CPU,
    pub bus: Bus,
}

impl Gameboy {
    pub fn tick(&mut self) {
        let cpu_bus_view = CpuBusView {
            bus: &mut self.bus,
        };
        self.cpu.tick(cpu_bus_view);
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
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.reset();

        self.bus.cartridge.load_rom(data);

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


