//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub struct CPUFeatures {
    // ECX
    #[skip(setters)]
    pub sse3: bool,
    #[skip(setters)]
    pub pclmulqdq: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub monitor: bool,
    #[skip]
    __: B5,
    #[skip(setters)]
    pub ssse3: bool,
    #[skip]
    __: B2,
    #[skip(setters)]
    pub fma: bool,
    #[skip(setters)]
    pub cmpxchg16b: bool,
    #[skip]
    __: B5,
    #[skip(setters)]
    pub sse41: bool,
    #[skip(setters)]
    pub sse42: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub movbe: bool,
    #[skip(setters)]
    pub popcnt: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub aes: bool,
    #[skip(setters)]
    pub xsave: bool,
    #[skip(setters)]
    pub osxsave: bool,
    #[skip(setters)]
    pub avx: bool,
    #[skip(setters)]
    pub f16c: bool,
    #[skip(setters)]
    pub rdrand: bool,
    #[skip(setters)]
    pub is_guest: bool,
    #[skip(setters)]
    pub fpu: bool,
    #[skip(setters)]
    pub vme: bool,
    #[skip(setters)]
    pub de: bool,
    #[skip(setters)]
    pub pse: bool,
    #[skip(setters)]
    pub tsc: bool,
    #[skip(setters)]
    pub msr: bool,
    #[skip(setters)]
    pub pae: bool,
    #[skip(setters)]
    pub mce: bool,
    #[skip(setters)]
    pub cmpxchg8b: bool,
    #[skip(setters)]
    pub apic: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub sysenter_sysexit: bool,
    #[skip(setters)]
    pub mtrr: bool,
    #[skip(setters)]
    pub pge: bool,
    #[skip(setters)]
    pub mca: bool,
    #[skip(setters)]
    pub cmov: bool,
    #[skip(setters)]
    pub pat: bool,
    #[skip(setters)]
    pub pse36: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub clfsh: bool,
    #[skip]
    __: B3,
    #[skip(setters)]
    pub mmx: bool,
    #[skip(setters)]
    pub fxsr: bool,
    #[skip(setters)]
    pub sse: bool,
    #[skip(setters)]
    pub sse2: bool,
    #[skip]
    __: bool,
    #[skip(setters)]
    pub htt: bool,
    #[skip]
    __: B3,
}
