use std::time::Duration;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use eframe::App;
use crate::core::bus::{serial, timer};
use crate::debugger::debug_view::DebuggerView;
use crate::frontend::layout::RIGHT_PANEL_WIDTH;
use crate::frontend::panels::cpu::control::ControlPanelState;
use crate::frontend::panels::cpu::disassembler::DisassemblerPanelState;
use crate::frontend::panels::interrupts::InterruptPanelState;
use crate::frontend::panels::metrics::MetricPanelState;
use crate::frontend::panels::serial::SerialPanelState;
use crate::frontend::panels::timer::TimerPanelState;

use super::super::debugger::backend::Debugger;

use super::font_config::{configure_text_styles, replace_fonts};

use super::layout::{LEFT_PANEL_MAX_WIDTH, LEFT_PANEL_WIDTH, LEFT_PANEL_MIN_WIDTH};


use super::panels::cpu::registers::RegisterPanelState;

pub struct Frontend {
    debugger: Debugger,

    control_panel: ControlPanelState,
    register_panel: RegisterPanelState,
    interrupt_panel: InterruptPanelState,
    metrics_panel: MetricPanelState,
    disassembler_panel: DisassemblerPanelState,
    serial_panel: SerialPanelState,
    timer_panel: TimerPanelState,

    last_time: std::time::Instant,
    accumulator: f64,
}

impl Frontend {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        replace_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        

        let mut debugger = Debugger::new();
        debugger.load_rom("src/tests/cpu_instrs/individual/10-bit ops.gb");

        let control_panel = ControlPanelState::default();
        let register_panel = RegisterPanelState::default();
        let interrupt_panel = InterruptPanelState::default();
        let metrics_panel = MetricPanelState::default();
        let disassembler_panel = DisassemblerPanelState::default();
        let serial_panel = SerialPanelState::default();
        let timer_panel = TimerPanelState::default();

        // set_visuals(&cc.egui_ctx);
        let frontend = Self {
            debugger: debugger,

            control_panel: control_panel,
            register_panel: register_panel,
            interrupt_panel: interrupt_panel,
            metrics_panel: metrics_panel,
            disassembler_panel: disassembler_panel,
            serial_panel: serial_panel,
            timer_panel: timer_panel,

            last_time: std::time::Instant::now(),
            accumulator: 0.0,
        };

        // let bytes = vec![0u8; 16];

        frontend
        
        // let bytes = std::fs::read("src/tests/cpu_instrs/individual/10-bit ops.gb").unwrap();
        // frontend.debugger.gb.load_rom(&bytes);

        // frontend.debugger.load_rom("src/tests/cpu_instrs/individual/10-bit ops.gb");
        // let N = 5_001_264; // RST near 5001264
        // let N = 478_500; // after serial = 150_000
        // let N = 478_900;
        // OP LD SP,HL - M-cycles: 479178
        // let N = 479_170;
        // let mut n = 0;
        // loop {
        //     frontend.debugger.step_cycle();
        //     n += 1;
            
        //     if n >= N { break; }
        // }

        // frontend
    }
}


impl App for Frontend {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.control_panel.is_running {
            // self.debugger.step_instruction();
            self.debugger.step_cycle();
            // ctx.request_repaint_after(Duration::from_millis(8));
            ctx.request_repaint();
        }
    }

    

    // fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    //     let now = std::time::Instant::now();
    //     let dt = now - self.last_time;
    //     self.last_time = now;

    //     self.accumulator += dt.as_secs_f64();

    //     if self.control_panel.is_running {
    //         let seconds_per_mcycle = 1.0 / 1_048_576.0;

    //         while self.accumulator >= seconds_per_mcycle {
    //             self.debugger.step_cycle();
    //             self.accumulator -= seconds_per_mcycle;
    //         }
    //         // self.debugger.step_instruction();
    //         // ctx.request_repaint_after(Duration::from_millis(8));
            
    //     }

    //     ctx.request_repaint();
    // }


    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        // if self.control_panel.is_running {
        //     self.debugger.step_instruction();
            
        // }

        let snapshot = self.debugger.snapshot();
        let view = &mut DebuggerView::new(&mut self.debugger, &snapshot);

        set_visuals_ui(ui);

        // Top control bar
        // egui::Panel::top("top_bar").show_inside(ui, |ui| {
        //     ui.heading("Game Boy Debugger");
        // });

        egui::Panel::left("left_panel")
            .default_size(LEFT_PANEL_WIDTH)
            .resizable(false)
            .max_size(LEFT_PANEL_MAX_WIDTH)
            .min_size(LEFT_PANEL_MIN_WIDTH)
            .show_inside(ui, |ui| {
                self.control_panel.show(ui, view);
                self.metrics_panel.show(ui, view);
                self.register_panel.show(ui, &view);
                self.interrupt_panel.show(ui, &view);
                self.disassembler_panel.show(ui, &view);
                self.serial_panel.show(ui, &view);
            });

        egui::Panel::right("right_panel")
            .default_size(RIGHT_PANEL_WIDTH)
            .resizable(false)
            .max_size(LEFT_PANEL_MAX_WIDTH)
            .min_size(LEFT_PANEL_MIN_WIDTH)
            .show_inside(ui, |ui| {
                self.timer_panel.show(ui, &view);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.group(|ui| {
                ui.heading("Memory Viewer");
                ui.label("Coming soon...");
            });
        });
    }
}


fn set_visuals_ui(ui: &mut egui::Ui) {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = egui::Color32::from_rgb(12,12,12);
    visuals.window_fill = egui::Color32::from_rgb(12,12,12);
    visuals.extreme_bg_color = egui::Color32::from_rgb(12,12,12);
    visuals.faint_bg_color = egui::Color32::from_gray(20);

    visuals.override_text_color = Some(egui::Color32::WHITE);

    visuals.widgets.noninteractive.bg_stroke = 
        egui::Stroke::new(1.0, egui::Color32::GRAY);

    visuals.widgets.inactive.bg_stroke = 
        egui::Stroke::new(1.0, egui::Color32::GRAY);

    visuals.widgets.hovered.bg_stroke = 
        egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE);;

    visuals.widgets.active.bg_stroke.color = egui::Color32::YELLOW;

    ui.set_visuals(visuals);
}