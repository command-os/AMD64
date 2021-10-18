use modular_bitfield::prelude::*;

#[amd64_macros::msr(0xC001_0114)]
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
