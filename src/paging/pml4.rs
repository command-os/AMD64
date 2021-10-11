pub trait PML4 {
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn map_pages(&mut self, virt: u64, phys: u64, count: u64);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn map_huge_pages(&mut self, virt: u64, phys: u64, count: u64);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn map_higher_half(&mut self);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn set(&mut self);
    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    unsafe fn get() -> &'static mut super::PageTable;
}

impl PML4 for super::PageTable {
    #[inline]
    unsafe fn map_pages(&mut self, virt: u64, phys: u64, count: u64) {
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
}
