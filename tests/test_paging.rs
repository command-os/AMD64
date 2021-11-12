use amd64::paging::Pml4;

#[test]
fn test_map_page() {
    unsafe {
        let mut pml4 = Box::new(amd64::paging::PageTable::new());
        pml4.map_pages(amd64::paging::PHYS_VIRT_OFFSET, 0x20_0000, 3);

        for i in 0..3 {
            assert_eq!(
                pml4.virt_to_phys(amd64::paging::PHYS_VIRT_OFFSET + 0x1000 * i),
                Some(0x20_0000 + 0x1000 * i)
            );
        }
    }
}

#[test]
fn test_map_huge_page() {
    unsafe {
        let mut pml4 = Box::new(amd64::paging::PageTable::new());
        pml4.map_huge_pages(amd64::paging::PHYS_VIRT_OFFSET, 0x20_0000, 3);

        for i in 0..3 {
            assert_eq!(
                pml4.virt_to_phys(amd64::paging::PHYS_VIRT_OFFSET + 0x20_0000 * i),
                Some(0x20_0000 + 0x20_0000 * i)
            );
        }
    }
}
