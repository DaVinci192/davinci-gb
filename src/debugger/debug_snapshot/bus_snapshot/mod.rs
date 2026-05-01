pub mod timer_snapshot;

use self::timer_snapshot::TimerSnapshot;

use super::super::super::core::bus::Bus;

#[derive(Clone, Debug)]
pub struct BusSnapshot {
    pub rom_bank: u8,
    pub ram_enabled: bool,

    pub dma_active: bool,
    pub dma_source: u8,

    pub serial_output: Vec<u8>,

    pub sb: u8,
    pub sc: u8,

    pub ie: u8,
    pub iflag: u8,

    pub timer: TimerSnapshot,
}


impl From<&Bus> for BusSnapshot {
    fn from(bus: &Bus) -> Self {
        Self {
            rom_bank: 0,
            ram_enabled: false,

            dma_active: false,
            dma_source: 0,

            serial_output: bus.serial.serial_output.clone(),

            sb: bus.memory[0xFF01],
            sc: bus.memory[0xFF02],

            ie: bus.ie(),
            iflag: bus.iflag(),

            timer: TimerSnapshot::from(&bus.timer),
        }
    }
}

