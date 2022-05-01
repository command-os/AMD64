//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::arch::asm;

pub trait PortInOut: Sized {
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn read(port: u16) -> Self;
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn write(port: u16, value: Self);
}

impl PortInOut for u8 {
    unsafe fn read(port: u16) -> Self {
        let ret: Self;
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    unsafe fn write(port: u16, value: Self) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortInOut for u16 {
    unsafe fn read(port: u16) -> Self {
        let ret: Self;
        asm!("in ax, dx", out("ax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    unsafe fn write(port: u16, value: Self) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortInOut for u32 {
    unsafe fn read(port: u16) -> Self {
        let ret: Self;
        asm!("in eax, dx", out("eax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    unsafe fn write(port: u16, value: Self) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

pub struct Port<T: PortInOut> {
    port: u16,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: PortInOut> Port<T> {
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            _phantom: core::marker::PhantomData,
        }
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[must_use]
    pub unsafe fn read(&self) -> T {
        T::read(self.port)
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    pub unsafe fn write(&self, value: T) {
        T::write(self.port, value);
    }
}
