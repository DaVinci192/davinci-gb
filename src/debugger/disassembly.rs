use super::backend::Debugger;
use super::opinfo::{OP_INFO, CB_OP_INFO, OpInfo};

impl Debugger {
    pub fn disassemble_window(&self, start: u16, rows: usize) -> Vec<DisassembleRow> {
        let mut pc = start;
        let mut out = vec![];

        for _ in 0..rows {
            let mut op = self.gb.bus.read8(pc);
            let mut is_cb_prefixed = false;
            let mut info = OP_INFO[op as usize];

            if op == 0xCB {
                op = self.gb.bus.read8(pc + 1);
                is_cb_prefixed = true;
                info = CB_OP_INFO[op as usize];
            }

            let mut bytes = vec![];
            for i in 0..info.len {
                bytes.push(self.gb.bus.read8(pc + i as u16));
            }

            let text = info.format_mnemonic(pc, &bytes, is_cb_prefixed);

            out.push(DisassembleRow {
                addr: pc,
                bytes,
                text,
                len: info.len,
                cycles_min: info.cycles_min,
                cycles_max: info.cycles_max,
                is_current_pc: pc == self.gb.cpu.pc,
            });

            pc = pc.wrapping_add(info.len as u16);
        }

        out
    }
}

pub struct DisassembleRow {
    pub addr: u16,
    pub bytes: Vec<u8>,
    pub text: String,
    pub cycles_min: u8,
    pub cycles_max: u8,
    pub len: u8,
    pub is_current_pc: bool,
}