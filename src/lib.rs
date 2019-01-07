//! Low level access to Cortex-A processors
//!
//! This crate provides:
//!
//! - Safe wrappers around assembly instructions
//!
//! For now, there's not much. I will update it gradually.
//! If you want to contribute, feel free to reach out!

#![no_std]
#![feature(asm)]
#![feature(core_intrinsics)]

#[macro_use]
pub mod asm;

pub mod barrier;
pub mod regs;

// TODO: this should be in asm but can't seem to resolve it when nested
#[macro_export]
macro_rules! svc {
    ( $syndrome:expr ) => {
        match () {
            #[cfg(target_arch = "aarch64")]
            () => unsafe {
                asm!(concat!("svc ", stringify!($syndrome)) :::: "volatile");
            },

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    };
}
