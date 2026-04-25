use crate::core::cpu::instructions::op_fetch_next;
use crate::core::cpu::registers::Reg16;

use super::super::alu;
use super::super::CpuExec;


// 16-bit ops are 2 M-cycles
// this is artificially modelled by using the 
// first M-cycle to load the operand into temp16

/* ADD16 */

pub fn op_add_hl_r16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp16 = ctx.cpu.get_reg16(reg);
            ctx.cpu.step += 1;
        },
        2 => {
            alu::alu_add16(ctx.cpu, ctx.cpu.temp16);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_add_hl_bc(ctx: &mut CpuExec) { op_add_hl_r16(ctx, Reg16::BC) }
pub fn op_add_hl_de(ctx: &mut CpuExec) { op_add_hl_r16(ctx, Reg16::DE) }
pub fn op_add_hl_hl(ctx: &mut CpuExec) { op_add_hl_r16(ctx, Reg16::HL) }

/* DEC16 */

pub fn op_dec_r16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp16 = ctx.cpu.get_reg16(reg);
            ctx.cpu.step += 1;
        },
        2 => {
            let res = alu::alu_dec16(ctx.cpu.temp16);
            ctx.cpu.set_reg16(reg, res);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_dec_bc(ctx: &mut CpuExec) { op_dec_r16(ctx, Reg16::BC) }
pub fn op_dec_de(ctx: &mut CpuExec) { op_dec_r16(ctx, Reg16::DE) }
pub fn op_dec_hl(ctx: &mut CpuExec) { op_dec_r16(ctx, Reg16::HL) }

/* INC16 */

pub fn op_inc_r16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp16 = ctx.cpu.get_reg16(reg);
            ctx.cpu.step += 1;
        },
        2 => {
            let res = alu::alu_inc16(ctx.cpu.temp16);
            ctx.cpu.set_reg16(reg, res);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_inc_bc(ctx: &mut CpuExec) { op_inc_r16(ctx, Reg16::BC) }
pub fn op_inc_de(ctx: &mut CpuExec) { op_inc_r16(ctx, Reg16::DE) }
pub fn op_inc_hl(ctx: &mut CpuExec) { op_inc_r16(ctx, Reg16::HL) }
