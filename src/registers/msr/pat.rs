/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(clippy::return_self_not_must_use, clippy::unnecessary_cast)]

use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
#[bits = 3]
#[repr(u8)]
pub enum PatEntry {
    #[default]
    Uncacheable = 0x0,
    WriteCombining = 0x1,
    WriteThrough = 0x4,
    WriteProtected = 0x5,
    WriteBack = 0x6,
    Uncached = 0x7,
}

#[bitfield(bits = 64)]
#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
#[repr(u64)]
pub struct Pat {
    pub pat0: PatEntry,
    #[skip]
    __: B5,
    pub pat1: PatEntry,
    #[skip]
    __: B5,
    pub pat2: PatEntry,
    #[skip]
    __: B5,
    pub pat3: PatEntry,
    #[skip]
    __: B5,
    pub pat4: PatEntry,
    #[skip]
    __: B5,
    pub pat5: PatEntry,
    #[skip]
    __: B5,
    pub pat6: PatEntry,
    #[skip]
    __: B5,
    pub pat7: PatEntry,
    #[skip]
    __: B5,
}

impl super::Msr for Pat {
    const MSR_NUM: u32 = 0x227;

    fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bytes(bytes)
    }

    fn into_bytes(self) -> [u8; 8] {
        Self::into_bytes(self)
    }
}
