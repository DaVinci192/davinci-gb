use super::{op_read_hl, op_write_hl, op_read_n8, op_write_r16};

use super::super::registers::{Reg, Reg16};
use super::super::CPU;

/* LD_r8_r8 */

pub fn op_ld_r8_r8(cpu: &mut CPU, lhs: Reg, rhs: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(rhs);
    cpu.set_reg(lhs, val);

    op_fetch_next(cpu);
}

pub fn op_ld_r8_r8_noop(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    // noop

    op_fetch_next(cpu);
}

pub fn op_ld_a_b(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::A, Reg::B); }
pub fn op_ld_a_c(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::A, Reg::C); }
pub fn op_ld_a_d(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::A, Reg::D); }
pub fn op_ld_a_e(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::A, Reg::E); }
pub fn op_ld_a_h(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::A, Reg::H); }
pub fn op_ld_a_l(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::A, Reg::L); }
pub fn op_ld_a_a(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }

pub fn op_ld_b_b(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }
pub fn op_ld_b_c(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::B, Reg::C); }
pub fn op_ld_b_d(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::B, Reg::D); }
pub fn op_ld_b_e(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::B, Reg::E); }
pub fn op_ld_b_h(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::B, Reg::H); }
pub fn op_ld_b_l(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::B, Reg::L); }
pub fn op_ld_b_a(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::B, Reg::A); }

pub fn op_ld_c_b(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::C, Reg::B); }
pub fn op_ld_c_c(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }
pub fn op_ld_c_d(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::C, Reg::D); }
pub fn op_ld_c_e(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::C, Reg::E); }
pub fn op_ld_c_h(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::C, Reg::H); }
pub fn op_ld_c_l(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::C, Reg::L); }
pub fn op_ld_c_a(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::C, Reg::A); }

pub fn op_ld_d_b(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::D, Reg::B); }
pub fn op_ld_d_c(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::D, Reg::C); }
pub fn op_ld_d_d(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }
pub fn op_ld_d_e(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::D, Reg::E); }
pub fn op_ld_d_h(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::D, Reg::H); }
pub fn op_ld_d_l(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::D, Reg::L); }
pub fn op_ld_d_a(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::D, Reg::A); }

pub fn op_ld_e_b(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::E, Reg::B); }
pub fn op_ld_e_c(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::E, Reg::C); }
pub fn op_ld_e_d(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::E, Reg::D); }
pub fn op_ld_e_e(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }
pub fn op_ld_e_h(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::E, Reg::H); }
pub fn op_ld_e_l(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::E, Reg::L); }
pub fn op_ld_e_a(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::E, Reg::A); }

pub fn op_ld_h_b(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::H, Reg::B); }
pub fn op_ld_h_c(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::H, Reg::C); }
pub fn op_ld_h_d(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::H, Reg::D); }
pub fn op_ld_h_e(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::H, Reg::E); }
pub fn op_ld_h_h(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }
pub fn op_ld_h_l(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::H, Reg::L); }
pub fn op_ld_h_a(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::H, Reg::A); }

pub fn op_ld_l_b(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::L, Reg::B); }
pub fn op_ld_l_c(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::L, Reg::C); }
pub fn op_ld_l_d(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::L, Reg::D); }
pub fn op_ld_l_e(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::L, Reg::E); }
pub fn op_ld_l_h(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::L, Reg::H); }
pub fn op_ld_l_l(cpu: &mut CPU) { op_ld_r8_r8_noop(cpu); }
pub fn op_ld_l_a(cpu: &mut CPU) { op_ld_r8_r8(cpu, Reg::L, Reg::A); }

/* LD_r8_n8 */

pub fn op_ld_r8_n8(cpu: &mut CPU, reg: Reg) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            cpu.set_reg(reg, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_b_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::B); }
pub fn op_ld_c_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::C); }
pub fn op_ld_d_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::D); }
pub fn op_ld_e_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::E); }
pub fn op_ld_h_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::H); }
pub fn op_ld_l_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::L); }
pub fn op_ld_a_n8(cpu: &mut CPU) { op_ld_r8_n8(cpu, Reg::A); }

/* LD_r8_hl */

pub fn op_ld_r8_hl(cpu: &mut CPU, reg: Reg) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            cpu.set_reg(reg, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_b_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::B); }
pub fn op_ld_c_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::C); }
pub fn op_ld_d_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::D); }
pub fn op_ld_e_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::E); }
pub fn op_ld_h_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::H); }
pub fn op_ld_l_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::L); }
pub fn op_ld_a_hl(cpu: &mut CPU) { op_ld_r8_hl(cpu, Reg::A); }

/* LD_hl */

pub fn op_ld_hl_r8(cpu: &mut CPU, reg: Reg) {
    match cpu.step {
        1 => {
            cpu.temp = cpu.get_reg(reg);
            
            op_write_hl(cpu); // cpu.step += 1;
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_ld_hl_b(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::B); }
pub fn op_ld_hl_c(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::C); }
pub fn op_ld_hl_d(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::D); }
pub fn op_ld_hl_e(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::E); }
pub fn op_ld_hl_h(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::H); }
pub fn op_ld_hl_l(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::L); }
pub fn op_ld_hl_a(cpu: &mut CPU) { op_ld_hl_r8(cpu, Reg::A); }

