use eframe::egui::layers::PaintList;

use crate::debugger::debug_snapshot::bus_snapshot::BusSnapshot;
use crate::debugger::debug_snapshot::cpu_snapshot::CPUSnapshot;

use super::debug_snapshot::DebugSnapshot;

use super::super::core::gameboy::Gameboy;

pub struct Debugger {
    pub gb: Gameboy,
    pub running: bool,
}



impl Debugger {
    pub fn new() -> Self {
        Self {
            gb: Gameboy::new(),
            running: false,
        }
    }

    pub fn snapshot(&self) -> DebugSnapshot {
        DebugSnapshot { 
            cpu: CPUSnapshot::from(&self.gb.cpu), 
            bus: BusSnapshot::from(&self.gb.bus), 
        }
    }

    pub fn step_instruction(&mut self) {
        self.gb.step_instruction();
    }

    pub fn step_cycle(&mut self) {
        self.gb.step_cycle();
    }

    pub fn tick(&mut self) {
        self.gb.tick();
    }

    pub fn load_rom(&mut self, path: &str) {
        let bytes = std::fs::read(path).unwrap();
        self.gb.load_rom(&bytes);
    }

    pub fn reset(&mut self) {
        self.gb.reset();
    }

    pub fn run(&mut self) {}

    pub fn pause(&mut self) {}

    
}

