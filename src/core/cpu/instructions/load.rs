use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_hl, op_write_hl, op_read_n8, op_write_r16};

use super::super::registers::{Reg, Reg16};
use super::super::CpuExec;

/* LD_r8_r8 */

pub fn op_ld_r8_r8(ctx: &mut CpuExec, lhs: Reg, rhs: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(rhs);
    ctx.cpu.set_reg(lhs, val);

    op_fetch_next(ctx);
}

pub fn op_ld_r8_r8_noop(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    // noop

    op_fetch_next(ctx);
}

pub fn op_ld_a_b(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::A, Reg::B); }
pub fn op_ld_a_c(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::A, Reg::C); }
pub fn op_ld_a_d(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::A, Reg::D); }
pub fn op_ld_a_e(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::A, Reg::E); }
pub fn op_ld_a_h(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::A, Reg::H); }
pub fn op_ld_a_l(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::A, Reg::L); }
pub fn op_ld_a_a(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }

pub fn op_ld_b_b(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }
pub fn op_ld_b_c(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::B, Reg::C); }
pub fn op_ld_b_d(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::B, Reg::D); }
pub fn op_ld_b_e(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::B, Reg::E); }
pub fn op_ld_b_h(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::B, Reg::H); }
pub fn op_ld_b_l(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::B, Reg::L); }
pub fn op_ld_b_a(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::B, Reg::A); }

pub fn op_ld_c_b(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::C, Reg::B); }
pub fn op_ld_c_c(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }
pub fn op_ld_c_d(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::C, Reg::D); }
pub fn op_ld_c_e(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::C, Reg::E); }
pub fn op_ld_c_h(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::C, Reg::H); }
pub fn op_ld_c_l(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::C, Reg::L); }
pub fn op_ld_c_a(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::C, Reg::A); }

pub fn op_ld_d_b(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::D, Reg::B); }
pub fn op_ld_d_c(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::D, Reg::C); }
pub fn op_ld_d_d(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }
pub fn op_ld_d_e(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::D, Reg::E); }
pub fn op_ld_d_h(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::D, Reg::H); }
pub fn op_ld_d_l(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::D, Reg::L); }
pub fn op_ld_d_a(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::D, Reg::A); }

pub fn op_ld_e_b(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::E, Reg::B); }
pub fn op_ld_e_c(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::E, Reg::C); }
pub fn op_ld_e_d(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::E, Reg::D); }
pub fn op_ld_e_e(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }
pub fn op_ld_e_h(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::E, Reg::H); }
pub fn op_ld_e_l(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::E, Reg::L); }
pub fn op_ld_e_a(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::E, Reg::A); }

pub fn op_ld_h_b(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::H, Reg::B); }
pub fn op_ld_h_c(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::H, Reg::C); }
pub fn op_ld_h_d(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::H, Reg::D); }
pub fn op_ld_h_e(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::H, Reg::E); }
pub fn op_ld_h_h(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }
pub fn op_ld_h_l(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::H, Reg::L); }
pub fn op_ld_h_a(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::H, Reg::A); }

pub fn op_ld_l_b(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::L, Reg::B); }
pub fn op_ld_l_c(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::L, Reg::C); }
pub fn op_ld_l_d(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::L, Reg::D); }
pub fn op_ld_l_e(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::L, Reg::E); }
pub fn op_ld_l_h(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::L, Reg::H); }
pub fn op_ld_l_l(ctx: &mut CpuExec) { op_ld_r8_r8_noop(ctx); }
pub fn op_ld_l_a(ctx: &mut CpuExec) { op_ld_r8_r8(ctx, Reg::L, Reg::A); }

/* LD_r8_n8 */

pub fn op_ld_r8_n8(ctx: &mut CpuExec, reg: Reg) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            ctx.cpu.set_reg(reg, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_b_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::B); }
pub fn op_ld_c_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::C); }
pub fn op_ld_d_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::D); }
pub fn op_ld_e_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::E); }
pub fn op_ld_h_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::H); }
pub fn op_ld_l_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::L); }
pub fn op_ld_a_n8(ctx: &mut CpuExec) { op_ld_r8_n8(ctx, Reg::A); }

/* LD_r8_hl */

pub fn op_ld_r8_hl(ctx: &mut CpuExec, reg: Reg) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            ctx.cpu.set_reg(reg, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_b_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::B); }
pub fn op_ld_c_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::C); }
pub fn op_ld_d_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::D); }
pub fn op_ld_e_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::E); }
pub fn op_ld_h_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::H); }
pub fn op_ld_l_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::L); }
pub fn op_ld_a_hl(ctx: &mut CpuExec) { op_ld_r8_hl(ctx, Reg::A); }

/* LD_hl */

