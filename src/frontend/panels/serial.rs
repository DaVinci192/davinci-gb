use eframe::egui::{self};
use egui_extras::{TableBuilder, Column};


use crate::debugger::{backend::Debugger, debug_view::DebuggerView};

pub struct SerialPanelState {
    pub last_index: usize,
    pub text_cache: String,
}

impl Default for SerialPanelState {
    fn default() -> Self {
        Self { 
           last_index: 0,
           text_cache: String::new(),
        }
    }
}

impl SerialPanelState {
    fn update(&mut self, view: &DebuggerView) {
        let data = &view.snapshot.bus.serial_output;

        if self.last_index < data.len() {
            for &b in &data[self.last_index..] {
                self.text_cache.push(b as char);
            }

            self.last_index = data.len();
        }

    }

    pub fn show(&mut self, ui: &mut egui::Ui, view: &DebuggerView) {
    ui.group(|ui| {
        ui.set_width(ui.available_width());

        self.update(view);

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));
                
            ui.heading("Serial Output");

            ui.separator();
            ui.columns(2, |cols| {
                cols[0].label(format!("SB:  {:02X}", view.snapshot.bus.sb));
                cols[1].label(format!("SC:  {:02X}", view.snapshot.bus.sc));
            });

            ui.separator();
            egui::ScrollArea::vertical()
                .id_salt("serial_output_area")
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.monospace(&self.text_cache);
                });

        });
    });
}
}