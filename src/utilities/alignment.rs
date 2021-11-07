/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

pub trait Alignment {
    fn align_up(&self, alignment: usize) -> Self;

    fn align_down(&self, alignment: usize) -> Self;
}

macro_rules! impl_addr_align {
    ($ty:ty) => {
        impl Alignment for $ty {
            #[inline]
            fn align_up(&self, alignment: usize) -> Self {
                let mut ret = *self;

                if (ret as usize & (alignment - 1)) != 0 {
                    ret &= (!(alignment - 1)) as Self;
                    ret += alignment as Self;
                }

                ret
            }

            #[inline]
            fn align_down(&self, alignment: usize) -> Self {
                let mut ret = *self;

                if (ret as usize & (alignment - 1)) != 0 {
                    ret &= (!(alignment - 1)) as Self;
                }

                ret
            }
        }
    };
}

impl_addr_align!(u16);
impl_addr_align!(u32);
impl_addr_align!(u64);
impl_addr_align!(usize);
