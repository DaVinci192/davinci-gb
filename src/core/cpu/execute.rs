
use crate::core::bus::cpu_bus_view::CpuBusView;
use crate::core::cpu::CpuExec;
use super::Bus;

use super::CPU;
use super::decoder::{CB_DISPATCH, DISPATCH};
use super::instructions::jumps_calls::{op_isr_40, op_isr_48, op_isr_50, op_isr_58, op_isr_60};
use super::instructions::op_fetch_next;

impl CPU {
    // pub fn step_instruction(&mut self, bus_view: CpuBusView) {
    //     loop {
    //         self.cycle(bus_view);
    //         if self.step == 0 {
    //             break;
    //         }
    //     }
    // }

    // step by m-cycle
    pub fn cycle(&mut self, bus_view: CpuBusView) {
        let mut ctx = CpuExec { cpu: self, bus: bus_view };

        if ctx.cpu.halted {
            if (ctx.bus.ie() & ctx.bus.iflag()) != 0 {
                ctx.cpu.halted = false;
            } else {
                ctx.cpu.m_cycles += 1;
                return;
            }
        }

        if ctx.cpu.step == 0 {
            let opcode = ctx.cpu.ir;
            
            if !handle_interrupts(&mut ctx) {
                if ctx.cpu.ime_scheduled {
                    ctx.cpu.ime = true;
                    ctx.cpu.ime_scheduled = false;
                }

                schedule_next_instruction(&mut ctx, opcode);    
            }
        }
        
        (ctx.cpu.current_op)(&mut ctx);
        ctx.cpu.m_cycles += 1;
    }

    // step by t-cycle
    pub fn tick(&mut self, bus_view: CpuBusView) {
        if self.t_cycle_curr == 3 {
            self.cycle(bus_view);
            self.t_cycle_curr = 0;
        } else {
            self.t_cycle_curr += 1;
        }
    }

}

fn schedule_next_instruction(ctx: &mut CpuExec, opcode: u8) {
    if ctx.cpu.cb_prefix {
        ctx.cpu.current_op = CB_DISPATCH[opcode as usize];
        ctx.cpu.cb_prefix = false;
    } else {
        ctx.cpu.current_op = DISPATCH[opcode as usize];
    }

    ctx.cpu.step = 1;
}

fn handle_interrupts(ctx: &mut CpuExec) -> bool {
    if !ctx.cpu.ime { return false; }

    let pending = ctx.bus.ie() & ctx.bus.iflag();
    if pending == 0 { return false; }

    let bit = pending.trailing_zeros() as u8;
    ctx.cpu.current_op = match bit {
        0 => op_isr_40,
        1 => op_isr_48,
        2 => op_isr_50,
        3 => op_isr_58,
        4 => op_isr_60,
        _ => unreachable!(),
    };

    ctx.cpu.step = 1;

    true
}