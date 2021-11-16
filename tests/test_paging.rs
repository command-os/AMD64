#![feature(asm)]

amd64::impl_pml4!(
    Box::leak(Box::new(amd64::paging::PageTable::new())) as *mut _ as usize,
    0usize
);

#[test]
fn test_map_higher_half_phys() {
    unsafe {
        let mut pml4 = Box::new(Pml4::new());
        pml4.map_higher_half();

        assert_eq!(pml4.virt_to_phys(amd64::paging::PHYS_VIRT_OFFSET), None);
        for i in 1..2048 {
            assert_eq!(
                pml4.virt_to_phys(amd64::paging::PHYS_VIRT_OFFSET + 0x20_0000 * i),
                Some(0x20_0000 * i)
            );
        }
    }
}

#[test]
fn test_map_higher_half_kern() {
    unsafe {
        let mut pml4 = Box::new(Pml4::new());
        pml4.map_higher_half();

        for i in 0..1024 {
            assert_eq!(
                pml4.virt_to_phys(amd64::paging::KERNEL_VIRT_OFFSET + 0x20_0000 * i),
                Some(0x20_0000 * i)
            );
        }
    }
}
