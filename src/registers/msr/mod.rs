mod efer;
mod vm_cr;

pub use efer::*;
pub use vm_cr::*;

pub trait Msr {
    const MSR_NUM: u32;

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn read() -> Self;
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn write(&self);
}
