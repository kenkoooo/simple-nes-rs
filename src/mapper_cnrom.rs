use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;

pub struct MapperCNROM {
    cartridge: Rc<RefCell<Cartridge>>,
    one_bank: bool,
}

impl MapperCNROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let one_bank = cartridge.borrow().prg_rom.len() == 0x4000;
        MapperCNROM {
            cartridge: cartridge,
            one_bank: one_bank,
        }
    }
    pub fn read_prg(&self, address: u16) -> u8 {
        let address = address as usize;
        if !self.one_bank {
            self.cartridge.borrow().prg_rom[address - 0x8000]
        } else {
            self.cartridge.borrow().prg_rom[(address - 0x8000) & 0x3fff]
        }
    }
}
