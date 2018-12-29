use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;

pub struct MapperUxROM {
    uses_character_ram: bool,
    character_ram: Vec<u8>,
    cartridge: Rc<RefCell<Cartridge>>,
    last_bank_idx: usize,
    select_prg: usize,
}

impl MapperUxROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }
        let last_bank_idx = cartridge.borrow().prg_rom.len() - 0x4000;
        MapperUxROM {
            character_ram: character_ram,
            uses_character_ram: uses_character_ram,
            last_bank_idx: last_bank_idx,
            cartridge: cartridge,
            select_prg: 0,
        }
    }

    pub fn read_prg(&self, address: u16) -> u8 {
        let addr = address as usize;
        if addr < 0xc000 {
            self.cartridge.borrow().prg_rom[((addr - 0x8000) & 0x3fff) | (self.select_prg << 14)]
        } else {
            self.cartridge.borrow().prg_rom[self.last_bank_idx + (addr & 0x3fff)]
        }
    }
}
