//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use modular_bitfield::prelude::*;

#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptEnable {
    pub data_available: bool,
    pub transmitter_empty: bool,
    pub break_or_error: bool,
    pub status_change: bool,
    #[skip]
    __: B4,
}

#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
#[bits = 2]
pub enum DataBits {
    FiveBits = 0b00,
    SixBits = 0b01,
    SevenBits = 0b10,
    EightBits = 0b11,
}

#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
#[bits = 1]
pub enum StopBits {
    OneBit = 0b0,
    OnePointFiveDividedBy2 = 0b1,
}

#[derive(Debug, Clone, Copy, BitfieldSpecifier)]
#[bits = 3]
pub enum Parity {
    None = 0b000,
    Odd = 0b001,
    Even = 0b011,
    Mark = 0b101,
    Space = 0b111,
}

#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub struct LineControl {
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
    #[skip]
    __: B1,
    pub dlab: bool,
}

#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub struct LineStatus {
    pub data_ready: bool,
    pub overrun_error: bool,
    pub parity_error: bool,
    pub framing_error: bool,
    pub break_indicator: bool,
    pub transmitter_empty: bool,
    pub transmitter_idle: bool,
    pub impending_error: bool,
}

#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub struct ModemControl {
    #[skip]
    __: B2,
    pub autoflow: bool,
    pub loopback: bool,
    pub aux_out_1: bool,
    pub aux_out_2: bool,
    pub req_send: bool,
    pub terminal_ready: bool,
}

#[allow(dead_code)]
pub struct SerialPort {
    data_or_divisor_low: super::port::Port<u8>,
    enable_intr_or_divisor_high: super::port::Port<u8>,
    intr_id_or_fifo: super::port::Port<u8>,
    line_ctl: super::port::Port<u8>,
    modem_ctl: super::port::Port<u8>,
    line_sts: super::port::Port<u8>,
}

impl SerialPort {
    pub const fn new(port_num: u16) -> Self {
        Self {
            data_or_divisor_low: super::port::Port::<u8>::new(port_num),
            enable_intr_or_divisor_high: super::port::Port::<u8>::new(port_num + 1),
            intr_id_or_fifo: super::port::Port::<u8>::new(port_num + 2),
            line_ctl: super::port::Port::<u8>::new(port_num + 3),
            modem_ctl: super::port::Port::<u8>::new(port_num + 4),
            line_sts: super::port::Port::<u8>::new(port_num + 5),
        }
    }

    fn can_send_data(&self) -> bool {
        LineStatus::from_bytes(unsafe { self.line_sts.read() }.to_le_bytes()).transmitter_empty()
    }

    pub fn init(&self) {
        unsafe {
            // Disable interrupts
            self.enable_intr_or_divisor_high.write(0);
            // Enable DLAB
            self.line_ctl.write(u8::from_le_bytes(
                LineControl::new().with_dlab(true).into_bytes(),
            ));
            // Set divisor to 1
            self.data_or_divisor_low.write(1);
            self.enable_intr_or_divisor_high.write(0);
            // 8 bits, no parity, one stop bit
            self.line_ctl.write(u8::from_le_bytes(
                LineControl::new()
                    .with_parity(Parity::None)
                    .with_data_bits(DataBits::EightBits)
                    .into_bytes(),
            ));
            // Disable FIFO
            self.intr_id_or_fifo.write(0);
            // Enable data terminal
            self.modem_ctl.write(u8::from_le_bytes(
                ModemControl::new()
                    .with_terminal_ready(true)
                    .with_aux_out_2(true)
                    .into_bytes(),
            ));
        }
    }

    pub fn transmit(&self, value: u8) {
        while !self.can_send_data() {}

        unsafe { self.data_or_divisor_low.write(value) }
    }

    fn can_receive_data(&self) -> bool {
        LineStatus::from_bytes(unsafe { self.line_sts.read() }.to_le_bytes()).data_ready()
    }

    pub fn receive(&self) -> u8 {
        while !self.can_receive_data() {}

        unsafe { self.data_or_divisor_low.read() }
    }
}
