use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::mapper::Mapper;

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
    fn get(address: u16) -> IORegisters {
        match address {
            0x2000 => IORegisters::PPUCTRL,
            0x4014 => IORegisters::OAMDMA,
            0x4016 => IORegisters::JOY1,
            0x4017 => IORegisters::JOY2,
            _ => unimplemented!(),
        }
    }
}

pub struct MainBus {
    mapper: Option<Rc<RefCell<Mapper>>>,
    ppu: Rc<RefCell<PPU>>,
    controller1: Rc<Ref<Controller>>,
    controller2: Rc<Ref<Controller>>,
    ram: [u8; 0x800],
    extended_ram: Vec<u8>,
}

impl MainBus {
    pub fn new() -> Self {
        MainBus {
            mapper: None,
            ram: [0; 0x800],
            extended_ram: vec![],
        }
    }
    pub fn set_mapper(&mut self, mapper: Rc<RefCell<Mapper>>) {
        if mapper.borrow().has_extended_ram {
            self.extended_ram.resize(0x2000, 0);
        }
        self.mapper = Some(mapper);
    }

    fn callback(&self, register: IORegisters) {
        match register {}
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let addr = address as usize;
        if address < 0x2000 {
            return self.ram[addr & 0x7ff];
        } else if address < 0x4020 {
            if addr < 0x4000 {
                // PPU registers, mirrored
                let register = IORegisters::get(address & 0x2007);
            //   auto it = m_readCallbacks.find(static_cast<IORegisters>(addr & 0x2007));
            //   if (it != m_readCallbacks.end())
            //     return (it->second)();
            //   // Second object is the pointer to the function object
            //   // Dereference the function pointer and call it
            //   else
            //     LOG(InfoVerbose) << "No read callback registered for I/O register at: "
            //                      << std::hex << +addr << std::endl;
            } else if addr < 0x4018 && addr >= 0x4014 {
                // Only *some* IO registers

                //   auto it = m_readCallbacks.find(static_cast<IORegisters>(addr));
                //   if (it != m_readCallbacks.end())
                //     return (it->second)();
                //   // Second object is the pointer to the function object
                //   // Dereference the function pointer and call it
                //   else
                //     LOG(InfoVerbose) << "No read callback registered for I/O register at: "
                //                      << std::hex << +addr << std::endl;
            } else {
                //   LOG(InfoVerbose) << "Read access attempt at: " << std::hex << +addr
                //                    << std::endl;
            }
            unimplemented!()
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
}
