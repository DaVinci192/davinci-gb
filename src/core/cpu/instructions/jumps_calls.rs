use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_n8};

use super::super::registers::{Reg16};
use super::super::CPU;


/* JP HL */

pub fn op_jp_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            cpu.pc = cpu.get_reg16(Reg16::HL);
            
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* JP n16 */

pub fn op_jp_n16(cpu: &mut CPU) {
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
            cpu.pc = cpu.temp16;
            cpu.step += 1;
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* JP cc, n16 */

pub fn op_jp_nz_n16(cpu: &mut CPU) {
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
            if !cpu.f.z {
                cpu.pc = cpu.temp16;
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_jp_z_n16(cpu: &mut CPU) {
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
            if cpu.f.z {
                cpu.pc = cpu.temp16;
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_jp_nc_n16(cpu: &mut CPU) {
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
            if !cpu.f.c {
                cpu.pc = cpu.temp16;
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_jp_c_n16(cpu: &mut CPU) {
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
            if !cpu.f.c {
                cpu.pc = cpu.temp16;
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* JR e8 */

pub fn op_jr_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
        },
        2 => {
            let offset = cpu.temp as i8 as u16;
            cpu.temp16 = cpu.pc.wrapping_add(offset);

            cpu.step += 1;
        },
        3 => {
            cpu.pc = cpu.temp16;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* JR cc, e8 */

pub fn op_jr_nz_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
        },
        2 => {
            let offset = cpu.temp as i8 as u16;

            if !cpu.f.z {
                cpu.temp16 = cpu.pc.wrapping_add(offset);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }

        },
        3 => {
            cpu.pc = cpu.temp16;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_jr_z_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
        },
        2 => {
            let offset = cpu.temp as i8 as u16;

            if cpu.f.z {
                cpu.temp16 = cpu.pc.wrapping_add(offset);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }

        },
        3 => {
            cpu.pc = cpu.temp16;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_jr_nc_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
        },
        2 => {
            let offset = cpu.temp as i8 as u16;

            if !cpu.f.c {
                cpu.temp16 = cpu.pc.wrapping_add(offset);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }

        },
        3 => {
            cpu.pc = cpu.temp16;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_jr_c_e8(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            op_read_n8(cpu);
        },
        2 => {
            let offset = cpu.temp as i8 as u16;

            if cpu.f.c {
                cpu.temp16 = cpu.pc.wrapping_add(offset);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }

        },
        3 => {
            cpu.pc = cpu.temp16;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* CALL n16 */

pub fn op_call_n16(cpu: &mut CPU) {
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
            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.step += 1;
        },
        4 => { 
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.pc = cpu.temp16;

            cpu.step += 1;
        },
        6 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* CALL cc, n16 */

pub fn op_call_nz_n16(cpu: &mut CPU) {
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
            if !cpu.f.z {
                cpu.sp = cpu.sp.wrapping_sub(1);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { 
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.pc = cpu.temp16;

            cpu.step += 1;
        },
        6 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_call_z_n16(cpu: &mut CPU) {
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
            if cpu.f.z {
                cpu.sp = cpu.sp.wrapping_sub(1);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { 
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.pc = cpu.temp16;

            cpu.step += 1;
        },
        6 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_call_nc_n16(cpu: &mut CPU) {
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
            if !cpu.f.c {
                cpu.sp = cpu.sp.wrapping_sub(1);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { 
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.pc = cpu.temp16;

            cpu.step += 1;
        },
        6 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_call_c_n16(cpu: &mut CPU) {
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
            if cpu.f.c {
                cpu.sp = cpu.sp.wrapping_sub(1);
                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        4 => { 
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.pc = cpu.temp16;

            cpu.step += 1;
        },
        6 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* RET */

pub fn op_ret(cpu: &mut CPU) {
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
            cpu.pc = cpu.temp16;
            cpu.step += 1;
        },
        4 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* RET cc */

pub fn op_ret_nz(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            cpu.step += 1;
        },
        2 => {
            if !cpu.f.z {
                cpu.temp = cpu.bus.read8(cpu.sp);
                cpu.temp16 = cpu.temp as u16;

                cpu.sp = cpu.sp.wrapping_add(1);

                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        3 => {
            cpu.temp = cpu.bus.read8(cpu.sp);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);

            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.step += 1;
        },
        4 => {
            cpu.pc = cpu.temp16;
            cpu.step += 1;
        },
        5 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ret_z(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            cpu.step += 1;
        },
        2 => {
            if cpu.f.z {
                cpu.temp = cpu.bus.read8(cpu.sp);
                cpu.temp16 = cpu.temp as u16;

                cpu.sp = cpu.sp.wrapping_add(1);

                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        3 => {
            cpu.temp = cpu.bus.read8(cpu.sp);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);

            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.step += 1;
        },
        4 => {
            cpu.pc = cpu.temp16;
            cpu.step += 1;
        },
        5 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ret_nc(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            cpu.step += 1;
        },
        2 => {
            if !cpu.f.c {
                cpu.temp = cpu.bus.read8(cpu.sp);
                cpu.temp16 = cpu.temp as u16;

                cpu.sp = cpu.sp.wrapping_add(1);

                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        3 => {
            cpu.temp = cpu.bus.read8(cpu.sp);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);

            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.step += 1;
        },
        4 => {
            cpu.pc = cpu.temp16;
            cpu.step += 1;
        },
        5 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_ret_c(cpu: &mut CPU) {
    match cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            cpu.step += 1;
        },
        2 => {
            if cpu.f.c {
                cpu.temp = cpu.bus.read8(cpu.sp);
                cpu.temp16 = cpu.temp as u16;

                cpu.sp = cpu.sp.wrapping_add(1);

                cpu.step += 1;
            } else {
                op_fetch_next(cpu);
            }
        },
        3 => {
            cpu.temp = cpu.bus.read8(cpu.sp);
            cpu.temp16 = cpu.temp16 | ((cpu.temp as u16) << 8);

            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.step += 1;
        },
        4 => {
            cpu.pc = cpu.temp16;
            cpu.step += 1;
        },
        5 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* RETI */

pub fn op_reti(cpu: &mut CPU) {
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
            cpu.pc = cpu.temp16;
            cpu.ime = true;
            cpu.step += 1;
        },
        4 => {
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}


/* RST vec */

pub fn op_rst_vec(cpu: &mut CPU, vec: u16) {
    match cpu.step {
        1 => {
            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.step += 1;
        },
        2 => {
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        3 => {
            let lsb = (0x00FF & cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.pc = vec;

            cpu.step += 1;
        },
        4 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_rst_00(cpu: &mut CPU) { op_rst_vec(cpu, 0x00) }
pub fn op_rst_08(cpu: &mut CPU) { op_rst_vec(cpu, 0x08) }
pub fn op_rst_10(cpu: &mut CPU) { op_rst_vec(cpu, 0x10) }
pub fn op_rst_18(cpu: &mut CPU) { op_rst_vec(cpu, 0x18) }
pub fn op_rst_20(cpu: &mut CPU) { op_rst_vec(cpu, 0x20) }
pub fn op_rst_28(cpu: &mut CPU) { op_rst_vec(cpu, 0x28) }
pub fn op_rst_30(cpu: &mut CPU) { op_rst_vec(cpu, 0x30) }
pub fn op_rst_38(cpu: &mut CPU) { op_rst_vec(cpu, 0x38) }

/* Interrupt service */

// M-cycles are not exactly correct here, but still fits within 5
pub fn op_isr_vec(cpu: &mut CPU, vec: u16, bit: u8) {
    match cpu.step {
        1 => {
            // unwind overlapped fetch
            cpu.pc = cpu.pc.wrapping_sub(1);

            cpu.ime = false;
            cpu.iflag = cpu.iflag & !(bit);

            cpu.step += 1;
        },
        2 => {
            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.step += 1;
        },
        3 => { 
            let msb = (cpu.pc >> 8) as u8;
            cpu.bus.write8(cpu.sp, msb);

            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.step += 1;
        },
        4 => {
            let lsb = (0x00FF | cpu.pc) as u8;
            cpu.bus.write8(cpu.sp, lsb);

            cpu.step += 1;
        },
        5 => {
            cpu.pc = vec;
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_isr_40(cpu: &mut CPU) { op_isr_vec(cpu, 0x40, 0); }
pub fn op_isr_48(cpu: &mut CPU) { op_isr_vec(cpu, 0x48, 1); }
pub fn op_isr_50(cpu: &mut CPU) { op_isr_vec(cpu, 0x50, 2); }
pub fn op_isr_58(cpu: &mut CPU) { op_isr_vec(cpu, 0x58, 3); }
pub fn op_isr_60(cpu: &mut CPU) { op_isr_vec(cpu, 0x60, 4); }