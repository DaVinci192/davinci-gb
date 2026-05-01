use davinci_gb::{core::gameboy::Gameboy, frontend::app::Frontend};
use eframe::{egui};

// fn main() -> eframe::Result {
//     let options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "Gameboy Debugger",
//         options,
//         Box::new(|cc| Ok(Box::new(Frontend::new(cc)))),
//     )
// }

fn main() {
    let mut gb = Gameboy::new();
    let bytes = std::fs::read("src/tests/gb-test-roms-master/cpu_instrs/cpu_instrs.gb").unwrap();
        
    gb.load_rom(&bytes);

    loop {
        gb.tick();
    }
}