use crate::core::cpu::instructions::op_fetch_next;

use super::super::CpuExec;

/* CB PREFIX */
// this opcode is not used as CB is handled directly

pub fn op_cb_prefix(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    //let cb_opcode = ctx.bus.read8(ctx.cpu.pc);
    ctx.cpu.pc += 1;

    ctx.cpu.step = 1;
}

/* DI */

pub fn op_di(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    ctx.cpu.ime = false;

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

    op_fetch_next(ctx);
}

/* HALT */

pub fn op_halt(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    let pending = ctx.cpu.ie & ctx.cpu.iflag;

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