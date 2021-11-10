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
