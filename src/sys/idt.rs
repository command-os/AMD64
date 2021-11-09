#![allow(clippy::unnecessary_cast)]

use modular_bitfield::prelude::*;

#[derive(Debug, BitfieldSpecifier, Clone, Copy, PartialEq, Eq)]
#[bits = 4]
#[repr(u8)]
pub enum EntryType {
    InterruptGate = 0b1110,
    TrapGate = 0b1111,
}

#[bitfield(bits = 16)]
#[derive(Debug, BitfieldSpecifier, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct EntryFlags {
    pub ist: B3,
    #[skip]
    __: B5,
    pub ty: EntryType,
    #[skip]
    __: B1,
    pub dpl: B2,
    pub present: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pub off_low: u16,
    pub selector: u16,
    pub flags: EntryFlags,
    pub off_middle: u16,
    pub off_high: u32,
    _reserved: u32,
}

impl Entry {
    pub const fn new(
        func: u64,
        selector: u16,
        ist: u8,
        ty: EntryType,
        dpl: u8,
        present: bool,
    ) -> Self {
        Self {
            off_low: (func & 0xFFFF) as u16,
            selector,
            flags: EntryFlags::from_bytes([
                ist & 0x7,
                ty as u8 | ((dpl & 0x3) << 5) | ((present as u8) << 7),
            ]),
            off_middle: ((func >> 16) & 0xFFFF) as u16,
            off_high: ((func >> 32) & 0xFFFFFFFF) as u32,
            _reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Idtr {
    pub limit: u16,
    pub base: *const Entry,
}

impl Idtr {
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    pub unsafe fn load(&self) {
        asm!("lidt [{}]", in(reg) self);
    }
}
