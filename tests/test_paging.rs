/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![deny(warnings, clippy::cargo, unused_extern_crates, rust_2021_compatibility)]

use amd64::paging::pml4::Pml4 as Pml4Trait;

#[repr(transparent)]
#[derive(Debug)]
pub struct Pml4(amd64::paging::PageTable);

impl Pml4 {
    pub fn new() -> Self {
        Self(amd64::paging::PageTable {
            entries: [amd64::paging::PageTableEntry::default(); 512],
        })
    }
}

impl Pml4Trait for Pml4 {
    const VIRT_OFF: usize = 0;

    fn get_entry(&mut self, offset: usize) -> &mut amd64::paging::PageTableEntry {
        &mut self.0.entries[offset]
    }

    fn alloc_entry() -> usize {
        Box::leak(Box::new(amd64::paging::PageTable::new())) as *mut _ as usize
    }
}

#[test]
fn test_map_higher_half_phys() {
    unsafe {
        let mut pml4 = Box::new(Pml4::new());
        pml4.map_higher_half();

        for i in 0..2048 {
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
