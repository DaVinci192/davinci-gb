use crate::core::bus::{Bus, Interrupt};

const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

const SYS_MASK: [u16; 4] = [(0x0001 << 9), (0x0001 << 3), (0x0001 << 5), (0x0001 << 7)];

pub struct Timer {
    sys_counter: u16,

    tac: u8,
    tma: u8,
    tima: u8,

    tima_rst_delay: u8,
    handle_overflow: bool,
}

// getters
impl Timer {
    pub fn sys_counter(&self) -> u16 { self.sys_counter }
    pub fn div(&self) -> u8 { (self.sys_counter >> 8) as u8 }
    pub fn tac(&self) -> u8 { self.tac }
    pub fn tma(&self) -> u8 { self.tma }
    pub fn tima(&self) -> u8 { self.tima }
}


impl Timer {
    pub fn new() -> Self {
        Self {
            sys_counter: 0xAB00,

            tac: 0xF8,
            tma: 0,
            tima: 0,

            tima_rst_delay: 0,
            handle_overflow: false,
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            DIV => {
                let sys_counter_old = self.sys_counter;
                self.sys_counter = 0;

                self.update_tima(sys_counter_old);

            },
            TIMA => {
                self.tima = val;
                self.handle_overflow = false;
                self.tima_rst_delay = 0;
            },
            TMA => self.tma = val,
            TAC => self.tac = val,
            _ => unreachable!(),
        };
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            DIV => self.div(),
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self) -> Option<Interrupt> {
        // increment sys_counter/DIV
        let sys_counter_old = self.sys_counter;
        self.sys_counter = self.sys_counter.wrapping_add(1);


        // handle overflow if it was queued
        if self.handle_overflow {
            if self.tima_rst_delay == 0 {
                // copy tma to tima
                self.tima = self.tma;

                self.handle_overflow = false;

                return Some(Interrupt::TimerInt);
            } else {
                self.tima_rst_delay -= 1;
                return None;
            }

        }

        // TAC/TIMA/TMA handling
        self.update_tima(sys_counter_old);

        None
    }

    fn update_tima(&mut self, sys_counter_old: u16) {
        let tac_enable = self.tac & 0x04 != 0;

        if tac_enable {
            let watch_mask = SYS_MASK[(self.tac & 0x03) as usize];

            // falling edge on watched bit = increment
            if is_falling_edge(self.sys_counter, sys_counter_old, watch_mask) {
                if self.tima == 0xFF {
                    // queue reset/interrupt for next m-cycle
                    self.tima = 0;
                    self.tima_rst_delay = 3;
                    self.handle_overflow = true;
                } else {
                    self.tima = self.tima.wrapping_add(1);
                }
            }

        }
    }

}

fn is_falling_edge(new: u16, old: u16, mask: u16) -> bool {
    return (old & mask != 0) && (new & mask == 0)
}