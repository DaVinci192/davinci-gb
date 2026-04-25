use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_n8};

use super::super::registers::{Reg16};
use super::super::CpuExec;


/* JP HL */

pub fn op_jp_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.pc = ctx.cpu.get_reg16(Reg16::HL);
            
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* JP n16 */

pub fn op_jp_n16(ctx: &mut CpuExec) {
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
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.step += 1;
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* JP cc, n16 */

pub fn op_jp_nz_n16(ctx: &mut CpuExec) {
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
            if !ctx.cpu.f.z {
                ctx.cpu.pc = ctx.cpu.temp16;
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_jp_z_n16(ctx: &mut CpuExec) {
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
            if ctx.cpu.f.z {
                ctx.cpu.pc = ctx.cpu.temp16;
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_jp_nc_n16(ctx: &mut CpuExec) {
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
            if !ctx.cpu.f.c {
                ctx.cpu.pc = ctx.cpu.temp16;
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_jp_c_n16(ctx: &mut CpuExec) {
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
            if !ctx.cpu.f.c {
                ctx.cpu.pc = ctx.cpu.temp16;
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* JR e8 */

pub fn op_jr_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
        },
        2 => {
            let offset = ctx.cpu.temp as i8 as u16;
            ctx.cpu.temp16 = ctx.cpu.pc.wrapping_add(offset);

            ctx.cpu.step += 1;
        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* JR cc, e8 */

pub fn op_jr_nz_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
        },
        2 => {
            let offset = ctx.cpu.temp as i8 as u16;

            if !ctx.cpu.f.z {
                ctx.cpu.temp16 = ctx.cpu.pc.wrapping_add(offset);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }

        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_jr_z_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
        },
        2 => {
            let offset = ctx.cpu.temp as i8 as u16;

            if ctx.cpu.f.z {
                ctx.cpu.temp16 = ctx.cpu.pc.wrapping_add(offset);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }

        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_jr_nc_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
        },
        2 => {
            let offset = ctx.cpu.temp as i8 as u16;

            if !ctx.cpu.f.c {
                ctx.cpu.temp16 = ctx.cpu.pc.wrapping_add(offset);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }

        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_jr_c_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
        },
        2 => {
            let offset = ctx.cpu.temp as i8 as u16;

            if ctx.cpu.f.c {
                ctx.cpu.temp16 = ctx.cpu.pc.wrapping_add(offset);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }

        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* CALL n16 */

pub fn op_call_n16(ctx: &mut CpuExec) {
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
            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
            ctx.cpu.step += 1;
        },
        4 => { 
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.pc = ctx.cpu.temp16;

            ctx.cpu.step += 1;
        },
        6 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* CALL cc, n16 */

pub fn op_call_nz_n16(ctx: &mut CpuExec) {
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
            if !ctx.cpu.f.z {
                ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { 
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.pc = ctx.cpu.temp16;

            ctx.cpu.step += 1;
        },
        6 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_call_z_n16(ctx: &mut CpuExec) {
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
            if ctx.cpu.f.z {
                ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { 
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.pc = ctx.cpu.temp16;

            ctx.cpu.step += 1;
        },
        6 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_call_nc_n16(ctx: &mut CpuExec) {
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
            if !ctx.cpu.f.c {
                ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { 
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.pc = ctx.cpu.temp16;

            ctx.cpu.step += 1;
        },
        6 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_call_c_n16(ctx: &mut CpuExec) {
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
            if ctx.cpu.f.c {
                ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        4 => { 
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        5 => {
            let lsb = (0x00FF | ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.pc = ctx.cpu.temp16;

            ctx.cpu.step += 1;
        },
        6 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* RET */

pub fn op_ret(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        2 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.step += 1;
        },
        4 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* RET cc */

pub fn op_ret_nz(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            ctx.cpu.step += 1;
        },
        2 => {
            if !ctx.cpu.f.z {
                ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
                ctx.cpu.temp16 = ctx.cpu.temp as u16;

                ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        3 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        4 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.step += 1;
        },
        5 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ret_z(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            ctx.cpu.step += 1;
        },
        2 => {
            if ctx.cpu.f.z {
                ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
                ctx.cpu.temp16 = ctx.cpu.temp as u16;

                ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        3 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        4 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.step += 1;
        },
        5 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ret_nc(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            ctx.cpu.step += 1;
        },
        2 => {
            if !ctx.cpu.f.c {
                ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
                ctx.cpu.temp16 = ctx.cpu.temp as u16;

                ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        3 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        4 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.step += 1;
        },
        5 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_ret_c(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            // cc check
            // this is done in the next m-cycle for easier
            // control flow implementation
            ctx.cpu.step += 1;
        },
        2 => {
            if ctx.cpu.f.c {
                ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
                ctx.cpu.temp16 = ctx.cpu.temp as u16;

                ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

                ctx.cpu.step += 1;
            } else {
                op_fetch_next(ctx);
            }
        },
        3 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        4 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.step += 1;
        },
        5 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* RETI */

pub fn op_reti(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        2 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        3 => {
            ctx.cpu.pc = ctx.cpu.temp16;
            ctx.cpu.ime = true;
            ctx.cpu.step += 1;
        },
        4 => {
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}


/* RST vec */

pub fn op_rst_vec(ctx: &mut CpuExec, vec: u16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
            ctx.cpu.step += 1;
        },
        2 => {
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        3 => {
            let lsb = (0x00FF & ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.pc = vec;

            ctx.cpu.step += 1;
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_rst_00(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x00) }
pub fn op_rst_08(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x08) }
pub fn op_rst_10(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x10) }
pub fn op_rst_18(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x18) }
pub fn op_rst_20(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x20) }
pub fn op_rst_28(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x28) }
pub fn op_rst_30(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x30) }
pub fn op_rst_38(ctx: &mut CpuExec) { op_rst_vec(ctx, 0x38) }

/* Interrupt service */

// M-cycles are not exactly correct here, but still fits within 5
pub fn op_isr_vec(ctx: &mut CpuExec, vec: u16, bit: u8) {
    match ctx.cpu.step {
        1 => {
            // unwind overlapped fetch
            ctx.cpu.pc = ctx.cpu.pc.wrapping_sub(1);

            ctx.cpu.ime = false;
            ctx.cpu.iflag = ctx.cpu.iflag & !(bit);

            ctx.cpu.step += 1;
        },
        2 => {
            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
            ctx.cpu.step += 1;
        },
        3 => { 
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        4 => {
            let lsb = (0x00FF | ctx.cpu.pc) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.step += 1;
        },
        5 => {
            ctx.cpu.pc = vec;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_isr_40(ctx: &mut CpuExec) { op_isr_vec(ctx, 0x40, 0); }
pub fn op_isr_48(ctx: &mut CpuExec) { op_isr_vec(ctx, 0x48, 1); }
pub fn op_isr_50(ctx: &mut CpuExec) { op_isr_vec(ctx, 0x50, 2); }
pub fn op_isr_58(ctx: &mut CpuExec) { op_isr_vec(ctx, 0x58, 3); }
pub fn op_isr_60(ctx: &mut CpuExec) { op_isr_vec(ctx, 0x60, 4); }