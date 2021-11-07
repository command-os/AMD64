/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(
    clippy::must_use_candidate,
    clippy::map_unwrap_or,
    clippy::unnecessary_cast,
    clippy::cast_possible_truncation
)]

pub use efer::*;
pub use pat::*;
pub use vm_cr::*;

mod efer;
mod pat;
mod vm_cr;

pub trait Msr {
    const MSR_NUM: u32;

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn read() -> Self;
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn write(&self);
}
