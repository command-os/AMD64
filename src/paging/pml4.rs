/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[macro_export]
macro_rules! impl_pml4 {
    ($expression:expr, $virt_off:expr) => {
        use core::arch::asm;

        #[repr(transparent)]
        #[derive(Debug)]
        pub struct Pml4(pub amd64::paging::PageTable);

        impl core::ops::Deref for Pml4 {
            type Target = amd64::paging::PageTable;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for Pml4 {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Pml4 {
            fn alloc_entry() -> usize {
                $expression
            }

            pub fn new() -> Self {
                Self(amd64::paging::PageTable::new())
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn set(&mut self) {
                asm!("mov cr3, {}", in(reg) self as *mut _ as usize - $virt_off, options(nostack, preserves_flags));
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn get() -> &'static mut Self {
                let pml4: *mut Self;
                asm!("mov {}, cr3", out(reg) pml4, options(nostack, preserves_flags));
                pml4.as_mut().unwrap()
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn get_or_alloc_entry(
                &mut self,
                offset: usize,
                flags: amd64::paging::PageTableEntry,
            ) -> &mut Self {
                let entry = &mut self.entries[offset];

                if !entry.present() {
                    entry.set_address((Self::alloc_entry() >> 12) as u64);
                    entry.set_present(flags.present());
                    entry.set_writable(flags.writable());
                    entry.set_user(flags.user());
                    entry.set_pat0(flags.pat0());
                    entry.set_pat1(flags.pat1());
                    entry.set_huge_or_pat2(flags.huge_or_pat2());
                    entry.set_global(flags.global());
                    entry.set_no_execute(flags.no_execute());
                }

                (((entry.address() << 12) as usize + $virt_off) as *mut Self).as_mut().unwrap()
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn get_or_null_entry(&self, offset: usize) -> Option<&mut Self> {
                let entry = &self.entries[offset];

                if !entry.present() {
                    None
                } else {
                    Some((((entry.address() << 12) as usize + $virt_off) as *mut Self).as_mut().unwrap())
                }
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn virt_to_phys(&self, virt: usize) -> Option<usize> {
                let offs = amd64::paging::PageTableOffsets::new(virt);
                let pdp = self.get_or_null_entry(offs.pml4)?;
                let pd = pdp.get_or_null_entry(offs.pdp)?;

                if pd.entries[offs.pd].huge_or_pat2() {
                    Some((pd.entries[offs.pd].address() << 12) as usize)
                } else {
                    let pt = pd.get_or_null_entry(offs.pd)?;

                    if pt.entries[offs.pt].present() {
                        Some((pt.entries[offs.pt].address() << 12) as usize)
                    } else {
                        None
                    }
                }
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn map_pages(&mut self, virt: usize, phys: usize, count: usize, flags: amd64::paging::PageTableEntry) {
                let flags = amd64::paging::PageTableEntry::new()
                    .with_present(true)
                    .with_writable(true)
                    .with_user(true);

                for i in 0..count {
                    let physical_address = phys + 0x1000 * i;
                    let virtual_address = virt + 0x1000 * i;
                    let offs = amd64::paging::PageTableOffsets::new(virtual_address);
                    let pdp = self.get_or_alloc_entry(offs.pml4, flags);
                    let pd = pdp.get_or_alloc_entry(offs.pdp, flags);
                    let pt = pd.get_or_alloc_entry(offs.pd, flags);
                    pt.entries[offs.pt] = flags
                        .with_present(true)
                        .with_address((physical_address >> 12) as u64);
                }
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn map_huge_pages(&mut self, virt: usize, phys: usize, count: usize, flags: amd64::paging::PageTableEntry) {
                let flags = amd64::paging::PageTableEntry::new()
                    .with_present(true)
                    .with_writable(true)
                    .with_user(true);

                for i in 0..count {
                    let physical_address = phys + 0x20_0000 * i;
                    let virtual_address = virt + 0x20_0000 * i;
                    let offs = amd64::paging::PageTableOffsets::new(virtual_address);
                    let pdp = self.get_or_alloc_entry(offs.pml4, flags);
                    let pd = pdp.get_or_alloc_entry(offs.pdp, flags);
                    pd.entries[offs.pd] = flags
                        .with_present(true)
                        .with_huge_or_pat2(true)
                        .with_address((physical_address >> 12) as u64);
                }
            }

            /// # Safety
            /// The caller must ensure that this operation has no unsafe side effects.
            #[inline]
            pub unsafe fn map_higher_half(&mut self) {
                self.map_huge_pages(
                    amd64::paging::PHYS_VIRT_OFFSET,
                    0,
                    2048,
                    amd64::paging::PageTableEntry::new()
                        .with_present(true)
                        .with_writable(true)
                );
                self.map_huge_pages(
                    amd64::paging::KERNEL_VIRT_OFFSET,
                    0,
                    1024,
                    amd64::paging::PageTableEntry::new()
                        .with_present(true)
                        .with_writable(true)
                );
            }
        }
    };
}
