use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;

pub struct MapperUxROM {
    uses_character_ram: bool,
    character_ram: Vec<u8>,
}

impl MapperUxROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }
        MapperUxROM {
            character_ram: character_ram,
            uses_character_ram: uses_character_ram,
        }
    }
}
