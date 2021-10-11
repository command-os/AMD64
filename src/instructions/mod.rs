/// # Safety
/// The caller must ensure that this operation has no unsafe side effects.
#[must_use]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let (low, high): (u32, u32);
    asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
    ((u64::from(high)) << 32) | (u64::from(low))
}

/// # Safety
/// The caller must ensure that this operation has no unsafe side effects.
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn wrmsr(msr: u32, value: u64) {
    let (low, high): (u32, u32) = (value as u32, (value >> 32) as u32);
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high, options(nomem, nostack, preserves_flags));
}
