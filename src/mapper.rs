use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::mapper_cnrom::MapperCNROM;
use crate::mapper_nrom::MapperNROM;
use crate::mapper_sxrom::MapperSxROM;
use crate::mapper_uxrom::MapperUxROM;
use crate::picture_bus::PictureBus;

pub enum NameTableMirroring {
    Horizontal,
    Vertical,
    FourScreen,
    OneScreenLower,
    OneScreenHigher,
}

pub enum Mapper {
    MapperCNROM(MapperCNROM),
    MapperNROM(MapperNROM),
    MapperSxROM(MapperSxROM),
    MapperUxROM(MapperUxROM),
}

impl Mapper {
    pub fn new(
        mapper_number: u8,
        cartridge: Rc<RefCell<Cartridge>>,
        picture_bus: Rc<RefCell<PictureBus>>,
    ) -> Self {
        match mapper_number {
            0 => Mapper::MapperNROM(MapperNROM::new(cartridge)),
            1 => Mapper::MapperSxROM(MapperSxROM::new(cartridge, picture_bus)),
            2 => Mapper::MapperUxROM(MapperUxROM::new(cartridge)),
            3 => Mapper::MapperCNROM(MapperCNROM::new(cartridge)),
            _ => unimplemented!("Mapper {} is not implemented", mapper_number),
        }
    }
}
