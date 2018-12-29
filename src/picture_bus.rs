use std::cell::RefCell;
use std::rc::Rc;

use crate::mapper::Mapper;

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
}
