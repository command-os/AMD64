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

    unsafe fn read() -> Self {
        Self::from_bytes(crate::instructions::rdmsr(Self::MSR_NUM).to_le_bytes())
    }

    unsafe fn write(&self) {
        crate::instructions::wrmsr(Self::MSR_NUM, u64::from_le_bytes(self.into_bytes()));
    }
}
