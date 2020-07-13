use typenum::{Integer, NonZero, PInt, Unsigned};

/// The marker trait for exponent type parameters
///
/// Any signed integer number supported by __typenum__ can be used as static exponent value.
///
/// An alias for [typenum::Integer](../typenum/marker_traits/traits.Integer.html).
pub trait Exponent: Integer {}

impl<I: Integer> Exponent for I {}

/// The marker trait for mantissa digits type parameter
///
/// A signed number from `P1` and up to `P64` (or `P128` when _i128_ feature is enabled) can be used as mantissa width in digits.
pub trait Digits: Integer {}

impl<U: Unsigned + NonZero> Digits for PInt<U> {}
