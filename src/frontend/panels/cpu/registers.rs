use eframe::egui::{self, RichText};

use crate::debugger::debug_view::DebuggerView;

#[derive(Default)]
pub struct RegisterPanelState {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
    pub temp16: u16,

    pub ir: u8,
    pub temp: u8,
}

impl RegisterPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, view: &DebuggerView) {
    ui.group(|ui| {
        // ui.set_width(ui.available_width());
        
        let total = ui.available_width();
        let col_w = total / 3.0;

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));

            // egui::Grid::new("cpu_register_grid")
            //     .num_columns(3)
            //     .spacing([20.0, 6.0]) // horizontal, vertical
            //     .show(ui, |ui| {
            //         ui.label(format!("AF: {:02X} {:02X}", cpu.a, cpu.f.as_u8()));
            //         ui.label(format!("BC: {:02X} {:02X}", cpu.b, cpu.c));
            //         ui.label(format!("DE: {:02X} {:02X}", cpu.d, cpu.e));
            //         ui.end_row();

            //         ui.label(format!("HL: {:02X} {:02X}", cpu.h, cpu.l));
            //         ui.label(format!("SP: {:04X}", cpu.sp));
            //         ui.label(format!("PC: {:04X}", cpu.pc));
            //         ui.end_row();
            //     });

            ui.columns(3, |cols| {
                cols[0].label(format!("AF: {:02X} {:02X}", view.snapshot.cpu.a, view.snapshot.cpu.f.as_u8()));
                cols[1].label(format!("BC: {:02X} {:02X}", view.snapshot.cpu.b, view.snapshot.cpu.c));
                cols[2].label(format!("DE: {:02X} {:02X}", view.snapshot.cpu.d, view.snapshot.cpu.e));
            });

            ui.columns(3, |cols| {
                cols[0].label(format!("HL: {:02X} {:02X}", view.snapshot.cpu.h, view.snapshot.cpu.l));
                cols[1].label(format!("SP:  {:04X}", view.snapshot.cpu.sp));
                cols[2].label(format!("PC:  {:04X}", view.snapshot.cpu.pc));
            });
            
            ui.separator();
            // egui::Grid::new("ir_scratch_grid")
            //     .num_columns(3)
            //     .spacing([20.0, 6.0]) // horizontal, vertical
            //     .show(ui, |ui| {
            //         ui.label(format!("IR:   {:02X}", cpu.ir));
            //         ui.label(format!(" Z:   {:02X}", cpu.temp));
            //         ui.label(format!("WZ: {:04X}", cpu.pc));
            //         ui.end_row();
            //     });
            ui.columns(3, |cols| {
                cols[0].label(format!("IR:    {:02X}", view.snapshot.cpu.ir));
                cols[1].label(format!(" Z:     {:02X}", view.snapshot.cpu.temp));
                cols[2].label(format!("WZ:  {:04X}", view.snapshot.cpu.temp16));
            });

            let z_color = if view.snapshot.cpu.f.z { egui::Color32::GREEN } else { egui::Color32::RED };
            let n_color = if view.snapshot.cpu.f.n { egui::Color32::GREEN } else { egui::Color32::RED };
            let h_color = if view.snapshot.cpu.f.h { egui::Color32::GREEN } else { egui::Color32::RED };
            let c_color = if view.snapshot.cpu.f.c { egui::Color32::GREEN } else { egui::Color32::RED };
            
            ui.separator();
            egui::Grid::new("flag_list")
                .num_columns(4)
                .spacing([30.0, 6.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Zero").color(z_color));
                    ui.label(RichText::new("Neg").color(n_color));
                    ui.label(RichText::new("H-carry").color(h_color));
                    ui.label(RichText::new("Carry").color(c_color));
                    ui.end_row();
                });
            // ui.columns(4, |cols| {
            //     cols[0].label("Zero   ");
            //     cols[1].label("Neg    ");
            //     cols[2].label("H-carry");
            //     cols[3].label("Carry  ");
            // });


        });
    });
}
}