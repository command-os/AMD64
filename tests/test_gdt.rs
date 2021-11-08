#[test]
fn code_segment_desc() {
    assert_eq!(
        amd64::registers::gdt::SegmentDescriptor::new_from_ty(
            amd64::registers::gdt::DescriptorType::CodeSegment
        )
        .attrs,
        amd64::registers::gdt::SegmentAttributes::new()
            .with_ty(amd64::registers::gdt::DescriptorType::CodeSegment)
            .with_present(true)
            .with_long(true)
    );
}

#[test]
fn data_segment_desc() {
    assert_eq!(
        amd64::registers::gdt::SegmentDescriptor::new_from_ty(
            amd64::registers::gdt::DescriptorType::DataSegment
        )
        .attrs,
        amd64::registers::gdt::SegmentAttributes::new()
            .with_ty(amd64::registers::gdt::DescriptorType::DataSegment)
            .with_present(true)
    );
}

#[test]
fn task_segment_desc() {
    assert_eq!(
        amd64::registers::gdt::SegmentDescriptor::new_from_ty(
            amd64::registers::gdt::DescriptorType::TaskSegment
        )
        .attrs,
        amd64::registers::gdt::SegmentAttributes::new()
            .with_ty(amd64::registers::gdt::DescriptorType::TaskSegment)
    );
}

#[test]
fn null_segment_desc() {
    assert_eq!(
        amd64::registers::gdt::SegmentDescriptor::default().attrs,
        amd64::registers::gdt::SegmentAttributes::new().with_present(true)
    );
}
