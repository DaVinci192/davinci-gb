DaVinci_GB is a gameboy color emulator in active development.

Currently the CPU implementation is complete, passing Blargg's cpu_instrs test ROMs

Debugging the CPU was accomplished using a GUI developed with eframe/egui. 

It is not possible to replicate these tests as the bus is being refactored.

The current objective is to implement the PPU, while bringing the bus implementation closer to hardware, with bus arbitration and memory devices correctly implemented.

