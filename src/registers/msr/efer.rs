/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use modular_bitfield::prelude::*;

#[bitfield(bits = 64)]
#[repr(u64)]
#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
pub struct Efer {
    pub syscall_ext: bool,
    #[skip]
    __: B7,
    pub long_mode: bool,
    #[skip]
    __: B1,
    pub long_mode_active: bool,
    pub no_execute: bool,
    pub secure_virtual_machine: bool,
    pub long_mode_seg_limit: bool,
    pub fast_fxsave_fxrstor: bool,
    pub translation_cache_ext: bool,
    #[skip]
    __: B1,
    pub mcommit: bool,
    pub interruptible_wbinvd: bool,
    #[skip]
    __: B45,
}

impl super::Msr for Efer {
    const MSR_NUM: u32 = 0xC000_0080;

    fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bytes(bytes)
    }

    fn into_bytes(self) -> [u8; 8] {
        Efer::into_bytes(self)
    }
}
