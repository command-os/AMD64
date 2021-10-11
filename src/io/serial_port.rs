pub trait PortInOut {
    unsafe fn read(port: u16) -> Self;
    unsafe fn write(port: u16, value: Self);
}

impl PortInOut for u8 {
    unsafe fn read(port: u16) -> u8 {
        let ret: u8;
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    unsafe fn write(port: u16, value: u8) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortInOut for u16 {
    unsafe fn read(port: u16) -> u16 {
        let ret: u16;
        asm!("in ax, dx", out("ax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    unsafe fn write(port: u16, value: u16) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortInOut for u32 {
    unsafe fn read(port: u16) -> u32 {
        let ret: u32;
        asm!("in eax, dx", out("eax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
        ret
    }

    unsafe fn write(port: u16, value: u32) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

pub struct SerialPort<T: PortInOut> {
    port: u16,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: PortInOut> SerialPort<T> {
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            _phantom: core::marker::PhantomData,
        }
    }

    pub unsafe fn read(&self) -> T {
        T::read(self.port)
    }

    pub unsafe fn write(&self, value: T) {
        T::write(self.port, value);
    }
}
