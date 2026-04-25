use super::super::CPU;

/* CB PREFIX */
// this opcode is not used as CB is handled directly

pub fn op_cb_prefix(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    //let cb_opcode = cpu.bus.read8(cpu.pc);
    cpu.pc += 1;

    cpu.step = 1;
}

/* DI */

pub fn op_di(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    cpu.ime = false;

    op_fetch_next(cpu);
}

/* EI */

pub fn op_ei(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    cpu.ime_scheduled = true;

    op_fetch_next(cpu);
}

/* NOP */

pub fn op_nop(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);
    op_fetch_next(cpu);
}

/* STOP */

pub fn op_stop(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    // usually followed by a 0x00 byte

    op_fetch_next(cpu);
}

/* HALT */

pub fn op_halt(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    let pending = cpu.ie & cpu.iflag;

    if cpu.ime {
        cpu.halted = true;
    } else {
        if pending == 0 {
            cpu.halted = true;
        } else {
            cpu.halt_bug = true;
        }
    }

    // special case - do not increment pc
    cpu.ir = cpu.bus.read8(cpu.pc);
    cpu.step = 0;
}