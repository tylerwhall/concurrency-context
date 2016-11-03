#![no_std]
#![feature(const_fn)]

mod singlethread;
pub use singlethread::*;

pub unsafe trait STC {}

/// Marker struct that can be constructed at the start of a program, before any threads are
/// launched or in an OS before any concurrency is enabled. Implements STC (single-thread context).
pub struct Init(());
impl Init {
    pub unsafe fn new() -> Self {
        Init(())
    }
}

unsafe impl STC for Init {}
