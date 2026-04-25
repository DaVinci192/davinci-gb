
use crate::core::cpu::CpuExec;
use super::Bus;

use super::CPU;
use super::decoder::{CB_DISPATCH, DISPATCH};
use super::instructions::jumps_calls::{op_isr_40, op_isr_48, op_isr_50, op_isr_58, op_isr_60};
use super::instructions::op_fetch_next;

impl CPU {
    pub fn step_instruction(&mut self, bus: &mut Bus) {
        loop {
            self.cycle(bus);
            if self.step == 1 {
                break;
            }
        }
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        let mut ctx = CpuExec { cpu: self, bus };

        if ctx.cpu.halted {
            if (ctx.cpu.ie & ctx.cpu.iflag) != 0 {
                ctx.cpu.halted = false;
            } else {
                return;
            }
        }

        if ctx.cpu.step == 0 {
            let opcode = ctx.cpu.ir;
            
            if ctx.cpu.handle_interrupts() {
                return;
            }
            
            if ctx.cpu.ime_scheduled {
                ctx.cpu.ime = true;
                ctx.cpu.ime_scheduled = false;
            }

            schedule_next_instruction(&mut ctx, opcode);
        } else {
            (ctx.cpu.current_op)(&mut ctx);

            ctx.cpu.m_cycles += 1;
            ctx.cpu.t_cycles += 1;
        }
    }



    pub fn handle_interrupts(&mut self) -> bool {
        if !self.ime { return false; }

        let pending = self.ie & self.iflag;
        if pending == 0 { return false; }

        let bit = pending.trailing_zeros() as u8;
        self.current_op = match bit {
            0 => op_isr_40,
            1 => op_isr_48,
            2 => op_isr_50,
            3 => op_isr_58,
            4 => op_isr_60,
            _ => unreachable!(),
        };

        self.step = 1;

        true
    }

}

fn schedule_next_instruction(ctx: &mut CpuExec, opcode: u8) {
    if opcode == 0xCB {
        op_fetch_next(ctx);
        let cb = ctx.cpu.ir;
        ctx.cpu.current_op = CB_DISPATCH[cb as usize];
    } else {
        ctx.cpu.current_op = DISPATCH[opcode as usize];
    }

    ctx.cpu.step = 1;
}