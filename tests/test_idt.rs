/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![deny(warnings, clippy::cargo, unused_extern_crates, rust_2021_compatibility)]

#[test]
fn idt_interrupt_gate() {
    assert_eq!(
        amd64::sys::idt::Entry::new(
            0,
            amd64::sys::cpu::SegmentSelector::new(0, amd64::sys::cpu::PrivilegeLevel::Hypervisor),
            0,
            amd64::sys::idt::EntryType::InterruptGate,
            0,
            true
        )
        .flags,
        amd64::sys::idt::EntryFlags::new()
            .with_ist(0)
            .with_ty(amd64::sys::idt::EntryType::InterruptGate)
            .with_dpl(0)
            .with_present(true)
    );
}

#[test]
fn idt_interrupt_gate_user() {
    assert_eq!(
        amd64::sys::idt::Entry::new(
            0,
            amd64::sys::cpu::SegmentSelector::new(0, amd64::sys::cpu::PrivilegeLevel::User),
            0,
            amd64::sys::idt::EntryType::InterruptGate,
            3,
            true
        )
        .flags,
        amd64::sys::idt::EntryFlags::new()
            .with_ist(0)
            .with_ty(amd64::sys::idt::EntryType::InterruptGate)
            .with_dpl(3)
            .with_present(true)
    );
}
