use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;

pub struct MapperNROM {
    cartridge: Rc<RefCell<Cartridge>>,
    one_bank: bool,
    uses_character_ram: bool,
    character_ram: Vec<u8>,
}

impl MapperNROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let one_bank = cartridge.borrow().prg_rom.len() == 0x4000;
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }
        MapperNROM {
            cartridge: cartridge,
            one_bank: one_bank,
            uses_character_ram: uses_character_ram,
            character_ram: character_ram,
        }
    }

    pub fn read_prg(&self, address: u16) -> u8 {
        let addr = address as usize;
        if !self.one_bank {
            self.cartridge.borrow().prg_rom[addr - 0x8000]
        } else {
            self.cartridge.borrow().prg_rom[(addr - 0x8000) & 0x3fff]
        }
    }
}
