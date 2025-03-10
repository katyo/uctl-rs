#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod aliases;
mod arithmetic;
mod cast;
mod cast_fixed;
mod comparison;
mod fixed;
mod format;
mod from_number;
mod hashing;
mod into_number;
mod positive;
mod radix;
mod try_mul;
mod types;
mod unsigned_pow;

//#[cfg(feature = "bitcode")]
//mod bitcode_impl;

#[cfg(feature = "serde")]
mod serde_impl;

pub use aliases::*;
pub use cast::{Cast, TryCast};
pub use fixed::Fix;
pub use positive::{FromPositive, Positive};
pub use radix::{Mantissa, Radix};
pub use try_mul::TryMul;
pub use types::{Digits, Exponent};
pub use unsigned_pow::UnsignedPow;

/// Fixed-point number error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Number is too small to convert safely
    #[error("Number is too small to convert")]
    TooSmall,
    /// Number is too big to convert safely
    #[error("Number is too big to convert")]
    TooBig,
}

/// Fixed-point number result
pub type Result<T> = core::result::Result<T, Error>;