pub fn op_ld_hl_r8(ctx: &mut CpuExec, reg: Reg) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.cpu.get_reg(reg);
            
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_ld_hl_b(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::B); }
pub fn op_ld_hl_c(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::C); }
pub fn op_ld_hl_d(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::D); }
pub fn op_ld_hl_e(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::E); }
pub fn op_ld_hl_h(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::H); }
pub fn op_ld_hl_l(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::L); }
pub fn op_ld_hl_a(ctx: &mut CpuExec) { op_ld_hl_r8(ctx, Reg::A); }

// this cannot be 2 cycles because both read and write
// operate on the address/data busses
pub fn op_ld_hl_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => op_write_hl(ctx),
        3 => { op_fetch_next(ctx); }, 
        _ => unreachable!(),
    }
}

/* LD_r16_A */

pub fn op_ld_r16_a(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.cpu.a;

            op_write_r16(ctx, reg);
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_ld_bc_a(ctx: &mut CpuExec) { op_ld_r16_a(ctx, Reg16::BC); }
pub fn op_ld_de_a(ctx: &mut CpuExec) { op_ld_r16_a(ctx, Reg16::DE); }

/* LD_n16_A */

pub fn op_ld_n16_a(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;
        },
        2 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);
        },
        3 => {
            ctx.bus.write8(ctx.cpu.temp16, ctx.cpu.a);
            ctx.cpu.step += 1;
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* LDH_n_A */

pub fn op_ldh_n_a(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = 0xFF00 | (ctx.cpu.temp as u16)
        },
        2 => {
            ctx.bus.write8(ctx.cpu.temp16, ctx.cpu.a);
            ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* LDH_c_A */

pub fn op_ldh_c_a(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp16 = 0xFF00 | (ctx.cpu.c as u16);
            ctx.bus.write8(ctx.cpu.temp16, ctx.cpu.a);
            ctx.cpu.step += 1;
        },
        2 => { op_fetch_next(ctx)},
        _ => unreachable!(),
    }
}

/* LD_A_r16 */

pub fn op_ld_a_r16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            let address = ctx.cpu.get_reg16(reg);
            ctx.cpu.temp = ctx.bus.read8(address);
            ctx.cpu.step += 1;
        },
        2 => {
            ctx.cpu.a = ctx.cpu.temp;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_a_bc(ctx: &mut CpuExec) { op_ld_a_r16(ctx, Reg16::BC) }
pub fn op_ld_a_de(ctx: &mut CpuExec) { op_ld_a_r16(ctx, Reg16::DE) }

/* LD_A_n16 */

pub fn op_ld_a_n16(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;
        },
        2 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);
        },
        3 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.temp16);
            ctx.cpu.step += 1;
        },
        4 => { 
            ctx.cpu.a = ctx.cpu.temp;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* LDH_A_n */

pub fn op_ldh_a_n(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = 0xFF00 | (ctx.cpu.temp as u16);
        },
        2 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.temp16);
            ctx.cpu.step += 1;
        },
        3 => {
            ctx.cpu.a = ctx.cpu.temp;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* LDH_A_c */

pub fn op_ldh_a_c(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp16 = 0xFF00 | (ctx.cpu.c as u16);
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.temp16);
            ctx.cpu.step += 1;
        },
        2 => {
            ctx.cpu.a = ctx.cpu.temp;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* LD_hli_a */

pub fn op_ld_hli_a(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.cpu.a;
            op_write_hl(ctx);

            let hli = ctx.cpu.get_reg16(Reg16::HL).wrapping_add(1);
            ctx.cpu.set_reg16(Reg16::HL, hli);
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* LD_hld_a */

pub fn op_ld_hld_a(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.cpu.a;
            op_write_hl(ctx);

            let hld = ctx.cpu.get_reg16(Reg16::HL).wrapping_sub(1);
            ctx.cpu.set_reg16(Reg16::HL, hld);
        },
        2 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}


/* LD_a_hli */

pub fn op_ld_a_hli(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_hl(ctx);
            ctx.cpu.a = ctx.cpu.temp;

            let hli = ctx.cpu.get_reg16(Reg16::HL).wrapping_add(1);
            ctx.cpu.set_reg16(Reg16::HL, hli);
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* LD_a_hld */

pub fn op_ld_a_hld(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_hl(ctx);
            ctx.cpu.a = ctx.cpu.temp;

            let hld = ctx.cpu.get_reg16(Reg16::HL).wrapping_sub(1);
            ctx.cpu.set_reg16(Reg16::HL, hld);
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* LD_r16_n16 */

pub fn op_ld_r16_n16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;
        },
        2 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);
        },
        3 => {
            ctx.cpu.set_reg16(reg, ctx.cpu.temp16);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_bc_n16(ctx: &mut CpuExec) { op_ld_r16_n16(ctx, Reg16::BC); }
pub fn op_ld_de_n16(ctx: &mut CpuExec) { op_ld_r16_n16(ctx, Reg16::DE); }
pub fn op_ld_hl_n16(ctx: &mut CpuExec) { op_ld_r16_n16(ctx, Reg16::HL); }