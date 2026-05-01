use eframe::egui::{self};
use egui_extras::{TableBuilder, Column};


use crate::debugger::{backend::Debugger, debug_view::DebuggerView};

pub struct DisassemblerPanelState {
    pub display_rows: usize,
    pub prev_disasm: String,
}

impl Default for DisassemblerPanelState {
    fn default() -> Self {
        Self { display_rows: 20, prev_disasm: String::new() }
    }
}

impl DisassemblerPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, view: &DebuggerView) {
    ui.group(|ui| {
        ui.set_width(ui.available_width());

        ui.scope(|ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::monospace(18.0));

            // ui.label(format!("Executing: {}", self.prev_disasm));
            // ui.separator();

            egui::ScrollArea::vertical().max_height(500.0).id_salt("disassembly_output_area").show(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::auto().resizable(false))
                    .column(Column::auto().resizable(false))
                    .column(Column::remainder())
                    .column(Column::auto().resizable(false))
                    .header(20.0, |mut header| {
                        header.col(|ui| { ui.label("ADDR"); });
                        header.col(|ui| { ui.label("HEX"); });
                        header.col(|ui| { ui.label("DISASM"); });
                        header.col(|ui| { ui.label("CYCLE"); });
                    })
                    .body(|mut body| {
                        let pc = view.snapshot.cpu.pc;
                        let rows = view.disassembly(pc, self.display_rows);
                        
                        // self.prev_disasm = rows[0].text.clone();

                        for row in rows {
                            let hex = row.bytes.iter()
                                .map(|b| format!("{:02X}", b))
                                .collect::<Vec<_>>()
                                .join(" ");

                            body.row(18.0, |mut table_row| {
                                table_row.col(|ui| { ui.label(format!("{:04X}", row.addr)); });
                                table_row.col(|ui| { ui.label(hex); });
                                table_row.col(|ui| { ui.label(&row.text); });
                                table_row.col(|ui| { 
                                    if row.cycles_min == row.cycles_max {
                                        ui.label(format!("{}", row.cycles_min));
                                    } else {
                                        ui.label(format!("{}/{}", row.cycles_min, row.cycles_max));
                                    }
                                });
                            });
                        }

                    });
            });

        });
    });
}
}