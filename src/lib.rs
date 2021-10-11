#![no_std]
#![deny(
    warnings,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![feature(asm)]
#![feature(const_fn_trait_bound)]

extern crate alloc;

pub mod io;
pub mod paging;
pub mod registers;
pub mod spec;
pub mod instructions;
