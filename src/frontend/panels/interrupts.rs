use eframe::egui::{self, RichText};

use crate::{debugger::debug_view::DebuggerView};

#[derive(Default)]
pub struct InterruptPanelState {
    pub ime: bool,
    pub ie: u8,
    pub iflag: u8,
}

impl InterruptPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, view: &DebuggerView) {
    ui.group(|ui| {
        ui.set_width(ui.available_width());

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));

            ui.label(format!("IME: {}", {
                if view.snapshot.cpu.ime { "Enabled" } else { "Disabled "}
            }));
            ui.label(format!("IE:   {:08b}", view.snapshot.bus.ie));
            ui.label(format!("IF:   {:08b}", view.snapshot.bus.iflag));

            let joypad_color = if view.snapshot.bus.iflag & 0x10 != 0 { egui::Color32::GREEN } else { egui::Color32::RED };
            let serial_color = if view.snapshot.bus.iflag & 0x08 != 0 { egui::Color32::GREEN } else { egui::Color32::RED };
            let timer_color = if view.snapshot.bus.iflag & 0x04 != 0 { egui::Color32::GREEN } else { egui::Color32::RED };
            let lcd_color = if view.snapshot.bus.iflag & 0x02 != 0 { egui::Color32::GREEN } else { egui::Color32::RED };
            let vblank_color = if view.snapshot.bus.iflag & 0x01 != 0 { egui::Color32::GREEN } else { egui::Color32::RED };

            ui.separator();
            egui::Grid::new("interrupt_flag_list")
                .num_columns(5)
                .spacing([23.0, 6.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Joypad").color(joypad_color));
                    ui.label(RichText::new("Serial").color(serial_color));
                    ui.label(RichText::new("Timer").color(timer_color));
                    ui.label(RichText::new("LCD").color(lcd_color));
                    ui.label(RichText::new("VBlank").color(vblank_color));
                    ui.end_row();
                });
            

        });
    });
}
}