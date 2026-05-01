use eframe::egui::{self};

use crate::debugger::debug_view::DebuggerView;

#[derive(Default)]
pub struct ControlPanelState {
    pub is_running: bool,
}

impl ControlPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, view: &mut DebuggerView) {
    ui.group(|ui| {
        ui.set_width(ui.available_width());

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));

            ui.horizontal(|ui| {
                if ui.button("Run").clicked() {
                    self.is_running = true;
                }
                if ui.button("Pause").clicked() {
                    self.is_running = false;
                }
                if ui.button("Reset").clicked() {
                    if !self.is_running {
                        view.reset();
                    }
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Step Instr").clicked() {
                    if !self.is_running {
                        view.step_instruction();
                    }
                }
                if ui.button("Step M").clicked() {
                    if !self.is_running {
                        view.step_cycle();
                    }
                }
                if ui.button("Tick").clicked() {
                    if !self.is_running {
                        view.tick();
                    }
                }
            });
            

        });
    });
}
}