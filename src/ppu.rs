use std::cell::RefCell;
use std::rc::Rc;

use crate::picture_bus::PictureBus;

type Byte = u8;
type Address = u16;

enum State {
    PreRender,
    Render,
    PostRender,
    VerticalBlank,
}

enum CharacterPage {
    Low,
    High,
}

pub struct PPU {
    picture_bus: Rc<RefCell<PictureBus>>,
    m_spriteMemory: [u8; 64 * 4],
    m_scanlineSprites: Vec<Byte>,

    m_pipelineState: State,

    m_cycle: i32,
    m_scanline: i32,
    m_evenFrame: bool,

    m_vblank: bool,
    m_sprZeroHit: bool,

    // Registers
    m_dataAddress: Address,
    m_tempAddress: Address,
    m_fineXScroll: Byte,
    m_firstWrite: bool,
    m_dataBuffer: Byte,

    m_spriteDataAddress: Byte,

    // Setup flags and variables
    m_longSprites: bool,
    m_generateInterrupt: bool,

    m_greyscaleMode: bool,
    m_showSprites: bool,
    m_showBackground: bool,
    m_hideEdgeSprites: bool,
    m_hideEdgeBackground: bool,

    m_bgPage: CharacterPage,
    m_sprPage: CharacterPage,

    m_dataAddrIncrement: Address,
}
impl PPU {
    pub fn new(picture_bus: Rc<RefCell<PictureBus>>) -> Self {
        PPU {
            picture_bus: picture_bus,
            m_spriteMemory: [0; 64 * 4],
            m_scanlineSprites: vec![],

            m_pipelineState: State::PreRender,

            m_cycle: 0,
            m_scanline: 0,
            m_evenFrame: true,

            m_vblank: false,
            m_sprZeroHit: false,

            // Registers
            m_dataAddress: 0,
            m_tempAddress: 0,
            m_fineXScroll: 0,
            m_firstWrite: true,
            m_dataBuffer: 0,

            m_spriteDataAddress: 0,

            // Setup flags and variables
            m_longSprites: false,
            m_generateInterrupt: false,

            m_greyscaleMode: false,
            m_showSprites: true,
            m_showBackground: true,
            m_hideEdgeSprites: false,
            m_hideEdgeBackground: false,

            m_bgPage: CharacterPage::Low,
            m_sprPage: CharacterPage::Low,

            m_dataAddrIncrement: 1,
        }
    }
}
