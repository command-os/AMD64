#![allow(clippy::must_use_candidate, clippy::map_unwrap_or)]

use modular_bitfield::prelude::*;

#[bitfield(bits = 64)]
#[repr(C, u64)]
#[derive(Debug, Default, Clone, Copy)]
pub struct VmCr {
    pub disable_debug_port: bool,
    pub reserve_init: bool,
    pub disable_a20: bool,
    #[skip(setters)]
    pub locked: bool,
    pub disabled: bool,
    #[skip]
    reserved: B59,
}

impl super::Msr for VmCr {
    const MSR_NUM: u32 = 0xC001_0114;

    unsafe fn read() -> Self {
        Self::from_bytes(crate::instructions::rdmsr(Self::MSR_NUM).to_le_bytes())
    }

    unsafe fn write(&self) {
        crate::instructions::wrmsr(Self::MSR_NUM, u64::from_le_bytes(self.into_bytes()));
    }
}
