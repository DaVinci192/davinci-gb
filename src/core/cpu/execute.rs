
use super::CPU;
use super::decoder::{CB_DISPATCH, DISPATCH};
use super::instructions::jumps_calls::{op_isr_40, op_isr_48, op_isr_50, op_isr_58, op_isr_60};

impl CPU {
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read8(self.pc);

        if self.halt_bug {
            // do not increment pc
            self.halt_bug = false;
        } else {
            self.pc = self.pc.wrapping_add(1);
        }

        byte
    }

    pub fn step_instruction(&mut self) {
        loop {
            self.cycle();
            if self.step == 1 {
                break;
            }
        }
    }

    pub fn cycle(&mut self) {
        if self.halted {
            if (self.ie & self.iflag) != 0 {
                self.halted = false;
            } else {
                return;
            }
        }

        if self.step == 0 {
            let opcode = self.ir;
            
            if self.handle_interrupts() {
                return;
            }
            
            if self.ime_scheduled {
                self.ime = true;
                self.ime_scheduled = false;
            }

            self.schedule_next_instruction(opcode);
        } else {
            (self.current_op)(self);

            self.m_cycles += 1;
            self.t_cycles += 1;
        }
    }

    pub fn schedule_next_instruction(&mut self, opcode: u8) {
        if opcode == 0xCB {
            let cb = self.fetch_byte();
            self.current_op = CB_DISPATCH[cb as usize];
        } else {
            self.current_op = DISPATCH[opcode as usize];
        }

        self.step = 1;
    }

    pub fn handle_interrupts(&mut self) -> bool {
        if !self.ime { return false; }

        let pending = self.ie & self.iflag;
        if pending == 0 { return false; }

        let bit = pending.trailing_zeros() as u8;
        self.current_op = match bit {
            0 => op_isr_40,
            1 => op_isr_48,
            2 => op_isr_50,
            3 => op_isr_58,
            4 => op_isr_60,
            _ => unreachable!(),
        };

        self.step = 1;

        true
    }

}