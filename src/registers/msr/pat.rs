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
    const MSR_NUM: u32 = 0x277;

    unsafe fn read() -> Self {
        Self::from_bytes(crate::instructions::rdmsr(Self::MSR_NUM).to_le_bytes())
    }

    unsafe fn write(&self) {
        crate::instructions::wrmsr(Self::MSR_NUM, u64::from_le_bytes(self.into_bytes()));
    }
}
