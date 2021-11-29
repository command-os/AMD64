/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use modular_bitfield::prelude::*;

#[bitfield(bits = 64)]
#[repr(u64)]
#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
pub struct Pat {
    pub pa0: B3,
    #[skip]
    __: B5,
    pub pa1: B3,
    #[skip]
    __: B5,
    pub pa2: B3,
    #[skip]
    __: B5,
    pub pa3: B3,
    #[skip]
    __: B5,
    pub pa4: B3,
    #[skip]
    __: B5,
    pub pa5: B3,
    #[skip]
    __: B5,
    pub pa6: B3,
    #[skip]
    __: B5,
    pub pa7: B3,
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