// this cannot be 2 cycles because both read and write
// operate on the address/data busses
pub fn op_ld_hl_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => op_write_hl(cpu),
        3 => { op_fetch_next(cpu); }, 
        _ => unreachable!(),
    }
}

/* LD_r16_A */

pub fn op_ld_r16_a(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            cpu.temp = cpu.a;

            op_write_r16(cpu, reg);
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_ld_bc_a(cpu: &mut CPU) { op_ld_r16_a(cpu, Reg16::BC); }
pub fn op_ld_de_a(cpu: &mut CPU) { op_ld_r16_a(cpu, Reg16::DE); }

/* LD_n16_A */

pub fn op_ld_n16_a(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
            cpu.temp16 = cpu.temp as u16;
        },
        2 => {
            op_read_n8(cpu);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);
        },
        3 => {
            cpu.bus.write8(cpu.temp16, cpu.a);
            cpu.step += 1;
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* LDH_n_A */

pub fn op_ldh_n_a(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
            cpu.temp16 = 0xFF00 | (cpu.temp as u16)
        },
        2 => {
            cpu.bus.write8(cpu.temp16, cpu.a);
            cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* LDH_c_A */

pub fn op_ldh_c_a(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.temp16 = 0xFF00 | (cpu.c as u16);
            cpu.bus.write8(cpu.temp16, cpu.a);
            cpu.step += 1;
        },
        2 => { op_fetch_next(cpu)},
        _ => unreachable!(),
    }
}

/* LD_A_r16 */

pub fn op_ld_a_r16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            let address = cpu.get_reg16(reg);
            cpu.temp = cpu.bus.read8(address);
            cpu.step += 1;
        },
        2 => {
            cpu.a = cpu.temp;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_a_bc(cpu: &mut CPU) { op_ld_a_r16(cpu, Reg16::BC) }
pub fn op_ld_a_de(cpu: &mut CPU) { op_ld_a_r16(cpu, Reg16::DE) }

/* LD_A_n16 */

pub fn op_ld_a_n16(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
            cpu.temp16 = cpu.temp as u16;
        },
        2 => {
            op_read_n8(cpu);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);
        },
        3 => {
            cpu.temp = cpu.bus.read8(cpu.temp16);
            cpu.step += 1;
        },
        4 => { 
            cpu.a = cpu.temp;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* LDH_A_n */

pub fn op_ldh_a_n(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
            cpu.temp16 = 0xFF00 | (cpu.temp as u16);
        },
        2 => {
            cpu.temp = cpu.bus.read8(cpu.temp16);
            cpu.step += 1;
        },
        3 => {
            cpu.a = cpu.temp;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* LDH_A_c */

pub fn op_ldh_a_c(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.temp16 = 0xFF00 | (cpu.c as u16);
            cpu.temp = cpu.bus.read8(cpu.temp16);
            cpu.step += 1;
        },
        2 => {
            cpu.a = cpu.temp;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* LD_hli_a */

pub fn op_ld_hli_a(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.temp = cpu.a;
            op_write_hl(cpu);

            let hli = cpu.get_reg16(Reg16::HL).wrapping_add(1);
            cpu.set_reg16(Reg16::HL, hli);
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* LD_hld_a */

pub fn op_ld_hld_a(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.temp = cpu.a;
            op_write_hl(cpu);

            let hld = cpu.get_reg16(Reg16::HL).wrapping_sub(1);
            cpu.set_reg16(Reg16::HL, hld);
        },
        2 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}


/* LD_a_hli */

pub fn op_ld_a_hli(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_hl(cpu);
            cpu.a = cpu.temp;

            let hli = cpu.get_reg16(Reg16::HL).wrapping_add(1);
            cpu.set_reg16(Reg16::HL, hli);
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* LD_a_hld */

pub fn op_ld_a_hld(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_hl(cpu);
            cpu.a = cpu.temp;

            let hld = cpu.get_reg16(Reg16::HL).wrapping_sub(1);
            cpu.set_reg16(Reg16::HL, hld);
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* LD_r16_n16 */

pub fn op_ld_r16_n16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
            cpu.temp16 = cpu.temp as u16;
        },
        2 => {
            op_read_n8(cpu);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);
        },
        3 => {
            cpu.set_reg16(reg, cpu.temp16);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ld_bc_n16(cpu: &mut CPU) { op_ld_r16_n16(cpu, Reg16::BC); }
pub fn op_ld_de_n16(cpu: &mut CPU) { op_ld_r16_n16(cpu, Reg16::DE); }
pub fn op_ld_hl_n16(cpu: &mut CPU) { op_ld_r16_n16(cpu, Reg16::HL); }