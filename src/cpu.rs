use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::main_bus::MainBus;

type Address = u16;
type Byte = u8;

const RESET_VECTOR: Address = 0xfffc;

pub struct CPU {
    main_bus: Rc<RefCell<MainBus>>,

    m_skipCycles: i32,
    m_cycles: i32,

    // Registers
    r_PC: Address,
    r_SP: Byte,
    r_A: Byte,
    r_X: Byte,
    r_Y: Byte,

    // Status flags.
    // Is storing them in one byte better ?
    f_C: bool,
    f_Z: bool,
    f_I: bool,
    f_D: bool,
    f_V: bool,
    f_N: bool,
}

impl CPU {
    pub fn new(main_bus: Rc<RefCell<MainBus>>) -> Self {
        CPU {
            main_bus: main_bus,

            m_skipCycles: 0,
            m_cycles: 0,

            r_PC: 0,
            r_SP: 0xfd,
            r_A: 0,
            r_X: 0,
            r_Y: 0,

            f_C: false,
            f_Z: false,
            f_I: true,
            f_D: false,
            f_V: false,
            f_N: false,
        }
    }

    pub fn reset(&mut self) {
        let a = self.read_address(RESET_VECTOR);
        self.reset_address(a);
    }

    pub fn reset_address(&mut self, start_address: Address) {
        self.m_skipCycles = 0;
        self.m_cycles = 0;

        self.r_A = 0;
        self.r_X = 0;
        self.r_Y = 0;

        self.f_I = true;
        self.f_C = false;
        self.f_D = false;
        self.f_N = false;
        self.f_V = false;
        self.f_Z = false;

        self.r_PC = start_address;
        self.r_SP = 0xfd;
    }

    fn read_address(&mut self, address: Address) -> Address {
        let a = self.main_bus.borrow_mut().read(address) as Address;
        let b = self.main_bus.borrow_mut().read(address + 1) as Address;
        a | b << 8
    }
    pub fn step(&mut self) {
        unimplemented!()
    }
}
