/*!

Fixed-point number types.

# What?

Fixed-point is a number representation with a fixed number of digits before and after the radix
point. This means that range is static rather than dynamic, as with floating-point. It also
means that they can be represented as integers, with their scale tracked by the type system.

In this library, the scale of a `Fix` is represented as two type-level integers: the base and
the exponent. Any underlying integer primitive can be used to store the number. Arithmetic can
be performed on these numbers, and they can be converted to different scale exponents.

# Why?

A classic example: let's sum 10 cents and 20 cents using floating-point. We expect a result of
30 cents.

```should_panic
assert_eq!(0.30, 0.10 + 0.20);
```

Wrong! We get an extra forty quintillionths of a dollar.

```text
assertion failed: `(left == right)` (left: `0.3`, right: `0.30000000000000004`)'
```

This is due to neither 0.1 nor 0.2 being exactly representable in base-2, just as a third can't
be represented exactly in base-10. With `Fix`, we can choose the precision we want in base-10,
at compile-time. In this case, hundredths of a dollar will do.

```
use typenum::P3;
use ufix::si::Centi; // Fix<_, U10, N2>

assert_eq!(Centi::<P3>::new(0_30), Centi::<P3>::new(0_10) + Centi::<P3>::new(0_20));
```

But decimal is inefficient for binary computers, right? Multiplying and dividing by 10 is
slower than bit-shifting, but that's only needed when _moving_ the point. With `Fix`, this is
only done explicitly with the `convert` method.

```
use typenum::U4;
use ufix::si::{Centi, Milli};

assert_eq!(Milli::<U4>::new(0_300), Centi::<U4>::new(0_30).convert());
```

We can also choose a base-2 scale just as easily.

```
use typenum::U5;
use ufix::iec::{Kibi, Mebi};

assert_eq!(Kibi::<U5>::new(1024), Mebi::<U5>::new(1).convert());
```

It's also worth noting that the type-level scale changes when multiplying and dividing,
avoiding any implicit conversion.

```
use typenum::{U1, U2};
use ufix::iec::{Gibi, Kibi, Mebi};

assert_eq!(Mebi::<U1>::new(3), Gibi::<U2>::new(6) / Kibi::<U1>::new(2));
```

# `no_std`

This crate is `no_std`.

# `i128` support

Support for `u128` and `i128` can be enabled on nightly Rust through the `i128` Cargo feature.

 */

use super::{Cast, FromUnsigned, Mantissa, Positive, Radix, UnsignedPow};
use core::marker::PhantomData;
use typenum::Integer;

/**

Fixed-point number representing _2 <sup>`Bits`</sup> × `Base` <sup>`Exp`</sup>_.

- `Bits` is a type-level integer which represent width of mantissa in bits.
  * [`Unsigned`] (`U*`) number means unsigned type.
  * [`Integer`] (`P*`) number means signed type.
- `Base` is an [`Unsigned`] type-level integer.
- `Exp` is a signed type-level [`Integer`].

[`Unsigned`]: ../typenum/marker_traits/trait.Unsigned.html
[`Integer`]: ../typenum/marker_traits/trait.Integer.html

# Summary of operations

Lower case variables represent values of mantissa. Upper case _M_, _B_ and _E_ represent type-level
integers _Bits_, _Base_ and _Exp_, respectively.

- _−(x B<sup>E</sup>) = (−x) B<sup>E</sup>_

- _(x<sub>M<sub>x</sub></sub> B<sup>E<sub>x</sub></sup>) + (y<sub>M<sub>y</sub></sub> B<sup>E<sub>y</sub></sup>) =
   (x + y)<sub>max M<sub>x</sub> M<sub>y</sub></sub> B<sup>min E<sub>x</sub> E<sub>y</sub></sup>_

- _(x<sub>M<sub>x</sub></sub> B<sup>E<sub>x</sub></sup>) - (y<sub>M<sub>y</sub></sub> B<sup>E<sub>y</sub></sup>) =
   (x - y)<sub>max M<sub>x</sub> M<sub>y</sub></sub> B<sup>min E<sub>x</sub> E<sub>y</sub></sup>_

- _(x<sub>M<sub>x</sub></sub> B<sup>E<sub>x</sub></sup>) × (y<sub>M<sub>y</sub></sub> B<sup>E<sub>y</sub></sup>) =
   (x × y)<sub>M<sub>x</sub> + M<sub>y</sub></sub> B<sup>E<sub>x</sub> + E<sub>y</sub></sup>_

- _(x<sub>M<sub>x</sub></sub> B<sup>E<sub>x</sub></sup>) ÷ (y<sub>M<sub>y</sub></sub> B<sup>E<sub>y</sub></sup>) =
  (x ÷ y)<sub>M<sub>x</sub> - M<sub>y</sub></sub> B<sup>E<sub>x</sub> − E<sub>y</sub></sup>_

- _(x B<sup>E<sub>x</sub></sup>) % (y B<sup>E<sub>y</sub></sup>) =
  (x % y) B<sup>E<sub>x</sub></sup>_

 */
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Fix<R, B, E>
where
    R: Radix<B>,
{
    /// The real mantissa
    pub bits: R::Type,

    /// The phantom exponent
    pub exp: PhantomData<E>,
}

impl<R, B, E> Fix<R, B, E>
where
    R: Radix<B>,
    B: Positive,
    E: Integer,
{
    /// Creates a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use typenum::P2;
    /// use ufix::si::{Kilo, Milli};
    ///
    /// Milli::<P2>::new(25); // 0.025
    /// Kilo::<P2>::new(25); // 25 000
    /// ```
    pub fn new(bits: R::Type) -> Self {
        Fix {
            bits,
            exp: PhantomData,
        }
    }

    /// Converts to another _Exp_.
    fn into_exp<Er>(self) -> Fix<R, B, Er>
    where
        Er: Integer,
    {
        // radix^|exp-to_exp|
        let ratio =
            Mantissa::<R, B>::from_unsigned::<R>().unsigned_pow((E::I32 - Er::I32).abs() as u32);

        if E::I32 < Er::I32 {
            Fix::new(self.bits / ratio)
        } else {
            Fix::new(self.bits * ratio)
        }
    }

    /// Convert to another _Bits_.
    fn into_bits<Br>(self) -> Fix<R, Br, E>
    where
        R: Radix<Br>,
        Br: Positive,
        Mantissa<R, Br>: Cast<Mantissa<R, B>>,
    {
        Fix::new(Mantissa::<R, Br>::cast(self.bits))
    }

    /// Converts to another _Bits_ and/or _Exp_.
    ///
    /// # Examples
    ///
    /// ```
    /// use typenum::U1;
    /// use ufix::si::{Kilo, Milli};
    ///
    /// let kilo = Kilo::<U1>::new(5);
    /// let milli = Milli::<U1>::new(5_000_000);
    ///
    /// assert_eq!(kilo, milli.convert());
    /// assert_eq!(milli, kilo.convert());
    /// ```
    pub fn convert<Br, Er>(self) -> Fix<R, Br, Er>
    where
        R: Radix<Br>,
        Er: Integer,
        Br: Positive,
        Mantissa<R, Br>: Cast<Mantissa<R, B>>,
    {
        if B::U32 < Br::U32 {
            self.into_bits::<Br>().into_exp::<Er>()
        } else {
            self.into_exp::<Er>().into_bits::<Br>()
        }
    }
}
