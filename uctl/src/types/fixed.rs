//! Fixed-point number types.
//!
//! # What?
//!
//! Fixed-point is a number representation with a fixed number of digits before and after the radix
//! point. This means that range is static rather than dynamic, as with floating-point. It also
//! means that they can be represented as integers, with their scale tracked by the type system.
//!
//! In this library, the scale of a `Fix` is represented as two type-level integers: the base and
//! the exponent. Any underlying integer primitive can be used to store the number. Arithmetic can
//! be performed on these numbers, and they can be converted to different scale exponents.
//!
//! # Why?
//!
//! A classic example: let's sum 10 cents and 20 cents using floating-point. We expect a result of
//! 30 cents.
//!
//! ```should_panic
//! assert_eq!(0.30, 0.10 + 0.20);
//! ```
//!
//! Wrong! We get an extra forty quintillionths of a dollar.
//!
//! ```text
//! assertion failed: `(left == right)` (left: `0.3`, right: `0.30000000000000004`)'
//! ```
//!
//! This is due to neither 0.1 nor 0.2 being exactly representable in base-2, just as a third can't
//! be represented exactly in base-10. With `Fix`, we can choose the precision we want in base-10,
//! at compile-time. In this case, hundredths of a dollar will do.
//!
//! ```
//! use uctl::si::Centi; // Fix<_, U10, N2>
//! assert_eq!(Centi::new(0_30), Centi::new(0_10) + Centi::new(0_20));
//! ```
//!
//! But decimal is inefficient for binary computers, right? Multiplying and dividing by 10 is
//! slower than bit-shifting, but that's only needed when _moving_ the point. With `Fix`, this is
//! only done explicitly with the `convert` method.
//!
//! ```
//! use uctl::si::{Centi, Milli};
//! assert_eq!(Milli::new(0_300), Centi::new(0_30).convert());
//! ```
//!
//! We can also choose a base-2 scale just as easily.
//!
//! ```
//! use uctl::iec::{Kibi, Mebi};
//! assert_eq!(Kibi::new(1024), Mebi::new(1).convert());
//! ```
//!
//! It's also worth noting that the type-level scale changes when multiplying and dividing,
//! avoiding any implicit conversion.
//!
//! ```
//! use uctl::iec::{Gibi, Kibi, Mebi};
//! assert_eq!(Mebi::new(3), Gibi::new(6) / Kibi::new(2));
//! ```
//!
//! # `no_std`
//!
//! This crate is `no_std`.
//!
//! # `i128` support
//!
//! Support for `u128` and `i128` can be enabled on nightly Rust through the `i128` Cargo feature.

/// Type aliases.
mod aliases;
mod from_number;
mod from_unsigned;
mod unsigned_pow;
mod operators;

pub use self::aliases::*;
pub use self::from_unsigned::FromUnsigned;
pub use self::unsigned_pow::UnsignedPow as Pow;

use core::{
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
    ops::{Div, Mul, Sub},
};

use typenum::{
    Z0,
    Bit, Integer, Unsigned,
    AbsVal, Diff, Le, Abs, IsLess,
};

/// Fixed-point number representing _Bits × Base <sup>Exp</sup>_.
///
/// - `Bits` is an integer primitive type, or any type which can be created from a type-level
///   integer and exponentiated.
/// - `Base` is an [`Unsigned`] type-level integer.
/// - `Exp` is a signed type-level [`Integer`].
///
/// [`Unsigned`]: ../typenum/marker_traits/trait.Unsigned.html
/// [`Integer`]: ../typenum/marker_traits/trait.Integer.html
///
/// # Summary of operations
///
/// Lower case variables represent values of _Bits_. Upper case _B_ and _E_ represent type-level
/// integers _Base_ and _Exp_, respectively.
///
/// - _−(x B<sup>E</sup>) = (−x) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) + (y B<sup>E</sup>) = (x + y) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) − (y B<sup>E</sup>) = (x − y) B<sup>E</sup>_
/// - _(x B<sup>E<sub>x</sub></sup>) × (y B<sup>E<sub>y</sub></sup>) =
///   (x × y) B<sup>E<sub>x</sub> + E<sub>y</sub></sup>_
/// - _(x B<sup>E<sub>x</sub></sup>) ÷ (y B<sup>E<sub>y</sub></sup>) =
///   (x ÷ y) B<sup>E<sub>x</sub> − E<sub>y</sub></sup>_
/// - _(x B<sup>E<sub>x</sub></sup>) % (y B<sup>E<sub>y</sub></sup>) =
///   (x % y) B<sup>E<sub>x</sub></sup>_
/// - _(x B<sup>E</sup>) × y = (x × y) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) ÷ y = (x ÷ y) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) % y = (x % y) B<sup>E</sup>_
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fix<Bits, Base, Exp> {
    /// The underlying integer.
    pub bits: Bits,

    marker: PhantomData<(Base, Exp)>,
}

impl<Bits, Base, Exp> Fix<Bits, Base, Exp> {
    /// Creates a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use uctl::si::{Kilo, Milli};
    /// Milli::new(25); // 0.025
    /// Kilo::new(25); // 25 000
    /// ```
    pub fn new(bits: Bits) -> Self {
        Fix { bits, marker: PhantomData }
    }

    /// Converts to another _Exp_.
    ///
    /// # Examples
    ///
    /// ```
    /// use uctl::si::{Kilo, Milli};
    /// let kilo = Kilo::new(5);
    /// let milli = Milli::new(5_000_000);
    /// assert_eq!(kilo, milli.convert());
    /// assert_eq!(milli, kilo.convert());
    /// ```
    pub fn convert<ToExp>(self) -> Fix<Bits, Base, ToExp>
    where
        Bits: FromUnsigned + Pow + Mul<Output = Bits> + Div<Output = Bits>,
        Base: Unsigned,
        Exp: Sub<ToExp>,
        Diff<Exp, ToExp>: Abs + IsLess<Z0>,
        AbsVal<Diff<Exp, ToExp>>: Integer
    {
        let base = Bits::from_unsigned::<Base>();
        let diff = AbsVal::<Diff<Exp, ToExp>>::to_i32();
        let inverse = Le::<Diff<Exp, ToExp>, Z0>::to_bool();

        // FIXME: Would like to do this with typenum::Pow, but that
        // seems to result in overflow evaluating requirements.
        let ratio = base.pow(diff as u32);

        if inverse {
            Fix::new(self.bits / ratio)
        } else {
            Fix::new(self.bits * ratio)
        }
    }
}

// The usual traits.

impl<Bits, Base, Exp> Debug for Fix<Bits, Base, Exp>
where Bits: Debug, Base: Unsigned, Exp: Integer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}x{}^{}", self.bits, Base::to_u64(), Exp::to_i64())
    }
}
