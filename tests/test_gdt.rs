//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#![deny(warnings, clippy::cargo, unused_extern_crates, rust_2021_compatibility)]

use amd64::cpu::gdt::{DescriptorType, SegmentAttributes, SegmentDescriptor};

#[test]
fn code_segment_desc() {
    assert_eq!(
        SegmentDescriptor::new_from_ty(DescriptorType::CodeSegment).attrs,
        SegmentAttributes::new()
            .with_ty(DescriptorType::CodeSegment)
            .with_present(true)
            .with_long(true)
    );
}

#[test]
fn data_segment_desc() {
    assert_eq!(
        SegmentDescriptor::new_from_ty(DescriptorType::DataSegment).attrs,
        SegmentAttributes::new()
            .with_ty(DescriptorType::DataSegment)
            .with_present(true)
    );
}

#[test]
fn task_segment_desc() {
    assert_eq!(
        SegmentDescriptor::new_from_ty(DescriptorType::TaskSegment).attrs,
        SegmentAttributes::new().with_ty(DescriptorType::TaskSegment)
    );
}

#[test]
fn null_segment_desc() {
    assert_eq!(
        SegmentDescriptor::default().attrs,
        SegmentAttributes::new().with_present(true)
    );
}
