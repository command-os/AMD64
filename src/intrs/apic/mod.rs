//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;
use num_enum::IntoPrimitive;

mod lvt;

pub struct LocalAPIC {
    addr: u64,
}

#[derive(Debug, IntoPrimitive)]
#[repr(u64)]
pub enum LocalAPICReg {
    ID = 0x20,
    Ver = 0x30,
    TaskPriority = 0x80,
    ArbitrationPriority = 0x90,
    ProcessorPriority = 0xA0,
    EndOfInterrupt = 0xB0,
    RemoteRead = 0xC0,
    LogicalDestination = 0xD0,
    DestinationFormat = 0xE0,
    SpuriousInterruptVector = 0xF0,
    InService = 0x100,
    TriggerMode = 0x180,
    InterruptRequest = 0x200,
    ErrorStatus = 0x280,
    LvtCorrectedMachineCheck = 0x2F0,
    InterruptCommand = 0x300,
    InterruptCommand2 = 0x310,
    LVTTimer = 0x320,
    LVTThermalSensor = 0x330,
    LVTPerfCounter = 0x340,
    LVTLint0 = 0x350,
    LVTLint1 = 0x360,
    LVTError = 0x370,
    TimerInitialCount = 0x380,
    TimerCurrentCount = 0x390,
    TimerDivideConfiguration = 0x3E0,
}

#[derive(Debug, BitfieldSpecifier, Default, Clone, Copy, PartialEq, Eq)]
#[bits = 3]
#[repr(u8)]
pub enum DeliveryMode {
    #[default]
    Fixed = 0b000,
    LowestPriority = 0b001,
    Smi = 0b010,
    Nmi = 0b100,
    Init = 0b101,
    StartuUp = 0b110,
    ExtInt = 0b111,
}

#[bitfield(bits = 32)]
#[derive(Debug, BitfieldSpecifier, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub struct ErrorStatus {
    pub send_checksum_err: bool,
    pub recv_checksum_err: bool,
    pub send_accept_err: bool,
    pub recv_accept_err: bool,
    pub redir_ipi: bool,
    pub send_illegal_vec: bool,
    pub recv_illegal_vec: bool,
    pub illegal_reg_addr: bool,
    #[skip]
    __: B24,
}

#[derive(Debug, BitfieldSpecifier, Default, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
#[repr(u8)]
pub enum IntCmdDestShorthand {
    #[default]
    None = 0b00,
    ToSelf,
    ToAllInclSelf,
    ToAllExclSelf,
}

#[bitfield(bits = 32)]
#[derive(Debug, BitfieldSpecifier, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub struct SpuriousIntrVector {
    pub vector: u8,
    pub apic_soft_enable: bool,
    pub focus_proc_check: bool,
    #[skip]
    __: B2,
    pub eoi_broadcast_suppress: bool,
    #[skip]
    __: B19,
}

#[bitfield(bits = 64)]
#[derive(Debug, BitfieldSpecifier, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub struct InterruptCommand {
    pub vector: u8,
    pub delivery_mode: DeliveryMode,
    pub logical_dest: bool,
    pub delivery_pending: bool,
    #[skip]
    __: bool,
    pub assert: bool,
    pub level_trigger: bool,
    #[skip]
    __: B2,
    pub dest_shorthand: IntCmdDestShorthand,
    #[skip]
    __: B36,
    pub dest: u8,
}

impl LocalAPIC {
    pub fn new(addr: usize) -> Self {
        Self { addr: addr as _ }
    }

    pub fn write_reg<T: Into<u64>, V: Into<u32>>(&self, reg: T, value: V) {
        unsafe {
            ((self.addr + reg.into()) as *mut u32).write_volatile(value.into());
        }
    }

    pub fn read_reg<T: Into<u64>, R: From<u32>>(&self, reg: T) -> R {
        unsafe { ((self.addr + reg.into()) as *const u32).read_volatile() }.into()
    }

    #[inline]
    pub fn send_eoi(&self) {
        self.write_reg(LocalAPICReg::EndOfInterrupt, 0u32);
    }

    #[inline]
    pub fn read_timer(&self) -> lvt::TimerLVT {
        self.read_reg(LocalAPICReg::LVTTimer)
    }

    #[inline]
    pub fn write_timer(&self, val: lvt::TimerLVT) {
        self.write_reg(LocalAPICReg::LVTTimer, val)
    }

    #[inline]
    pub fn read_lint(&self, lint1: bool) -> lvt::LocalVectorTable {
        self.read_reg(if lint1 {
            LocalAPICReg::LVTLint1
        } else {
            LocalAPICReg::LVTLint0
        })
    }

    #[inline]
    pub fn error(&self) -> ErrorStatus {
        self.read_reg(LocalAPICReg::ErrorStatus)
    }

    #[inline]
    pub fn read_icr(&self) -> InterruptCommand {
        (((self.read_reg::<_, u64>(LocalAPICReg::InterruptCommand2)) << 32)
            | (self.read_reg::<_, u64>(LocalAPICReg::InterruptCommand)))
        .into()
    }

    #[inline]
    pub fn write_icr(&self, val: InterruptCommand) {
        let val: u64 = val.into();
        let a = val as u32;
        let b = (val >> 32) as u32;
        self.write_reg(LocalAPICReg::InterruptCommand2, b);
        self.write_reg(LocalAPICReg::InterruptCommand, a);
    }

    #[inline]
    pub fn write_spurious_intr_vec(&self, val: SpuriousIntrVector) {
        self.write_reg(LocalAPICReg::SpuriousInterruptVector, val)
    }
}
