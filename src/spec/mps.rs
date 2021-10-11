#![allow(clippy::must_use_candidate, clippy::map_unwrap_or)]

use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum Polarity {
    ConformToBusSpec = 0,
    ActiveHigh = 0b01,
    ActiveLow = 0b11,
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum TriggerMode {
    ConformToBusSpec = 0,
    EdgeTriggered = 0b01,
    LevelTriggered = 0b11,
}

#[bitfield(bits = 16)]
#[repr(C, u16)]
#[derive(Debug, Clone, Copy)]
pub struct Inti {
    #[skip(setters)]
    pub polarity: B2,
    #[skip(setters)]
    pub trigger_mode: B2,
    #[skip]
    reserved: B12,
}
