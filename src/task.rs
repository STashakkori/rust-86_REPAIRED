//! Helpers to program the task state segment.
//! See Intel 3a, Chapter 7

pub use crate::segmentation;
use core::arch::asm;

/// Returns the current value of the task register.
pub fn tr() -> segmentation::SegmentSelector {
    let segment: u16;
    //unsafe { llvm_asm!("str $0" : "=r" (segment) ) };
    unsafe { asm!("str {}", out(reg) segment); }
    segmentation::SegmentSelector::from_raw(segment)
}

/// Loads the task register.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn load_tr(sel: segmentation::SegmentSelector) {
    //llvm_asm!("ltr $0" :: "r" (sel.bits()));
    let sel_bits: u16 = sel.bits();
    asm!("ltr [{}]", in(reg) &sel_bits, options(nostack, preserves_flags));
}
