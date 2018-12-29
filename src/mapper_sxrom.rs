use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::mapper::NameTableMirroring;
use crate::picture_bus::PictureBus;

pub struct MapperSxROM {
    mirroring: NameTableMirroring,
    uses_character_ram: bool,
    character_ram: Vec<u8>,
    picture_bus: Rc<RefCell<PictureBus>>,
}

impl MapperSxROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, picture_bus: Rc<RefCell<PictureBus>>) -> Self {
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }

        MapperSxROM {
            picture_bus: picture_bus,
            mirroring: NameTableMirroring::Horizontal,
            uses_character_ram: uses_character_ram,
            character_ram: character_ram,
        }
    }
}
