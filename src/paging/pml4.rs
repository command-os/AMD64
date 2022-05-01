//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use core::arch::asm;

pub trait Pml4: Sized {
    const VIRT_OFF: usize;

    fn get_entry(&mut self, offset: usize) -> &mut super::PageTableEntry;
    fn alloc_entry() -> usize;

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn set(&mut self) {
        asm!("mov cr3, {}", in(reg) self as *mut _ as usize - Self::VIRT_OFF, options(nostack, preserves_flags));
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn get() -> &'static mut Self {
        let pml4: *mut Self;
        asm!("mov {}, cr3", out(reg) pml4, options(nostack, preserves_flags));
        pml4.as_mut().unwrap()
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn get_or_alloc_entry(
        &mut self,
        offset: usize,
        flags: super::PageTableEntry,
    ) -> &mut Self {
        let entry = self.get_entry(offset);

        if !entry.present() {
            *entry = flags.with_address((Self::alloc_entry() >> 12) as u64);
        }

        (((entry.address() << 12) as usize + Self::VIRT_OFF) as *mut Self)
            .as_mut()
            .unwrap()
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn get_or_null_entry(&mut self, offset: usize) -> Option<&mut Self> {
        let entry = self.get_entry(offset);

        if entry.present() {
            Some(
                (((entry.address() << 12) as usize + Self::VIRT_OFF) as *mut Self)
                    .as_mut()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn virt_to_phys(&mut self, virt: usize) -> Option<usize> {
        let offs = super::PageTableOffsets::new(virt);
        let pdp = self.get_or_null_entry(offs.pml4)?;
        let pd = pdp.get_or_null_entry(offs.pdp)?;

        if pd.get_entry(offs.pd).huge_or_pat() {
            Some((pd.get_entry(offs.pd).address() << 12) as usize)
        } else {
            let pt = pd.get_or_null_entry(offs.pd)?;

            if pt.get_entry(offs.pt).present() {
                Some((pt.get_entry(offs.pt).address() << 12) as usize)
            } else {
                None
            }
        }
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn map_pages(
        &mut self,
        virt: usize,
        phys: usize,
        count: usize,
        flags: super::PageTableEntry,
    ) {
        for i in 0..count {
            let physical_address = phys + 0x1000 * i;
            let virtual_address = virt + 0x1000 * i;
            let offs = super::PageTableOffsets::new(virtual_address);
            let pdp = self.get_or_alloc_entry(offs.pml4, flags);
            let pd = pdp.get_or_alloc_entry(offs.pdp, flags);
            let pt = pd.get_or_alloc_entry(offs.pd, flags);
            *pt.get_entry(offs.pt) = flags
                .with_present(true)
                .with_address((physical_address >> 12) as u64);
        }
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn map_huge_pages(
        &mut self,
        virt: usize,
        phys: usize,
        count: usize,
        flags: super::PageTableEntry,
    ) {
        for i in 0..count {
            let physical_address = phys + 0x20_0000 * i;
            let virtual_address = virt + 0x20_0000 * i;
            let offs = super::PageTableOffsets::new(virtual_address);
            let pdp = self.get_or_alloc_entry(offs.pml4, flags);
            let pd = pdp.get_or_alloc_entry(offs.pdp, flags);
            *pd.get_entry(offs.pd) = flags
                .with_present(true)
                .with_huge_or_pat(true)
                .with_address((physical_address >> 12) as u64);
        }
    }

    /// # Safety
    /// The caller must ensure that this operation has no unsafe side effects.
    #[inline]
    unsafe fn map_higher_half(&mut self) {
        self.map_huge_pages(
            super::PHYS_VIRT_OFFSET,
            0,
            2048,
            super::PageTableEntry::new()
                .with_present(true)
                .with_writable(true),
        );
        self.map_huge_pages(
            super::KERNEL_VIRT_OFFSET,
            0,
            1024,
            super::PageTableEntry::new()
                .with_present(true)
                .with_writable(true),
        );
    }
}
