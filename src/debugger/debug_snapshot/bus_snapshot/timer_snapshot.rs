use super::super::super::super::core::bus::timer::Timer;

#[derive(Clone, Debug)]
pub struct TimerSnapshot {
    pub sys_counter: u16,

    pub div: u8,

    pub tac: u8,
    pub tma: u8,
    pub tima: u8,
}


impl From<&Timer> for TimerSnapshot {
    fn from(timer: &Timer) -> Self {
        Self {
            sys_counter: timer.sys_counter(),

            div: timer.div(),

            tac: timer.tac(),
            tma: timer.tma(),
            tima: timer.tima(),
        }
    }
}

