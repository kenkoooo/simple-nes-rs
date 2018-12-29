use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::main_bus::MainBus;

type Address = u16;
type Byte = u8;
const INSTRUCTION_MODE_MASK: Byte = 0x3;

const OPERATION_MASK: Byte = 0xe0;
const OPERATION_SHIFT: Byte = 5;

const ADDR_MODE_MASK: Byte = 0x1c;
const ADDR_MODE_SHIFT: Byte = 2;

const BRANCH_INSTRUCTION_MASK: Byte = 0x1f;
const BRANCH_INSTRUCTION_MASK_RESULT: Byte = 0x10;
const BRANCH_CONDITION_MASK: Byte = 0x20;
const BRANCH_ON_FLAG_SHIFT: Byte = 6;

const RESET_VECTOR: Address = 0xfffc;
const OPERATION_CYCLES: [i32; 0x100] = [
    7, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 0, 4, 6, 0, 2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 3, 3, 5, 0, 4, 2, 2, 0, 4, 4, 6, 0, 2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 3, 4, 6, 0, 2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 4, 2, 2, 0, 5, 4, 6, 0, 2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    0, 6, 0, 0, 3, 3, 3, 0, 2, 0, 2, 0, 4, 4, 4, 0, 2, 6, 0, 0, 4, 4, 4, 0, 2, 5, 2, 0, 0, 5, 0, 0,
    2, 6, 2, 0, 3, 3, 3, 0, 2, 2, 2, 0, 4, 4, 4, 0, 2, 5, 0, 0, 4, 4, 4, 0, 2, 4, 2, 0, 4, 4, 4, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0, 2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 2, 4, 4, 6, 0, 2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
];

#[derive(PartialEq)]
enum AddrMode2 {
    Immediate_,
    ZeroPage_,
    Accumulator,
    Absolute_,
    Indexed,
    AbsoluteIndexed,
    Unsupported,
}

impl AddrMode2 {
    fn get(b: u8) -> AddrMode2 {
        match b {
            0 => AddrMode2::Immediate_,
            1 => AddrMode2::ZeroPage_,
            2 => AddrMode2::Accumulator,
            3 => AddrMode2::Absolute_,
            5 => AddrMode2::Indexed,
            7 => AddrMode2::AbsoluteIndexed,
            _ => AddrMode2::Unsupported,
        }
    }
}

enum Operation0 {
    BIT,
    STY,
    LDY,
    CPY,
    CPX,
    Unsupported,
}
impl Operation0 {
    fn get(b: u8) -> Operation0 {
        match b {
            1 => Operation0::BIT,
            4 => Operation0::STY,
            5 => Operation0::LDY,
            6 => Operation0::CPY,
            7 => Operation0::CPX,
            _ => Operation0::Unsupported,
        }
    }
}

#[derive(PartialEq)]
enum Operation1 {
    ORA,
    AND,
    EOR,
    ADC,
    STA,
    LDA,
    CMP,
    SBC,
    Unsupported,
}

impl Operation1 {
    fn get(x: u8) -> Operation1 {
        match x {
            0 => Operation1::ORA,
            1 => Operation1::AND,
            2 => Operation1::EOR,
            3 => Operation1::ADC,
            4 => Operation1::STA,
            5 => Operation1::LDA,
            6 => Operation1::CMP,
            7 => Operation1::SBC,
            _ => Operation1::Unsupported,
        }
    }
}

enum AddrMode1 {
    IndexedIndirectX,
    ZeroPage,
    Immediate,
    Absolute,
    IndirectY,
    IndexedX,
    AbsoluteY,
    AbsoluteX,
    Unsupported,
}

impl AddrMode1 {
    fn get(x: u8) -> AddrMode1 {
        match x {
            0 => AddrMode1::IndexedIndirectX,
            1 => AddrMode1::ZeroPage,
            2 => AddrMode1::Immediate,
            3 => AddrMode1::Absolute,
            4 => AddrMode1::IndirectY,
            5 => AddrMode1::IndexedX,
            6 => AddrMode1::AbsoluteY,
            7 => AddrMode1::AbsoluteX,
            _ => AddrMode1::Unsupported,
        }
    }
}

