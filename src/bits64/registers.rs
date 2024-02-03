use core::arch::asm;

/// Read the RIP register (instruction pointer).
#[inline(always)]
pub fn rip() -> u64 {
    let rip: u64;
    unsafe {
        asm!("lea 0(%rip), {}", out(reg) rip);
    }
    rip
}

/// Read the RSP register (stack pointer register).
#[inline(always)]
pub fn rsp() -> u64 {
    let rsp: u64;
    unsafe {
        asm!("mov %rsp, {}", out(reg) rsp);
    }
    rsp
}

/// Read the RBP register (base pointer register).
#[inline(always)]
pub fn rbp() -> u64 {
    let rbp: u64;
    unsafe {
        asm!("mov %rbp, {}", out(reg) rbp);
    }
    rbp
}