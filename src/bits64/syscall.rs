use core::arch::asm;

#[macro_export]
macro_rules! syscall {
    ($arg0:expr) => {
        x86::bits64::syscall::syscall0($arg0 as u64)
    };

    ($arg0:expr, $arg1:expr) => {
        x86::bits64::syscall::syscall1($arg0 as u64, $arg1 as u64)
    };

    ($arg0:expr, $arg1:expr, $arg2:expr) => {
        x86::bits64::syscall::syscall2($arg0 as u64, $arg1 as u64, $arg2 as u64)
    };

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        x86::bits64::syscall::syscall3($arg0 as u64, $arg1 as u64, $arg2 as u64, $arg3 as u64)
    };

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        x86::bits64::syscall::syscall4(
            $arg0 as u64,
            $arg1 as u64,
            $arg2 as u64,
            $arg3 as u64,
            $arg4 as u64,
        )
    };

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        x86::bits64::syscall::syscall5(
            $arg0 as u64,
            $arg1 as u64,
            $arg2 as u64,
            $arg3 as u64,
            $arg4 as u64,
            $arg5 as u64,
        )
    };

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        x86::bits64::syscall::syscall6(
            $arg0 as u64,
            $arg1 as u64,
            $arg2 as u64,
            $arg3 as u64,
            $arg4 as u64,
            $arg5 as u64,
            $arg6 as u64,
        )
    };

    (
        $arg0:expr,
        $arg1:expr,
        $arg2:expr,
        $arg3:expr,
        $arg4:expr,
        $arg5:expr,
        $arg6:expr,
        $arg7:expr
    ) => {
        x86::bits64::syscall::syscall7(
            $arg0 as u64,
            $arg1 as u64,
            $arg2 as u64,
            $arg3 as u64,
            $arg4 as u64,
            $arg5 as u64,
            $arg6 as u64,
            $arg7 as u64,
        )
    };
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall0(arg0: u64) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0) : "rcx", "r11", "memory" : "volatile");
    asm!("syscall",in("rax") arg0,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall1(arg0: u64, arg1: u64) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1)
    //               : "rcx", "r11", "memory" : "volatile");
    asm!("syscall",in("rax") arg0,in("rdi") arg1,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall2(arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2)
    //               : "rcx", "r11", "memory" : "volatile");
    asm!("syscall",in("rax") arg0,in("rdi") arg1,in("rsi") arg2,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall3(arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3)
    //               : "rcx", "r11", "memory" : "volatile");
    asm!("syscall",in("rax") arg0,in("rdi") arg1,in("rsi") arg2,in("rdx") arg3,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall4(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret)
    //               : "{rax}"  (arg0), "{rdi}"  (arg1), "{rsi}"  (arg2), "{rdx}"  (arg3), "{r10}"  (arg4)
    //               : "rcx", "r11", "memory" : "volatile");
    asm!("syscall",in("rax") arg0,in("rdi") arg1,in("rsi") arg2,in("rdx") arg3,in("r10") arg4,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall5(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret)
    //               : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3), "{r10}" (arg4), "{r8}" (arg5)
    //               : "rcx", "r11", "memory"
    //               : "volatile");
    asm!("syscall",in("rax") arg0,in("rdi") arg1,in("rsi") arg2,in("rdx") arg3,in("r10") arg4,in("r8") arg5,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}

/// Invoke a syscall.
///
/// # Safety
/// Throws `#UD` if IA32_EFER.SCE = 0.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall6(
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> u64 {
    let mut ret: u64;
    //llvm_asm!("syscall" : "={rax}" (ret)
    //               : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3),
    //                 "{r10}" (arg4), "{r8}" (arg5), "{r9}" (arg6)
    //               : "rcx", "r11", "memory"
    //               : "volatile");
    asm!("syscall",in("rax") arg0,in("rdi") arg1,in("rsi") arg2,in("rdx") arg3,in("r10") arg4,in("r8") arg5,in("r9") arg6,lateout("rax") ret,lateout("rcx") _,lateout("r11") _,options(nostack, preserves_flags));
    ret
}
