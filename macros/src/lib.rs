/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![no_std]
#![deny(warnings, clippy::cargo, unused_extern_crates)]
#![feature(asm)]

#[proc_macro_attribute]
pub fn msr(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    assert!(attr.len() == 1);
    let msr = match &attr[0] {
        syn::NestedMeta::Meta(_) => panic!("expected literal, got meta instead"),
        syn::NestedMeta::Lit(v) => v,
    };
    let item = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &item.ident;
    let expanded = quote::quote! {
        #item

        impl super::Msr for #name {
            const MSR_NUM: u32 = #msr;

            unsafe fn read() -> Self {
                let (low, high): (u32, u32);
                asm!("rdmsr", in("ecx") Self::MSR_NUM, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
                Self::from_bytes(((u64::from(high) << 32) | u64::from(low)).to_le_bytes())
            }

            unsafe fn write(&self) {
                let value = u64::from_le_bytes(self.into_bytes());
                let (low, high): (u32, u32) = (value as u32, (value >> 32) as u32);
                asm!("wrmsr", in("ecx") Self::MSR_NUM, in("eax") low, in("edx") high, options(nomem, nostack, preserves_flags));
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
