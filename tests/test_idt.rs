//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#![deny(warnings, clippy::cargo, unused_extern_crates, rust_2021_compatibility)]

use amd64::{
    cpu::{PrivilegeLevel, SegmentSelector},
    intrs::idt::{Entry, EntryFlags, EntryType},
};

#[test]
fn idt_interrupt_gate() {
    assert_eq!(
        Entry::new(
            0,
            SegmentSelector::new(0, PrivilegeLevel::Hypervisor),
            0,
            EntryType::InterruptGate,
            0,
            true
        )
        .flags,
        EntryFlags::new()
            .with_ist(0)
            .with_ty(EntryType::InterruptGate)
            .with_dpl(0)
            .with_present(true)
    );
}

#[test]
fn idt_interrupt_gate_user() {
    assert_eq!(
        Entry::new(
            0,
            SegmentSelector::new(0, PrivilegeLevel::User),
            0,
            EntryType::InterruptGate,
            3,
            true
        )
        .flags,
        EntryFlags::new()
            .with_ist(0)
            .with_ty(EntryType::InterruptGate)
            .with_dpl(3)
            .with_present(true)
    );
}
