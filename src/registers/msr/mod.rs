/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use core::arch::asm;

pub use efer::*;
pub use pat::*;
pub use vm_cr::*;

mod efer;
mod pat;
mod vm_cr;

pub trait Msr {
    const MSR_NUM: u32;

    fn from_bytes(bytes: [u8; 8]) -> Self;
    fn into_bytes(self) -> [u8; 8];

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn read() -> Self
    where
        Self: Sized,
    {
        let (low, high): (u32, u32);
        asm!("rdmsr", in("ecx") Self::MSR_NUM, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
        Self::from_bytes(((u64::from(high) << 32) | u64::from(low)).to_le_bytes())
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn write(&self)
    where
        Self: Sized + Copy,
    {
        let value = u64::from_le_bytes(Self::into_bytes(*self));
        let (low, high): (u32, u32) = (value as u32, (value >> 32) as u32);
        asm!("wrmsr", in("ecx") Self::MSR_NUM, in("eax") low, in("edx") high, options(nomem, nostack, preserves_flags));
    }
}
