use crate::core::cpu::instructions::op_fetch_next;
use crate::core::cpu::instructions::op_read_n8;

use super::super::CpuExec;

/* CB PREFIX */

pub fn op_cb_prefix(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    ctx.cpu.cb_prefix = true;
    
    op_fetch_next(ctx);
}

/* DI */

pub fn op_di(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    ctx.cpu.ime = false;
    ctx.cpu.ime_scheduled = false;

    op_fetch_next(ctx);
}

/* EI */

pub fn op_ei(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    ctx.cpu.ime_scheduled = true;

    op_fetch_next(ctx);
}

/* NOP */

pub fn op_nop(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);
    op_fetch_next(ctx);
}

/* STOP */

pub fn op_stop(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    // usually followed by a 0x00 byte
    // read/discard next byte for now
    op_read_n8(ctx);

    op_fetch_next(ctx);
}

/* HALT */

pub fn op_halt(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    let pending = ctx.bus.ie() & ctx.bus.iflag();

    if ctx.cpu.ime {
        ctx.cpu.halted = true;
    } else {
        if pending == 0 {
            ctx.cpu.halted = true;
        } else {
            ctx.cpu.halt_bug = true;
        }
    }

    // special case - do not increment pc
    ctx.cpu.ir = ctx.bus.read8(ctx.cpu.pc);
    ctx.cpu.step = 0;
}