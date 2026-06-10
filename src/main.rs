use davinci_gb::{core::gameboy::Gameboy, frontend::app::Frontend, debugger::backend::Debugger};
use eframe::{egui};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Gameboy Debugger",
        options,
        Box::new(|cc| Ok(Box::new(Frontend::new(cc)))),
    )
}

// fn main() {
//     let mut gb = Gameboy::new();
//     // let bytes = std::fs::read("src/tests/gb-test-roms-master/mem_timing-2/mem_timing.gb").unwrap();
    

//     // passes all src/tests/cpu_instrs/individual/*
//     // passes src/tests/gb-test-roms-master/instr_timing/instr_timing.gb
//     // 

//     // let bytes = std::fs::read("src/tests/gb-test-roms-master/interrupt_time/interrupt_time.gb").unwrap();
//     let bytes = std::fs::read("src/tests/cpu_instrs/individual/10-bit ops.gb").unwrap();
        
//     gb.load_rom(&bytes);

//     println!("Running");

//     loop {
//         gb.tick();
//     }
// }