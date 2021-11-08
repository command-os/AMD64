/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![no_std]
#![deny(warnings, clippy::cargo, unused_extern_crates, rust_2021_compatibility)]
#![feature(asm)]
#![feature(const_fn_trait_bound)]
#![feature(derive_default_enum)]

extern crate alloc;

pub mod io;
pub mod paging;
pub mod registers;
pub mod spec;
pub mod utilities;
