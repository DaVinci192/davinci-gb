use crate::debugger::{debug_snapshot::{DebugSnapshot, cpu_snapshot::CPUSnapshot}, disassembly::DisassembleRow};

use super::backend::Debugger;

pub struct DebuggerView<'a, 'b> {
    debugger: &'a mut Debugger,
    pub snapshot: &'b DebugSnapshot,
}


impl<'a, 'b> DebuggerView<'a, 'b> {
    pub fn new(debugger: &'a mut Debugger, snapshot: &'b DebugSnapshot) -> Self {
        Self {
            debugger: debugger,
            snapshot: snapshot,
        }
    }

    pub fn disassembly(&self, pc: u16, n_rows: usize) -> Vec<DisassembleRow> {
        self.debugger.disassemble_window(pc, n_rows)
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.debugger.gb.bus.read8(address)
    }

    pub fn step_instruction(&mut self) {
        self.debugger.step_instruction();
    }

    pub fn step_cycle(&mut self) {
        self.debugger.step_cycle();
    }

    pub fn tick(&mut self) {
        self.debugger.tick();
    }

    pub fn reset(&mut self) {
        self.debugger.reset();
    }
}
