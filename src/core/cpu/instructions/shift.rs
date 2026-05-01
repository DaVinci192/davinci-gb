use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_hl, op_write_hl};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CpuExec;

/* RL, RLA */

pub fn op_rl_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_rl(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_rl_b(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::B); }
pub fn op_rl_c(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::C); }
pub fn op_rl_d(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::D); }
pub fn op_rl_e(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::E); }
pub fn op_rl_h(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::H); }
pub fn op_rl_l(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::L); }
pub fn op_rl_a(ctx: &mut CpuExec) { op_rl_r8(ctx, Reg::A); }

pub fn op_rla(ctx: &mut CpuExec) { 
    debug_assert!(ctx.cpu.step == 1);
    
    alu::alu_rla(ctx.cpu);
    op_fetch_next(ctx);
}

pub fn op_rl_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_rl(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;
            
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* RLC, RLCA */

pub fn op_rlc_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_rlc(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_rlc_b(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::B); }
pub fn op_rlc_c(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::C); }
pub fn op_rlc_d(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::D); }
pub fn op_rlc_e(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::E); }
pub fn op_rlc_h(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::H); }
pub fn op_rlc_l(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::L); }
pub fn op_rlc_a(ctx: &mut CpuExec) { op_rlc_r8(ctx, Reg::A); }

pub fn op_rlca(ctx: &mut CpuExec) { 
    debug_assert!(ctx.cpu.step == 1);
    
    alu::alu_rlca(ctx.cpu);
    op_fetch_next(ctx);
}

pub fn op_rlc_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_rlc(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* RR, RRA */

pub fn op_rr_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_rr(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_rr_b(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::B); }
pub fn op_rr_c(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::C); }
pub fn op_rr_d(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::D); }
pub fn op_rr_e(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::E); }
pub fn op_rr_h(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::H); }
pub fn op_rr_l(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::L); }
pub fn op_rr_a(ctx: &mut CpuExec) { op_rr_r8(ctx, Reg::A); }

pub fn op_rra(ctx: &mut CpuExec) { 
    debug_assert!(ctx.cpu.step == 1);
    
    alu::alu_rra(ctx.cpu);
    op_fetch_next(ctx);
}

pub fn op_rr_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_rr(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* RRC, RRCA */

pub fn op_rrc_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_rrc(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_rrc_b(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::B); }
pub fn op_rrc_c(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::C); }
pub fn op_rrc_d(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::D); }
pub fn op_rrc_e(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::E); }
pub fn op_rrc_h(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::H); }
pub fn op_rrc_l(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::L); }
pub fn op_rrc_a(ctx: &mut CpuExec) { op_rrc_r8(ctx, Reg::A); }

pub fn op_rrca(ctx: &mut CpuExec) { 
    debug_assert!(ctx.cpu.step == 1);
    
    alu::alu_rrca(ctx.cpu);
    op_fetch_next(ctx);
}

pub fn op_rrc_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_rrc(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* SLA */

pub fn op_sla_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_sla(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_sla_b(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::B); }
pub fn op_sla_c(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::C); }
pub fn op_sla_d(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::D); }
pub fn op_sla_e(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::E); }
pub fn op_sla_h(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::H); }
pub fn op_sla_l(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::L); }
pub fn op_sla_a(ctx: &mut CpuExec) { op_sla_r8(ctx, Reg::A); }

pub fn op_sla_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_sla(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;

            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* SRA */

pub fn op_sra_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_sra(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_sra_b(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::B); }
pub fn op_sra_c(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::C); }
pub fn op_sra_d(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::D); }
pub fn op_sra_e(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::E); }
pub fn op_sra_h(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::H); }
pub fn op_sra_l(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::L); }
pub fn op_sra_a(ctx: &mut CpuExec) { op_sra_r8(ctx, Reg::A); }

pub fn op_sra_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_sra(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;

            op_write_hl(ctx);
        },
        3 => op_fetch_next(ctx),
        _ => unreachable!(),
    }
}

/* SRL */

pub fn op_srl_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_srl(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_srl_b(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::B); }
pub fn op_srl_c(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::C); }
pub fn op_srl_d(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::D); }
pub fn op_srl_e(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::E); }
pub fn op_srl_h(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::H); }
pub fn op_srl_l(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::L); }
pub fn op_srl_a(ctx: &mut CpuExec) { op_srl_r8(ctx, Reg::A); }

pub fn op_srl_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_srl(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* SWAP */

pub fn op_swap_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_swap(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_swap_b(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::B); }
pub fn op_swap_c(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::C); }
pub fn op_swap_d(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::D); }
pub fn op_swap_e(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::E); }
pub fn op_swap_h(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::H); }
pub fn op_swap_l(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::L); }
pub fn op_swap_a(ctx: &mut CpuExec) { op_swap_r8(ctx, Reg::A); }

pub fn op_swap_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_swap(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;
            
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

