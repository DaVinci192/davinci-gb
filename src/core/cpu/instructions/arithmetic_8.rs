use super::{op_read_hl, op_read_n8, op_write_hl};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CPU;

/* ADC */

pub fn op_adc_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_adc(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_adc_a_b(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::B); }
pub fn op_adc_a_c(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::C); }
pub fn op_adc_a_d(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::D); }
pub fn op_adc_a_e(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::E); }
pub fn op_adc_a_h(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::H); }
pub fn op_adc_a_l(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::L); }
pub fn op_adc_a_a(cpu: &mut CPU) { op_adc_a_r8(cpu, Reg::A); }

pub fn op_adc_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_adc(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_adc_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_adc(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* ADD */

pub fn op_add_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_add(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_add_a_b(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::B); }
pub fn op_add_a_c(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::C); }
pub fn op_add_a_d(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::D); }
pub fn op_add_a_e(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::E); }
pub fn op_add_a_h(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::H); }
pub fn op_add_a_l(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::L); }
pub fn op_add_a_a(cpu: &mut CPU) { op_add_a_r8(cpu, Reg::A); }

pub fn op_add_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_add(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_add_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_add(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* CP */

pub fn op_cp_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_cp(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_cp_a_b(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::B); }
pub fn op_cp_a_c(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::C); }
pub fn op_cp_a_d(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::D); }
pub fn op_cp_a_e(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::E); }
pub fn op_cp_a_h(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::H); }
pub fn op_cp_a_l(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::L); }
pub fn op_cp_a_a(cpu: &mut CPU) { op_cp_a_r8(cpu, Reg::A); }

pub fn op_cp_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_cp(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_cp_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_cp(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* DEC */

pub fn op_dec_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_dec(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_dec_b(cpu: &mut CPU) { op_dec_r8(cpu, Reg::B); }
pub fn op_dec_c(cpu: &mut CPU) { op_dec_r8(cpu, Reg::C); }
pub fn op_dec_d(cpu: &mut CPU) { op_dec_r8(cpu, Reg::D); }
pub fn op_dec_e(cpu: &mut CPU) { op_dec_r8(cpu, Reg::E); }
pub fn op_dec_h(cpu: &mut CPU) { op_dec_r8(cpu, Reg::H); }
pub fn op_dec_l(cpu: &mut CPU) { op_dec_r8(cpu, Reg::L); }
pub fn op_dec_a(cpu: &mut CPU) { op_dec_r8(cpu, Reg::A); }

pub fn op_dec_hl_ptr(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_dec(cpu, cpu.temp);
            cpu.temp = res;

            op_write_hl(cpu); // cpu.step += 1 within
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* INC */

pub fn op_inc_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_inc(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_inc_b(cpu: &mut CPU) { op_inc_r8(cpu, Reg::B); }
pub fn op_inc_c(cpu: &mut CPU) { op_inc_r8(cpu, Reg::C); }
pub fn op_inc_d(cpu: &mut CPU) { op_inc_r8(cpu, Reg::D); }
pub fn op_inc_e(cpu: &mut CPU) { op_inc_r8(cpu, Reg::E); }
pub fn op_inc_h(cpu: &mut CPU) { op_inc_r8(cpu, Reg::H); }
pub fn op_inc_l(cpu: &mut CPU) { op_inc_r8(cpu, Reg::L); }
pub fn op_inc_a(cpu: &mut CPU) { op_inc_r8(cpu, Reg::A); }

pub fn op_inc_hl_ptr(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_inc(cpu, cpu.temp);
            cpu.temp = res;

            op_write_hl(cpu); // cpu.step += 1
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* SBC */

pub fn op_sbc_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_sbc(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_sbc_a_b(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::B); }
pub fn op_sbc_a_c(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::C); }
pub fn op_sbc_a_d(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::D); }
pub fn op_sbc_a_e(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::E); }
pub fn op_sbc_a_h(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::H); }
pub fn op_sbc_a_l(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::L); }
pub fn op_sbc_a_a(cpu: &mut CPU) { op_sbc_a_r8(cpu, Reg::A); }

pub fn op_sbc_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_sbc(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_sbc_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_sbc(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* SUB */

pub fn op_sub_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_sub(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_sub_a_b(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::B); }
pub fn op_sub_a_c(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::C); }
pub fn op_sub_a_d(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::D); }
pub fn op_sub_a_e(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::E); }
pub fn op_sub_a_h(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::H); }
pub fn op_sub_a_l(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::L); }
pub fn op_sub_a_a(cpu: &mut CPU) { op_sub_a_r8(cpu, Reg::A); }

pub fn op_sub_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_sub(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_sub_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_sub(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* DAA */

pub fn op_daa(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    if cpu.f.n {
        let mut adj = 0u8;

        if cpu.f.h {
            adj = adj.wrapping_add(0x6);
        }

        if cpu.f.c {
            adj = adj.wrapping_add(0x60);
        }

        cpu.a = cpu.a.wrapping_sub(adj);
    } else {
        let mut adj = 0u8;

        if cpu.f.h || ((cpu.a & 0xF) > 9) {
            adj = adj.wrapping_add(6);
        }

        if cpu.f.c || (cpu.a > 0x99) {
            adj = adj.wrapping_add(0x60);
            cpu.f.c = true;
        }

        cpu.a = cpu.a.wrapping_add(adj);
    }

    cpu.f.h = false;

    op_fetch_next(cpu);
}