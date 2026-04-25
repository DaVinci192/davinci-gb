use super::debug_snapshot::DebugSnapshot;

use super::super::core::gameboy::Gameboy;

pub struct Debugger {
    pub gb: Gameboy,
    pub running: bool,
    pub breakpoints: Vec<u16>,
}



impl Debugger {
    pub fn snapshot(&self) -> DebugSnapshot {

    }

    pub fn update(&mut self) {}

    pub fn step_instruction(&mut self) {}

    pub fn run(&mut self) {}

    pub fn pause(&mut self) {}
}