#[derive(PartialEq)]
enum Operation2 {
    ASL,
    ROL,
    LSR,
    ROR,
    STX,
    LDX,
    DEC,
    INC,
    Unsupported,
}
impl Operation2 {
    fn get(b: u8) -> Operation2 {
        match b {
            0 => Operation2::ASL,
            1 => Operation2::ROL,
            2 => Operation2::LSR,
            3 => Operation2::ROR,
            4 => Operation2::STX,
            5 => Operation2::LDX,
            6 => Operation2::DEC,
            7 => Operation2::INC,
            _ => Operation2::Unsupported,
        }
    }
}

enum OperationImplied {
    NOP,
    BRK,
    JSR,
    RTI,
    RTS,
    JMP,
    JMPI,
    PHP,
    PLP,
    PHA,
    PLA,
    DEY,
    DEX,
    TAY,
    INY,
    INX,
    CLC,
    SEC,
    CLI,
    SEI,
    TYA,
    CLV,
    CLD,
    SED,
    TXA,
    TXS,
    TAX,
    TSX,
    Unsupported,
}

impl OperationImplied {
    fn get(opcode: Byte) -> OperationImplied {
        match opcode {
            0xea => OperationImplied::NOP,
            0x00 => OperationImplied::BRK,
            0x20 => OperationImplied::JSR,
            0x40 => OperationImplied::RTI,
            0x60 => OperationImplied::RTS,

            0x4C => OperationImplied::JMP,
            0x6C => OperationImplied::JMPI, // JMP Indirect

            0x08 => OperationImplied::PHP,
            0x28 => OperationImplied::PLP,
            0x48 => OperationImplied::PHA,
            0x68 => OperationImplied::PLA,

            0x88 => OperationImplied::DEY,
            0xca => OperationImplied::DEX,
            0xa8 => OperationImplied::TAY,
            0xc8 => OperationImplied::INY,
            0xe8 => OperationImplied::INX,

            0x18 => OperationImplied::CLC,
            0x38 => OperationImplied::SEC,
            0x58 => OperationImplied::CLI,
            0x78 => OperationImplied::SEI,
            0x98 => OperationImplied::TYA,
            0xb8 => OperationImplied::CLV,
            0xd8 => OperationImplied::CLD,
            0xf8 => OperationImplied::SED,

            0x8a => OperationImplied::TXA,
            0x9a => OperationImplied::TXS,
            0xaa => OperationImplied::TAX,
            0xba => OperationImplied::TSX,
            _ => OperationImplied::Unsupported,
        }
    }
}

