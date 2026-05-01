pub mod cpu;
pub mod memory;
pub mod ppu;
pub mod dma;
pub mod interrupts;
pub mod timer;
pub mod cartridge;
pub mod metrics;
pub mod serial;

use eframe::egui::{PanelState, Ui};

use crate::debugger::backend::Debugger;

pub trait Panel {
    fn show(ui: &mut Ui, debugger: &Debugger, state: &mut PanelState);
}