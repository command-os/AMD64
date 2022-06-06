//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::arch::asm;

pub trait PortIO: Sized {
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn read(port: u16) -> Self;
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn write(port: u16, value: Self);
}

impl PortIO for u8 {
    #[inline]
    unsafe fn read(port: u16) -> Self {
        let ret: Self;
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    #[inline]
    unsafe fn write(port: u16, value: Self) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortIO for u16 {
    #[inline]
    unsafe fn read(port: u16) -> Self {
        let ret: Self;
        asm!("in ax, dx", out("ax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    #[inline]
    unsafe fn write(port: u16, value: Self) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortIO for u32 {
    #[inline]
    unsafe fn read(port: u16) -> Self {
        let ret: Self;
        asm!("in eax, dx", out("eax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    #[inline]
    unsafe fn write(port: u16, value: Self) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

pub struct Port<T: PortIO, R: From<T> + Into<T>> {
    port: u16,
    __: core::marker::PhantomData<T>,
    ___: core::marker::PhantomData<R>,
}

impl<T: PortIO, R: From<T> + Into<T>> Port<T, R> {
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            __: core::marker::PhantomData,
            ___: core::marker::PhantomData,
        }
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[must_use]
    #[inline]
    pub unsafe fn read(&self) -> R {
        T::read(self.port).into()
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[must_use]
    #[inline]
    pub unsafe fn read_off<A: Into<u16>>(&self, off: A) -> R {
        T::read(self.port + off.into()).into()
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    pub unsafe fn write(&self, value: R) {
        T::write(self.port, value.into());
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    pub unsafe fn write_off<A: Into<u16>>(&self, value: R, off: A) {
        T::write(self.port + off.into(), value.into());
    }
}
