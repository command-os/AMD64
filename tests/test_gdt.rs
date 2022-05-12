//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#![deny(warnings, clippy::cargo, unused_extern_crates, rust_2021_compatibility)]

#[test]
fn code_segment_desc() {
    assert_eq!(
        amd64::sys::gdt::SegmentDescriptor::new_from_ty(
            amd64::sys::gdt::DescriptorType::CodeSegment
        )
        .attrs,
        amd64::sys::gdt::SegmentAttributes::new()
            .with_ty(amd64::sys::gdt::DescriptorType::CodeSegment)
            .with_present(true)
            .with_long(true)
    );
}

#[test]
fn data_segment_desc() {
    assert_eq!(
        amd64::sys::gdt::SegmentDescriptor::new_from_ty(
            amd64::sys::gdt::DescriptorType::DataSegment
        )
        .attrs,
        amd64::sys::gdt::SegmentAttributes::new()
            .with_ty(amd64::sys::gdt::DescriptorType::DataSegment)
            .with_present(true)
    );
}

#[test]
fn task_segment_desc() {
    assert_eq!(
        amd64::sys::gdt::SegmentDescriptor::new_from_ty(
            amd64::sys::gdt::DescriptorType::TaskSegment
        )
        .attrs,
        amd64::sys::gdt::SegmentAttributes::new()
            .with_ty(amd64::sys::gdt::DescriptorType::TaskSegment)
    );
}

#[test]
fn null_segment_desc() {
    assert_eq!(
        amd64::sys::gdt::SegmentDescriptor::default().attrs,
        amd64::sys::gdt::SegmentAttributes::new().with_present(true)
    );
}
