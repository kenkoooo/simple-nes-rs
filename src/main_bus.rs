use std::cell::RefCell;
use std::rc::Rc;

use crate::controller::Controller;
use crate::mapper::Mapper;
use crate::ppu::PPU;

#[derive(Debug)]
enum IORegisters {
    PPUCTRL,
    PPUMASK,
    PPUSTATUS,
    OAMADDR,
    OAMDATA,
    PPUSCROL,
    PPUADDR,
    PPUDATA,
    OAMDMA,
    JOY1,
    JOY2,
}

impl IORegisters {
    fn get(address: u16) -> Option<IORegisters> {
        match address {
            0x2000 => Some(IORegisters::PPUCTRL),
            0x4014 => Some(IORegisters::OAMDMA),
            0x4016 => Some(IORegisters::JOY1),
            0x4017 => Some(IORegisters::JOY2),
            _ => None,
        }
    }
}

pub struct MainBus {
    mapper: Option<Rc<RefCell<Mapper>>>,
    ppu: Option<Rc<RefCell<PPU>>>,
    controller1: Option<Rc<RefCell<Controller>>>,
    controller2: Option<Rc<RefCell<Controller>>>,
    ram: [u8; 0x800],
    extended_ram: Vec<u8>,
}

impl MainBus {
    pub fn new() -> Self {
        MainBus {
            mapper: None,
            ram: [0; 0x800],
            extended_ram: vec![],
            ppu: None,
            controller1: None,
            controller2: None,
        }
    }
    pub fn set_mapper(&mut self, mapper: Rc<RefCell<Mapper>>) {
        if mapper.borrow().has_extended_ram {
            self.extended_ram.resize(0x2000, 0);
        }
        self.mapper = Some(mapper);
    }

    fn read_callback(&self, register: IORegisters) -> u8 {
        match register {
            IORegisters::PPUSTATUS => self.ppu.as_ref().unwrap().borrow().get_status(),
            IORegisters::PPUDATA => self.ppu.as_ref().unwrap().borrow().get_data(),
            IORegisters::JOY1 => self.controller1.as_ref().unwrap().borrow_mut().read(),
            IORegisters::JOY2 => self.controller2.as_ref().unwrap().borrow_mut().read(),
            IORegisters::OAMDATA => self.ppu.as_ref().unwrap().borrow_mut().get_oam_data(),
            _ => unimplemented!(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let addr = address as usize;
        if address < 0x2000 {
            return self.ram[addr & 0x7ff];
        } else if address < 0x4020 {
            if addr < 0x4000 {
                // PPU registers, mirrored
                match IORegisters::get(address & 0x2007) {
                    Some(register) => self.read_callback(register),
                    None => 0,
                }
            } else if addr < 0x4018 && addr >= 0x4014 {
                match IORegisters::get(address) {
                    Some(register) => self.read_callback(register),
                    None => 0,
                }
            } else {
                0
            }
        } else if address < 0x6000 {
            eprintln!("Expansion ROM read attempted. This is currently unsupported");
            0
        } else if address < 0x8000 {
            if self.mapper.as_ref().unwrap().borrow().has_extended_ram {
                self.extended_ram[addr - 0x6000]
            } else {
                0
            }
        } else {
            self.mapper.as_ref().unwrap().borrow().read_prg(address)
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            self.ram[(addr & 0x7ff) as usize] = value;
        } else if addr < 0x4020 {
            if addr < 0x4000 {
                // PPU registers, mirrored
                match IORegisters::get(addr & 0x2007) {
                    Some(register) => {
                        self.write_callback(register, value);
                    }
                    None => {
                        eprintln!(
                            "No write callback registered for I/O register at: {}",
                            addr & 0x2007
                        );
                    }
                }
            } else if addr < 0x4017 && addr >= 0x4014 {
                // only some registers
                match IORegisters::get(addr) {
                    Some(register) => {
                        self.write_callback(register, value);
                    }
                    None => {
                        eprintln!(
                            "No write callback registered for I/O register at: {}",
                            addr & 0x2007
                        );
                    }
                }
            } else {
                eprintln!("Write access attempt at: {}", addr);
            }
        } else if addr < 0x6000 {
            eprintln!("Expansion ROM access attempted. This is currently unsupported");
        } else if addr < 0x8000 {
            if self.mapper.as_ref().unwrap().borrow().has_extended_ram {
                self.extended_ram[(addr - 0x6000) as usize] = value;
            }
        } else {
            self.mapper
                .as_ref()
                .unwrap()
                .borrow_mut()
                .write_prg(addr, value);
        }
    }

    fn write_callback(&mut self, register: IORegisters, b: u8) {
        match register {
            IORegisters::PPUCTRL => {}
            IORegisters::PPUMASK => {}
            IORegisters::OAMADDR => {}
            IORegisters::PPUADDR => {}
            IORegisters::PPUSCROL => {}
            IORegisters::PPUDATA => {}
            IORegisters::OAMDMA => {}
            IORegisters::JOY1 => {}
            IORegisters::OAMDATA => {}
            _ => unreachable!(),
        }
    }
}
