#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate libm;
pub mod math;
pub mod matrix;
pub mod vector;
pub mod geometry;
