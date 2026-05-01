use eframe::egui::{self};

use crate::debugger::debug_view::DebuggerView;

#[derive(Default)]
pub struct MetricPanelState {
    pub cycles: u64,
    pub instructions: u64,
    pub fps: f32,
    pub clock_hz: u32,
    pub is_running: bool,
}

impl MetricPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, view: &DebuggerView) {
    ui.group(|ui| {
        ui.set_width(ui.available_width());

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));

            egui::Grid::new("metrics_list")
                .num_columns(2)
                .spacing([23.0, 6.0])
                .show(ui, |ui| {
                    ui.label(format!("State: {}", 
                        if self.is_running { "Running" } else { "Paused" }
                    ));
                    ui.label("FPS: --");
                    ui.end_row();

                    ui.label(format!("Instr: {}", 
                        self.instructions
                    ));
                    ui.label("Clock: 1x");
                    ui.end_row();

                    ui.label(format!("Cycles: {}", 
                        view.snapshot.cpu.m_cycles
                    ));
                    ui.end_row();
                });
            

        });
    });
}
}