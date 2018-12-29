use std::cell::RefCell;
use std::rc::Rc;

use crate::mapper::{Mapper, NameTableMirroring};

pub struct PictureBus {
    ram: Vec<u8>,
    name_table0: usize,
    name_table1: usize,
    name_table2: usize,
    name_table3: usize,
    mapper: Option<Rc<RefCell<Mapper>>>,
    palette: Vec<u8>,
}

impl PictureBus {
    pub fn new() -> Self {
        PictureBus {
            ram: vec![0; 0x800],
            palette: vec![0; 0x20],
            name_table0: 0,
            name_table1: 0,
            name_table2: 0,
            name_table3: 0,
            mapper: None,
        }
    }

    pub fn set_mapper(&mut self, mapper: Rc<RefCell<Mapper>>) {
        self.mapper = Some(mapper);
        self.update_mirroring();
    }

    pub fn update_mirroring(&mut self) {
        match self
            .mapper
            .as_ref()
            .unwrap()
            .borrow()
            .get_name_table_mirroring()
        {
            NameTableMirroring::Horizontal => {
                self.name_table0 = 0;
                self.name_table1 = 0;
                self.name_table2 = 0x400;
                self.name_table3 = 0x400;
            }
            NameTableMirroring::Vertical => {
                self.name_table0 = 0;
                self.name_table1 = 0x400;
                self.name_table2 = 0;
                self.name_table3 = 0x400;
            }
            NameTableMirroring::OneScreenLower => {
                self.name_table0 = 0;
                self.name_table1 = 0;
                self.name_table2 = 0;
                self.name_table3 = 0;
            }
            NameTableMirroring::OneScreenHigher => {
                self.name_table0 = 0x400;
                self.name_table1 = 0x400;
                self.name_table2 = 0x400;
                self.name_table3 = 0x400;
            }
            _ => {
                self.name_table0 = 0;
                self.name_table1 = 0;
                self.name_table2 = 0;
                self.name_table3 = 0;
            }
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address < 0x2000 {
            self.mapper.as_ref().unwrap().borrow().read_chr(address)
        } else if address < 0x3eff
        // Name tables upto 0x3000, then mirrored upto 3eff
        {
            let index = (address & 0x3ff) as usize;
            if address < 0x2400 {
                // NT0
                self.ram[self.name_table0 + index]
            } else if address < 0x2800 {
                // NT1
                self.ram[self.name_table1 + index]
            } else if address < 0x2c00 {
                // NT2
                self.ram[self.name_table2 + index]
            } else {
                // NT3
                self.ram[self.name_table3 + index]
            }
        } else if address < 0x3fff {
            self.palette[(address & 0x1f) as usize]
        } else {
            0
        }
    }

    pub fn read_palette(&self, palette_address: u16) -> u8 {
        self.palette[palette_address as usize]
    }
}