enum InterruptType {
    IRQ,
    NMI,
    BRK_,
}

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
        self.m_cycles += 1;
        self.m_skipCycles -= 1;
        if self.m_skipCycles > 0 {
            return;
        }

        self.m_skipCycles = 0;
        let psw = if self.f_N { 1 } else { 0 } << 7
            | if self.f_V { 1 } else { 0 } << 6
            | 1 << 5
            | if self.f_D { 1 } else { 0 } << 3
            | if self.f_I { 1 } else { 0 } << 2
            | if self.f_Z { 1 } else { 0 } << 1
            | if self.f_C { 1 } else { 0 };

        self.r_PC += 1;
        let opcode = self.main_bus.borrow().read(self.r_PC);
        let cycle_length = OPERATION_CYCLES[opcode as usize];
        if cycle_length != 0
            && (self.execute_implied(opcode)
                || self.executed_branch(opcode)
                || self.executed_type0(opcode)
                || self.executed_type1(opcode)
                || self.executed_type2(opcode))
        {
            self.m_skipCycles += cycle_length;
        } else {
            eprintln!("Unrecognized opcode: {}", opcode);
        }
    }

    fn execute_implied(&mut self, opcode: u8) -> bool {
        match OperationImplied::get(opcode) {
            OperationImplied::NOP => {}
            OperationImplied::BRK => {
                self.interrupt(InterruptType::BRK_);
            }
            OperationImplied::JSR => {
                self.push_stack(((self.r_PC + 1) >> 8) as u8);
                self.push_stack((self.r_PC + 1) as u8);
                self.r_PC = self.read_address(self.r_PC);
            }
            OperationImplied::RTS => {
                self.r_PC = self.pull_stack() as u16;
                self.r_PC |= (self.pull_stack() as u16) << 8;
                self.r_PC += 1;
            }
            OperationImplied::RTI => {
                let flags = self.pull_stack();
                self.f_N = (flags & 0x80) != 0;
                self.f_V = (flags & 0x40) != 0;
                self.f_D = (flags & 0x8) != 0;
                self.f_I = (flags & 0x4) != 0;
                self.f_Z = (flags & 0x2) != 0;
                self.f_C = (flags & 0x1) != 0;
                self.r_PC = self.pull_stack() as u16;
                self.r_PC |= (self.pull_stack() as u16) << 8;
            }
            OperationImplied::JMP => {
                self.r_PC = self.read_address(self.r_PC);
            }
            OperationImplied::JMPI => {
                let location = self.read_address(self.r_PC);
                // 6502 has a bug such that the when the vector of anindirect address begins
                // at the last byte of a page, the second byte is fetched from the beginning
                // of that page rather than the beginning of the next Recreating here:
                let page = location & 0xff00;
                self.r_PC = (self.main_bus.borrow().read(location) as u16)
                    | (self.main_bus.borrow().read(page | ((location + 1) & 0xff)) as u16) << 8;
            }
            OperationImplied::PHP => {
                let flags = if self.f_N { 1 } else { 0 } << 7
                    | if self.f_V { 1 } else { 0 } << 6
                    | 1 << 5
                    | 1 << 4
                    | if self.f_D { 1 } else { 0 } << 3
                    | if self.f_I { 1 } else { 0 } << 2
                    | if self.f_Z { 1 } else { 0 } << 1
                    | if self.f_C { 1 } else { 0 };
                self.push_stack(flags);
            }
            OperationImplied::PLP => {
                let flags = self.pull_stack();
                self.f_N = (flags & 0x80) != 0;
                self.f_V = (flags & 0x40) != 0;
                self.f_D = (flags & 0x8) != 0;
                self.f_I = (flags & 0x4) != 0;
                self.f_Z = (flags & 0x2) != 0;
                self.f_C = (flags & 0x1) != 0;
            }
            OperationImplied::PHA => {
                self.push_stack(self.r_A);
            }
            OperationImplied::PLA => {
                self.r_A = self.pull_stack();
                self.set_zn(self.r_A);
            }
            OperationImplied::DEY => {
                self.r_Y -= 1;
                self.set_zn(self.r_Y);
            }
            OperationImplied::DEX => {
                self.r_X -= 1;
                self.set_zn(self.r_X);
            }
            OperationImplied::TAY => {
                self.r_Y = self.r_A;
                self.set_zn(self.r_Y);
            }
            OperationImplied::INY => {
                self.r_Y += 1;
                self.set_zn(self.r_Y);
            }
            OperationImplied::INX => {
                self.r_X += 1;
                self.set_zn(self.r_X);
            }
            OperationImplied::CLC => {
                self.f_C = false;
            }
            OperationImplied::SEC => {
                self.f_C = true;
            }
            OperationImplied::CLI => {
                self.f_I = false;
            }
            OperationImplied::SEI => {
                self.f_I = true;
            }
            OperationImplied::CLD => {
                self.f_D = false;
            }
            OperationImplied::SED => {
                self.f_D = true;
            }
            OperationImplied::TYA => {
                self.r_A = self.r_Y;
                self.set_zn(self.r_A);
            }
            OperationImplied::CLV => {
                self.f_V = false;
            }
            OperationImplied::TXA => {
                self.r_A = self.r_X;
                self.set_zn(self.r_A);
            }
            OperationImplied::TXS => {
                self.r_SP = self.r_X;
            }
            OperationImplied::TAX => {
                self.r_X = self.r_A;
                self.set_zn(self.r_X);
            }
            OperationImplied::TSX => {
                self.r_X = self.r_SP;
                self.set_zn(self.r_X);
            }
            _ => {
                return false;
            }
        }
        true
    }
    fn executed_branch(&mut self, opcode: u8) -> bool {
        if opcode & BRANCH_INSTRUCTION_MASK == BRANCH_INSTRUCTION_MASK_RESULT {
            let mut branch = (opcode & BRANCH_CONDITION_MASK) != 0;
            match opcode >> BRANCH_ON_FLAG_SHIFT {
                0 => {
                    branch = !(branch ^ self.f_N);
                }
                1 => {
                    branch = !(branch ^ self.f_V);
                }
                2 => {
                    branch = !(branch ^ self.f_C);
                }
                3 => {
                    branch = !(branch ^ self.f_Z);
                }
                _ => {
                    return false;
                }
            }
            if branch {
                let offset = self.main_bus.borrow().read(self.r_PC) as u16;
                self.r_PC += 1;
                self.m_skipCycles += 1;
                let new_pc = self.r_PC + offset;
                self.set_page_crossed(self.r_PC, new_pc, 2);
                self.r_PC = new_pc;
            } else {
                self.r_PC += 1;
            }
            true
        } else {
            false
        }
    }
    fn executed_type0(&mut self, opcode: u8) -> bool {
        if (opcode & INSTRUCTION_MODE_MASK) == 0x0 {
            let mut location;
            match AddrMode2::get((opcode & ADDR_MODE_MASK) >> ADDR_MODE_SHIFT) {
                AddrMode2::Immediate_ => {
                    location = self.r_PC;
                    self.r_PC += 1;
                }
                AddrMode2::ZeroPage_ => {
                    location = self.main_bus.borrow().read(self.r_PC) as u16;
                    self.r_PC += 1;
                }
                AddrMode2::Absolute_ => {
                    location = self.read_address(self.r_PC);
                    self.r_PC += 2;
                }
                AddrMode2::Indexed => {
                    location =
                        (self.main_bus.borrow().read(self.r_PC) as u16 + self.r_X as u16) & 0xff;
                    self.r_PC += 1;
                }
                AddrMode2::AbsoluteIndexed => {
                    location = self.read_address(self.r_PC);
                    self.r_PC += 2;
                    self.set_page_crossed(location, location + (self.r_X as u16), 1);
                    location += self.r_X as u16;
                }
                _ => {
                    return false;
                }
            }
            match Operation0::get((opcode & ADDR_MODE_MASK) >> ADDR_MODE_SHIFT) {
                Operation0::BIT => {
                    let operand = self.main_bus.borrow().read(location);
                    self.f_Z = (self.r_A & operand) == 0;
                    self.f_V = (operand & 0x40) != 0;
                    self.f_N = (operand & 0x80) != 0;
                }
                Operation0::STY => {
                    self.main_bus.borrow_mut().write(location, self.r_Y);
                }
                Operation0::LDY => {
                    self.r_Y = self.main_bus.borrow().read(location);
                    self.set_zn(self.r_Y);
                }
                Operation0::CPY => {
                    let diff = (self.r_Y - self.main_bus.borrow().read(location)) as u16;
                    self.f_C = (diff & 0x100) == 0;
                    self.set_zn(diff as u8);
                }
                Operation0::CPX => {
                    let diff = (self.r_X - self.main_bus.borrow().read(location)) as u16;
                    self.f_C = (diff & 0x100) == 0;
                    self.set_zn(diff as u8);
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
    fn executed_type1(&mut self, opcode: u8) -> bool {
        if (opcode & INSTRUCTION_MODE_MASK) == 0x1 {
            let mut location: Address = 0; // Location of the operand, could be in RAM
            let op = Operation1::get((opcode & OPERATION_MASK) >> OPERATION_SHIFT);
            match AddrMode1::get((opcode & ADDR_MODE_MASK) >> ADDR_MODE_SHIFT) {
                AddrMode1::IndexedIndirectX => {
                    let zero_addr = (self.r_X + self.main_bus.borrow().read(self.r_PC)) as u16;
                    self.r_PC += 1;
                    // Addresses wrap in zero page mode, thus pass through a mask
                    location = (self.main_bus.borrow().read(zero_addr & 0xff) as u16)
                        | (self.main_bus.borrow().read((zero_addr + 1) & 0xff) as u16) << 8;
                }
                AddrMode1::ZeroPage => {
                    location = self.main_bus.borrow().read(self.r_PC) as u16;
                    self.r_PC += 1;
                }
                AddrMode1::Immediate => {
                    location = self.r_PC;
                    self.r_PC += 1;
                }
                AddrMode1::Absolute => {
                    location = self.read_address(self.r_PC as u16);
                    self.r_PC += 2;
                }
                AddrMode1::IndirectY => {
                    let zero_addr = self.main_bus.borrow().read(self.r_PC) as u16;
                    self.r_PC += 1;
                    location = (self.main_bus.borrow().read(zero_addr & 0xff) as u16)
                        | (self.main_bus.borrow().read((zero_addr + 1) & 0xff) as u16) << 8;
                    if op != Operation1::STA {
                        self.set_page_crossed(location, location + self.r_Y as u16, 1);
                    }
                    location += self.r_Y as u16;
                }
                AddrMode1::IndexedX => {
                    //   // Address wraps around in the zero page
                    location = (self.main_bus.borrow().read(self.r_PC) + self.r_X) as u16 & 0xff;
                    self.r_PC += 1;
                }
                AddrMode1::AbsoluteY => {
                    location = self.read_address(self.r_PC);
                    self.r_PC += 2;
                    if op != Operation1::STA {
                        self.set_page_crossed(location, location + self.r_Y as u16, 1);
                    }
                    location += self.r_Y as u16;
                }
                AddrMode1::AbsoluteX => {
                    location = self.read_address(self.r_PC);
                    self.r_PC += 2;
                    if op != Operation1::STA {
                        self.set_page_crossed(location, location + self.r_X as u16, 1);
                    }
                    location += self.r_X as u16;
                }
                _ => {
                    return false;
                }
            }
            match op {
                Operation1::ORA => {
                    self.r_A |= self.main_bus.borrow().read(location);
                    self.set_zn(self.r_A);
                }
                Operation1::AND => {
                    self.r_A &= self.main_bus.borrow().read(location);
                    self.set_zn(self.r_A);
                }
                Operation1::EOR => {
                    self.r_A ^= self.main_bus.borrow().read(location);
                    self.set_zn(self.r_A);
                }
                Operation1::ADC => {
                    let operand = self.main_bus.borrow().read(location);
                    let sum: Address = self.r_A as u16 + (operand as u16) + bu16(self.f_C);
                    // Carry forward or UNSIGNED overflow
                    self.f_C = u16b(sum & 0x100);
                    // SIGNED overflow, would only happen if the sign of sum is
                    // different from BOTH the operands
                    self.f_V = u16b(((self.r_A as u16) ^ sum) & ((operand as u16) ^ sum) & 0x80);
                    self.r_A = sum as u8;
                    self.set_zn(self.r_A);
                }
                Operation1::STA => self.main_bus.borrow_mut().write(location, self.r_A),
                Operation1::LDA => {
                    self.r_A = self.main_bus.borrow().read(location);
                    self.set_zn(self.r_A);
                }
                Operation1::SBC => {
                    //   High carry means "no borrow", thus negate and subtract
                    let subtrahend: Address = self.main_bus.borrow().read(location) as u16;
                    let diff: Address = (self.r_A as u16) - subtrahend - bu16(!self.f_C);
                    // if the ninth bit is 1, the resulting number is negative => borrow =>
                    // low carry
                    self.f_C = !u16b(diff & 0x100);
                    // Same as ADC, except instead of the subtrahend,
                    // substitute with it's one complement
                    self.f_V = u16b(((self.r_A as u16) ^ diff) & (!subtrahend ^ diff) & 0x80);
                    self.r_A = diff as u8;
                    self.set_zn(self.r_A);
                }
                Operation1::CMP => {
                    let diff: Address = (self.r_A - self.main_bus.borrow().read(location)) as u16;
                    self.f_C = !u16b(diff & 0x100);
                    self.set_zn(diff as u8);
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
    fn executed_type2(&mut self, opcode: u8) -> bool {
        if (opcode & INSTRUCTION_MODE_MASK) == 2 {
            let mut location = 0;
            let op = Operation2::get((opcode & OPERATION_MASK) >> OPERATION_SHIFT);
            let addr_mode = AddrMode2::get((opcode & ADDR_MODE_MASK) >> ADDR_MODE_SHIFT);
            match addr_mode {
                AddrMode2::Immediate_ => {
                    location = self.r_PC;
                    self.r_PC += 1;
                }
                AddrMode2::ZeroPage_ => {
                    location = self.main_bus.borrow().read(self.r_PC) as u16;
                    self.r_PC += 1;
                }
                AddrMode2::Accumulator => {}
                AddrMode2::Absolute_ => {
                    location = self.read_address(self.r_PC);
                    self.r_PC += 2;
                }
                AddrMode2::Indexed => {
                    location = self.main_bus.borrow().read(self.r_PC) as u16;
                    self.r_PC += 1;
                    let index = if op == Operation2::LDX || op == Operation2::STX {
                        self.r_Y
                    } else {
                        self.r_X
                    } as u16; // The mask wraps address around zero page
                    location = (location + index) & 0xff;
                }
                AddrMode2::AbsoluteIndexed => {
                    location = self.read_address(self.r_PC);
                    self.r_PC += 2;
                    let index = if op == Operation2::LDX || op == Operation2::STX {
                        self.r_Y
                    } else {
                        self.r_X
                    } as u16;
                    self.set_page_crossed(location, location + index, 1);
                    location += index;
                }
                _ => {
                    return false;
                }
            }

            let mut operand = 0;
            match op {
                Operation2::ASL | Operation2::ROL => {
                    if addr_mode == AddrMode2::Accumulator {
                        let prev_C = self.f_C;
                        self.f_C = (self.r_A & 0x80) != 0;
                        self.r_A <<= 1;
                        // If Rotating, set the bit-0 to the the previous carry
                        self.r_A = self.r_A | bu8(prev_C && (op == Operation2::ROL));
                        self.set_zn(self.r_A);
                    } else {
                        let prev_C = self.f_C;
                        operand = self.main_bus.borrow().read(location);
                        self.f_C = (operand & 0x80) != 0;
                        operand = operand << 1 | bu8(prev_C && op == Operation2::ROL);
                        self.set_zn(operand);
                        self.main_bus.borrow_mut().write(location, operand);
                    }
                }
                Operation2::LSR | Operation2::ROR => {
                    if addr_mode == AddrMode2::Accumulator {
                        let prev_C = self.f_C;
                        self.f_C = (self.r_A & 1) != 0;
                        self.r_A >>= 1;
                        // If Rotating, set the bit-7 to the previous carry
                        self.r_A = self.r_A | bu8(prev_C && (op == Operation2::ROR)) << 7;
                        self.set_zn(self.r_A);
                    } else {
                        let prev_C = self.f_C;
                        operand = self.main_bus.borrow().read(location);
                        self.f_C = u8b(operand & 1);
                        operand = operand >> 1 | bu8(prev_C && (op == Operation2::ROR)) << 7;
                        self.set_zn(operand);
                        self.main_bus.borrow_mut().write(location, operand);
                    }
                }
                Operation2::STX => {
                    self.main_bus.borrow_mut().write(location, self.r_X);
                }
                Operation2::LDX => {
                    self.r_X = self.main_bus.borrow().read(location);
                    self.set_zn(self.r_X);
                }
                Operation2::DEC => {
                    let tmp = self.main_bus.borrow().read(location) - 1;
                    self.set_zn(tmp);
                    self.main_bus.borrow_mut().write(location, tmp);
                }
                Operation2::INC => {
                    let tmp = self.main_bus.borrow().read(location) + 1;
                    self.set_zn(tmp);
                    self.main_bus.borrow_mut().write(location, tmp);
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    fn interrupt(&mut self, t: InterruptType) {
        unimplemented!()
    }
    fn pull_stack(&mut self) -> Byte {
        self.r_SP += 1;
        self.main_bus.borrow().read(0x100 | (self.r_SP as u16))
    }
    fn push_stack(&mut self, value: Byte) {
        unimplemented!();
        self.r_SP -= 1;
    }
    fn set_zn(&mut self, value: Byte) {
        self.f_Z = value == 0;
        self.f_N = (value & 0x80) != 0;
    }

    fn set_page_crossed(&mut self, a: Address, b: Address, inc: i32) {
        if (a & 0xff00) != (b & 0xff00) {
            self.m_skipCycles += inc;
        }
    }
}

fn bu8(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

fn bu16(b: bool) -> u16 {
    if b {
        1
    } else {
        0
    }
}

fn u8b(x: u8) -> bool {
    x != 0
}
fn u16b(x: u16) -> bool {
    x != 0
}
