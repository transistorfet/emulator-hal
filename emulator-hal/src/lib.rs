#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod cpu;
pub mod bus;
//pub mod interrupt;

