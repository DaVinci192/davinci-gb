use super::super::alu;

use super::{op_read_n8};

use super::super::registers::{Reg16};
use super::super::CPU;

/* ADD_hl_sp */

pub fn op_add_hl_sp(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            alu::alu_add16(cpu, cpu.sp);
            cpu.step += 1;
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* ADD_sp_e8 */

pub fn op_add_sp_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => { op_read_n8(cpu); },
        2 => { 
            alu::alu_add_sp_e8(cpu, cpu.temp);
            cpu.step += 1;
        },
        3 => {
            // extra cycle since previous spans 2 cycles
            // this would be MSB operation
            cpu.step += 1;
        },
        4 => {
            // write result sp <- temp16
            // already done so not necessary
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* DEC_sp */

pub fn op_dec_sp(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.sp = alu::alu_dec16(cpu.sp);
            cpu.step += 1;
        },
        2 => {
            // extra cycle since IDU also used for fetch
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* INC_sp */

pub fn op_inc_sp(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.sp = alu::alu_inc16(cpu.sp);
            cpu.step += 1;
        },
        2 => {
            // extra cycle since IDU also used for fetch
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}


/* LD_sp_n16 */

pub fn op_ld_sp_n16(cpu: &mut CPU) {
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
            cpu.sp = cpu.temp16;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}


/* LD_n16_sp */

pub fn op_ld_n16_sp(cpu: &mut CPU) {
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
            let lsb = (0x00FF & cpu.sp) as u8;
            cpu.bus.write8(cpu.temp16, lsb);
            
            cpu.temp16 = cpu.temp16.wrapping_add(1);

            cpu.step += 1;
        },
        4 => {
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.temp16, msb);

            cpu.step += 1;
        },
        5 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}


/* LD_hl_sp_e8 */

pub fn op_ld_hl_sp_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => { op_read_n8(cpu); },
        2 => { 
            alu::alu_add_sp_e8(cpu, cpu.temp);
            cpu.step += 1;
        },
        3 => {
            // technically msb part of sp_e8 would be performed here
            cpu.set_reg16(Reg16::HL, cpu.sp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}


/* LD SP, HL */

pub fn op_ld_sp_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.sp = cpu.get_reg16(Reg16::HL);
            cpu.step += 1;
        },
        2 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}


/* POP_r16 */

pub fn op_pop_r16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            cpu.temp = cpu.bus.read8(cpu.sp);
            cpu.temp16 = cpu.temp as u16;

            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.step += 1;
        },
        2 => {
            cpu.temp = cpu.bus.read8(cpu.sp);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);

            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.step += 1;
        },
        3 => {
            cpu.set_reg16(reg, cpu.temp16);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_pop_bc(cpu: &mut CPU) { op_pop_r16(cpu, Reg16::BC) }
pub fn op_pop_de(cpu: &mut CPU) { op_pop_r16(cpu, Reg16::DE) }
pub fn op_pop_hl(cpu: &mut CPU) { op_pop_r16(cpu, Reg16::HL) }
pub fn op_pop_af(cpu: &mut CPU) { op_pop_r16(cpu, Reg16::AF) }

/* PUSH */

pub fn op_push_r16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.step += 1;
        },
        2 => {
            let msb = (cpu.get_reg16(reg) >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        3 => {
            let lsb = (0x00FF & cpu.get_reg16(reg)) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.step += 1;
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_push_bc(cpu: &mut CPU) { op_push_r16(cpu, Reg16::BC) }
pub fn op_push_de(cpu: &mut CPU) { op_push_r16(cpu, Reg16::DE) }
pub fn op_push_hl(cpu: &mut CPU) { op_push_r16(cpu, Reg16::HL) }
pub fn op_push_af(cpu: &mut CPU) { op_push_r16(cpu, Reg16::AF) }

