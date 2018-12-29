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

pub struct Mapper {
    mapper: MapperEnum,
    cartridge: Rc<RefCell<Cartridge>>,
}

pub enum MapperEnum {
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
        let m_cartridge = cartridge.clone();
        let mapper = match mapper_number {
            0 => MapperEnum::MapperNROM(MapperNROM::new(cartridge)),
            1 => MapperEnum::MapperSxROM(MapperSxROM::new(cartridge, picture_bus)),
            2 => MapperEnum::MapperUxROM(MapperUxROM::new(cartridge)),
            3 => MapperEnum::MapperCNROM(MapperCNROM::new(cartridge)),
            _ => unimplemented!("Mapper {} is not implemented", mapper_number),
        };
        Mapper {
            mapper: mapper,
            cartridge: m_cartridge,
        }
    }

    pub fn has_extended_ram(&self) -> bool {
        unimplemented!()
    }

    pub fn get_name_table_mirroring(&self) -> NameTableMirroring {
        let name_table_mirroring = self.cartridge.borrow().name_table_mirroring;
        match name_table_mirroring {
            0 => NameTableMirroring::Horizontal,
            1 => NameTableMirroring::Vertical,
            8 => NameTableMirroring::FourScreen,
            _ => unimplemented!("Unsupported Name Table Mirroring: {}", name_table_mirroring),
        }
    }
}
