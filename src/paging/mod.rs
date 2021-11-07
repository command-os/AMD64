/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(clippy::must_use_candidate, clippy::map_unwrap_or)]

use bit::BitIndex;
use modular_bitfield::prelude::*;
pub use pml4::*;

use crate::alloc::boxed::Box;

mod pml4;

pub const PHYS_VIRT_OFFSET: u64 = 0xFFFF_8000_0000_0000;
pub const KERNEL_VIRT_OFFSET: u64 = 0xFFFF_FFFF_8000_0000;

#[derive(Debug)]
struct PageTableOffsets {
    pub pml4: usize,
    pub pml3: usize,
    pub pml2: usize,
    pub pml1: usize,
}

impl PageTableOffsets {
    #[inline]
    pub fn new(virtual_address: u64) -> Self {
        Self {
            pml4: virtual_address.bit_range(39..48).try_into().unwrap(),
            pml3: virtual_address.bit_range(30..39).try_into().unwrap(),
            pml2: virtual_address.bit_range(21..30).try_into().unwrap(),
            pml1: virtual_address.bit_range(12..21).try_into().unwrap(),
        }
    }
}

#[bitfield(bits = 64)]
#[repr(u64)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PageTableEntry {
    pub present: bool,
    pub writable: bool,
    pub user: bool,
    pub wt: bool,
    pub no_cache: bool,
    #[skip(setters)]
    pub accessed: bool,
    #[skip(setters)]
    pub dirty: bool,
    pub huge: bool,
    pub global: bool,
    pub available_to_os: B3,
    pub address: B40,
    pub available_to_os2: B11,
    pub no_execute: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: [PageTableEntry::default(); 512],
        }
    }

    #[inline]
    pub fn get_or_alloc_entry(
        &mut self,
        offset: usize,
        flags: PageTableEntry,
    ) -> &'static mut Self {
        let entry = &mut self.entries[offset];

        if !entry.present() {
            let table = Box::new(Self::new());

            entry.set_address((Box::leak(table) as *mut _ as u64) >> 12);
            entry.set_present(flags.present());
            entry.set_writable(flags.writable());
            entry.set_user(flags.user());
            entry.set_wt(flags.wt());
            entry.set_no_cache(flags.no_cache());
            entry.set_huge(flags.huge());
            entry.set_global(flags.global());
            entry.set_no_execute(flags.no_execute());
        }

        unsafe { &mut *((entry.address() << 12) as *mut Self) }
    }
}
