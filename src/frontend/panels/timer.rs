use eframe::egui::{self};

use crate::debugger::debug_view::DebuggerView;

#[derive(Default)]
pub struct TimerPanelState;

impl TimerPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, view: &DebuggerView) {
    ui.group(|ui| {
        ui.set_width(ui.available_width());

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));

            ui.heading("Timers");

            ui.separator();
            ui.columns(2, |cols| {
                cols[0].label(format!("SYS:  {:04X}", view.snapshot.bus.timer.sys_counter));
                cols[1].label(format!("DIV:  {:04X}", view.snapshot.bus.timer.div));
            });

            ui.separator();
            ui.columns(3, |cols| {
                cols[0].label(format!("TAC:  {:02X}", view.snapshot.bus.timer.tac));
                cols[1].label(format!("TMA:  {:02X}", view.snapshot.bus.timer.tma));
                cols[2].label(format!("TIMA:  {:02X}", view.snapshot.bus.timer.tima));
            });
        });
    });
}
}