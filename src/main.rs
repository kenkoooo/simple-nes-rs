extern crate simple_nes_rs as nes;

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use nes::cartridge::Cartridge;
use nes::cpu::CPU;
use nes::emulator::Emulator;
use nes::main_bus::MainBus;
use nes::picture_bus::PictureBus;
use nes::ppu::PPU;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cartridge = Rc::new(RefCell::new(Cartridge::new()));
    let picture_bus = Rc::new(RefCell::new(PictureBus::new()));
    let main_bus = Rc::new(RefCell::new(MainBus::new()));
    let cpu = CPU::new(main_bus.clone());
    let ppu = PPU::new(picture_bus.clone());
    let mut emulator = Emulator {
        cartridge: cartridge,
        picture_bus: picture_bus,
        main_bus: main_bus,
        cpu: cpu,
        ppu: ppu,
    };
    emulator.run(&args[1]);
}
