#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod bus;

//pub mod interrupt;

pub mod step;

pub mod time;
