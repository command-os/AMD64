use modular_bitfield::prelude::*;

#[amd64_macros::msr(0x227)]
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
