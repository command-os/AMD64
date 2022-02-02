/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(clippy::return_self_not_must_use, clippy::unnecessary_cast)]

use core::arch::asm;

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
    pub offset_low: u16,
    pub selector: super::cpu::SegmentSelector,
    pub flags: EntryFlags,
    pub offset_middle: u16,
    pub offset_high: u32,
    _reserved: u32,
}

impl Entry {
    pub const fn new(
        base: u64,
        selector: super::cpu::SegmentSelector,
        ist: u8,
        ty: EntryType,
        dpl: u8,
        present: bool,
    ) -> Self {
        Self {
            offset_low: base as u16,
            selector,
            flags: EntryFlags::from_bytes([
                ist & 0x7,
                ty as u8 | ((dpl & 0x3) << 5) | ((present as u8) << 7),
            ]),
            offset_middle: (base >> 16) as u16,
            offset_high: (base >> 32) as u32,
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

unsafe impl Sync for Idtr {}
