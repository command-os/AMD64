//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

pub mod features;

use arrayvec::ArrayString;
use modular_bitfield::prelude::*;

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub struct FeaturesMisc {
    #[skip(setters)]
    pub brand_id: u8,
    #[skip(setters)]
    pub clflush: u8,
    #[skip(setters)]
    pub proc_count: u8,
    #[skip(setters)]
    pub apic_id: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CPUIdentification {
    pub largest_func_id: u32,
    pub vendor_string: ArrayString<12>,
    pub features: features::CPUFeatures,
    pub misc: FeaturesMisc,
}

impl Default for CPUIdentification {
    fn default() -> Self {
        Self::new()
    }
}

impl CPUIdentification {
    pub fn new() -> Self {
        // Function 0
        let res = unsafe { core::arch::x86_64::__cpuid(0) };
        let mut s = [0u8; 12];
        s[..4].copy_from_slice(&res.ebx.to_le_bytes()[..]);
        s[4..8].copy_from_slice(&res.edx.to_le_bytes()[..]);
        s[8..12].copy_from_slice(&res.ecx.to_le_bytes()[..]);
        let largest_func_id = res.eax;
        let vendor_string = ArrayString::from_byte_string(&s).unwrap();

        // Function 1
        let res = unsafe { core::arch::x86_64::__cpuid(1) };
        let features = features::CPUFeatures::from(res.ecx as u64 | ((res.edx as u64) << 32));
        let misc = FeaturesMisc::from(res.ebx);

        Self {
            largest_func_id,
            vendor_string,
            features,
            misc,
        }
    }
}
