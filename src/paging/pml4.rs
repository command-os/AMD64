/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

pub trait Pml4 {
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn set(&mut self);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn get() -> &'static mut super::PageTable;
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn virt_to_phys(&self, virt: u64) -> Option<u64>;
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn map_pages(&mut self, virt: u64, phys: u64, count: u64);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn map_huge_pages(&mut self, virt: u64, phys: u64, count: u64);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn map_higher_half(&mut self);
}

impl Pml4 for super::PageTable {
    #[inline]
    unsafe fn set(&mut self) {
        asm!("mov cr3, {}", in(reg) self as *mut _, options(nostack, preserves_flags));
    }

    #[inline]
    unsafe fn get() -> &'static mut super::PageTable {
        let pml4: *mut Self;
        asm!("mov {}, cr3", out(reg) pml4, options(nomem, nostack, preserves_flags));
        &mut *pml4
    }

    #[inline]
    unsafe fn virt_to_phys(&self, virt: u64) -> Option<u64> {
        let offs = super::PageTableOffsets::new(virt);
        let pml3 = self.get_or_null_entry(offs.pml4)?;
        let pml2 = pml3.get_or_null_entry(offs.pml3)?;

        if pml2.entries[offs.pml2].huge() {
            Some(pml2.entries[offs.pml2].address() << 12)
        } else {
            let pml1 = pml2.get_or_null_entry(offs.pml2)?;

            if pml1.entries[offs.pml1].present() {
                Some(pml1.entries[offs.pml1].address() << 12)
            } else {
                None
            }
        }
    }

    #[inline]
    unsafe fn map_pages(&mut self, virt: u64, phys: u64, count: u64) {
        let flags = super::PageTableEntry::new()
            .with_present(true)
            .with_writable(true)
            .with_user(true);

        for i in 0..count {
            let physical_address = phys + 0x1000 * i;
            let virtual_address = virt + 0x1000 * i;
            let offs = super::PageTableOffsets::new(virtual_address);
            let pml3 = self.get_or_alloc_entry(offs.pml4, flags);
            let pml2 = pml3.get_or_alloc_entry(offs.pml3, flags);
            let pml1 = pml2.get_or_alloc_entry(offs.pml2, flags);
            pml1.entries[offs.pml1] = super::PageTableEntry::new()
                .with_present(true)
                .with_writable(true)
                .with_address(physical_address >> 12);
        }
    }

    #[inline]
    unsafe fn map_huge_pages(&mut self, virt: u64, phys: u64, count: u64) {
        let flags = super::PageTableEntry::new()
            .with_present(true)
            .with_writable(true)
            .with_user(true);

        for i in 0..count {
            let physical_address = phys + 0x20_0000 * i;
            let virtual_address = virt + 0x20_0000 * i;
            let offs = super::PageTableOffsets::new(virtual_address);
            let pml3 = self.get_or_alloc_entry(offs.pml4, flags);
            let pml2 = pml3.get_or_alloc_entry(offs.pml3, flags);
            pml2.entries[offs.pml2] = super::PageTableEntry::new()
                .with_present(true)
                .with_writable(true)
                .with_huge(true)
                .with_address(physical_address >> 12);
        }
    }

    #[inline]
    unsafe fn map_higher_half(&mut self) {
        self.map_huge_pages(super::PHYS_VIRT_OFFSET + 0x20_0000, 0, 2047);
        self.map_huge_pages(super::KERNEL_VIRT_OFFSET, 0, 1024);
    }
}
