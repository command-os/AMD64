/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(clippy::unnecessary_cast)]

use modular_bitfield::prelude::*;

#[derive(Debug, Default, BitfieldSpecifier, Clone, Copy, PartialEq, Eq)]
#[bits = 5]
#[repr(u8)]
pub enum DescriptorType {
    CodeSegment = 0b11010,
    DataSegment = 0b10010,
    TaskSegment = 0b01001,
    #[default]
    None = 0b0,
}

#[bitfield(bits = 16)]
#[derive(Debug, Default, BitfieldSpecifier, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct SegmentAttributes {
    pub ty: DescriptorType,
    pub dpl: B2,
    pub present: bool,
    pub limit_high: B4,
    pub avl: B1,
    pub long: bool,
    pub default_op_size: bool,
    pub granularity: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct SegmentDescriptor {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub attrs: SegmentAttributes,
    pub base_high: u8,
}

impl SegmentDescriptor {
    pub const fn default() -> Self {
        Self::new(0, DescriptorType::None, true, false)
    }

    pub const fn new(limit_low: u16, ty: DescriptorType, present: bool, long: bool) -> Self {
        Self {
            limit_low,
            base_low: 0,
            base_middle: 0,
            attrs: SegmentAttributes::from_bytes([
                ty as u8 | ((present as u8) << 7),
                (long as u8) << 5,
            ]),
            base_high: 0,
        }
    }

    pub const fn new_from_ty(ty: DescriptorType) -> Self {
        match ty {
            DescriptorType::CodeSegment => Self::new(0, ty, true, true),
            DescriptorType::TaskSegment => Self::new(104, ty, false, false),
            _ => Self::new(0, ty, true, false),
        }
    }
}

#[repr(C, packed)]
pub struct Gdtr {
    pub limit: u16,
    pub addr: *const SegmentDescriptor,
}

unsafe impl Sync for Gdtr {}

impl Gdtr {
    /// # Safety
    /// The caller must ensure the safety of this operation.
    pub unsafe fn load(&self) {
        asm!(
            "lgdt [{}]",
            "push rbp",
            "mov rbp, rsp",
            "push {}",
            "push rbp",
            "pushfq",
            "push {}",
            "push 2f",
            "iretq",
            "2:",
            "pop rbp",
            "mov ds, {1}",
            "mov es, {1}",
            "mov fs, {1}",
            "mov gs, {1}",
            "mov ss, {1}",
            in(reg) self,
            in(reg) 2u64 << 3u64,
            in(reg) 1u64 << 3u64,
        );
    }
}
