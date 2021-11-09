/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use modular_bitfield::prelude::*;

#[bitfield(bits = 64)]
#[repr(u64)]
#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
pub struct VmCr {
    pub disable_debug_port: bool,
    pub reserve_init: bool,
    pub disable_a20: bool,
    pub locked: bool,
    pub disabled: bool,
    #[skip]
    __: B59,
}

impl super::Msr for VmCr {
    const MSR_NUM: u32 = 0xC001_0114;

    fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bytes(bytes)
    }

    fn into_bytes(self) -> [u8; 8] {
        VmCr::into_bytes(self)
    }
}
