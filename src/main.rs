extern crate simple_nes_rs as nes;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use nes::bus::Bus;
use nes::cartridge::Cartridge;
use nes::cpu;
use nes::cpu_registers::{CpuRegisters, Registers};
use nes::ram::Ram;
use nes::rom::Rom;

fn main() {
    let cartridge = Cartridge::new("./roms/nestest.nes").unwrap();
    let mut console = Console::new(cartridge);

    console.reset();
    console.cpu_registers.set_PC(0xC000);
    console.cpu_registers.set_P(0x24);

    let f = File::open("./assets/nestest.log").unwrap();
    let f = BufReader::new(f);
    for (i, line) in f.lines().enumerate() {
        let line = line.unwrap().bytes().collect::<Vec<_>>();
        let pc: u16 = u16::from_str_radix(&String::from_utf8_lossy(&line[..4]), 16).unwrap();
        let a: u8 = u8::from_str_radix(&String::from_utf8_lossy(&line[50..52]), 16).unwrap();
        let x: u8 = u8::from_str_radix(&String::from_utf8_lossy(&line[55..57]), 16).unwrap();
        let y: u8 = u8::from_str_radix(&String::from_utf8_lossy(&line[60..62]), 16).unwrap();
        let p: u8 = u8::from_str_radix(&String::from_utf8_lossy(&line[65..67]), 16).unwrap();
        let sp: u8 = u8::from_str_radix(&String::from_utf8_lossy(&line[71..73]), 16).unwrap();

        assert_eq!(pc, console.cpu_registers.get_PC());
        assert_eq!(a, console.cpu_registers.get_A());
        assert_eq!(x, console.cpu_registers.get_X());
        assert_eq!(y, console.cpu_registers.get_Y());
        assert_eq!(p, console.cpu_registers.get_P());
        assert_eq!(sp, console.cpu_registers.get_SP());

        let mut cpu_bus = Bus::new(&console.program_rom, &mut console.work_ram);
        cpu::run(&mut console.cpu_registers, &mut cpu_bus, &mut console.nmi);
        println!("{}", i);
    }
}

struct Console {
    program_rom: Rom,
    work_ram: Ram,
    cpu_registers: Registers,
    nmi: bool,
}

impl Console {
    fn new(cartridge: Cartridge) -> Self {
        let program_rom = Rom::new(cartridge.program_rom);
        let work_ram = Ram::new(vec![0; 0x0800]);
        let cpu_registers = Registers::new();
        Self {
            program_rom,
            work_ram,
            cpu_registers,
            nmi: false,
        }
    }
    fn reset(&mut self) {
        let mut cpu_bus = Bus::new(&self.program_rom, &mut self.work_ram);
        cpu::reset(&mut self.cpu_registers, &mut cpu_bus);
    }
}
