#![no_std]
#![deny(warnings, clippy::cargo)]
#![feature(asm)]
#![feature(const_fn_trait_bound)]

extern crate alloc;

pub mod io;
pub mod paging;
pub mod registers;
pub mod spec;
