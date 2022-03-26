//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#![allow(clippy::return_self_not_must_use, clippy::unnecessary_cast)]

use modular_bitfield::prelude::*;

#[bitfield(bits = 64)]
#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
#[repr(u64)]
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
}
