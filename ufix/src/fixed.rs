/*!

Flexible and safe fixed-point numberic types.

# What?

Fixed-point is a number representation with a fixed number of digits before and after the radix point. This means that range is static rather than dynamic, as with floating-point. It also means that they can be represented as integers, with their scale tracked by the type system.

In this library, the scale of a `Fix` is represented as two type-level integers: _mantissa_ digits and _exponent_ value. The number of mantissa digits corresponds to selected radix: this is number of bits in case of binary fixed (when _radix_ is 2) and number of decimal places in case of decimal fixed (when _radix_ is 10). The underlying integer primitive which will be used to store the number in each concrete place will corresponds to radix and mantissa digits. Arithmetic can be performed on these numbers, and they can be converted to different scale exponents explicitly.

# Why?

A classic example: let's sum 10 cents and 20 cents using binary floating-point. We expect a result of 30 cents.

```should_panic
assert_eq!(0.30, 0.10 + 0.20);
```

Wrong! We get an extra forty quintillionths of a dollar.

```text
assertion failed: `(left == right)` (left: `0.3`, right: `0.30000000000000004`)'
```

This is due to neither 0.1 nor 0.2 being exactly representable in base-2, just as a third can't be represented exactly in base-10. With `Fix`, we can choose the precision we want in base-10, at compile-time. In this case, hundredths of a dollar will do.

```
use typenum::{P3, P4};
use ufix::si::Centi; // dec::Fix<_, N2>

assert_eq!(Centi::<P4>::new(0_30), Centi::<P3>::new(0_10) + Centi::<P3>::new(0_20));
```

But decimal is inefficient for binary computers, right? Multiplying and dividing by 10 is slower than bit-shifting, but that's only needed when _moving_ the point. With `Fix`, this is only done explicitly with the `convert` method.

```
use typenum::P4;
use ufix::si::{Centi, Milli};

assert_eq!(Milli::<P4>::new(0_300), Centi::<P4>::new(0_30).convert());
```

We can also choose a base-2 scale just as easily.

```
use typenum::P10;
use ufix::iec::{Kibi, Mebi};

assert_eq!(Kibi::<P10>::new(1024), Mebi::<P10>::new(1).convert());
```

It's also worth noting that the type-level scale changes when multiplying and dividing, avoiding any implicit conversion.

```
use typenum::{P1, P2};
use ufix::iec::{Gibi, Kibi, Mebi};

assert_eq!(Mebi::<P1>::new(3), Gibi::<P2>::new(6) / Kibi::<P1>::new(2));
```

 */

use super::{Cast, Digits, Exponent, Mantissa, Radix};
use core::marker::PhantomData;

/**

Fixed-point number representing _ `Radix` <sub>`Digits`</sub> × `Radix` <sup>`Exponent`</sup>_.

- `Radix` is a type-level integer which represents the base of number.
  * [`Unsigned`] (`U*`) number means unsigned type.
  * [`Integer`] (`P*`) number means signed type.
- `Digits` is a signed positive type-level [`Integer`] which represent width of mantissa in digits of specified radix.
- `Exponent` is a signed type-level [`Integer`].

[`Unsigned`]: ../typenum/marker_traits/trait.Unsigned.html
[`Integer`]: ../typenum/marker_traits/trait.Integer.html

# Summary of operations

Lower case variables represent values of mantissa. Upper case _R_, _B_ and _E_ represent type-level integers _Radix_, _Digits_ and _Exponent_, respectively.

## Arithmetic

__TODO__: Update outdated docs

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

## Comparison

To compare fixed-point values of different types you should first cast at least one of its to type of other or cast both to single common type. Implicit semi-automatic conversion lacks because in common case it may give ambiguous results.

 */
#[derive(Clone, Copy, Default)]
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

//impl<R: Clone, B: Clone, E: Clone> Copy for Fix<R, B, E> {}

impl<R, B, E> Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
{
    /// Minimum value
    pub const MIN: Self = Self::new(R::MIN);

    /// Maximum value
    pub const MAX: Self = Self::new(R::MAX);

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
    pub const fn new(bits: R::Type) -> Self {
        Fix {
            bits,
            exp: PhantomData,
        }
    }

    /// Converts to another _Exp_.
    fn into_exp<Er>(self) -> Fix<R, B, Er>
    where
        Er: Exponent,
    {
        // radix^|exp-to_exp|
        let ratio = R::ratio((E::I32 - Er::I32).abs() as u32);

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
        Br: Digits,
        Mantissa<R, Br>: Cast<Mantissa<R, B>>,
    {
        Fix::new(Mantissa::<R, Br>::cast(self.bits))
    }

    /// Converts to another _Bits_ and/or _Exp_.
    ///
    /// # Examples
    ///
    /// ```
    /// use typenum::{P1, P7};
    /// use ufix::si::{Kilo, Milli};
    ///
    /// let kilo = Kilo::<P1>::new(5);
    /// let milli = Milli::<P7>::new(5_000_000);
    ///
    /// assert_eq!(kilo, milli.convert());
    /// assert_eq!(milli, kilo.convert());
    /// ```
    pub fn convert<Br, Er>(self) -> Fix<R, Br, Er>
    where
        R: Radix<Br>,
        Er: Exponent,
        Br: Digits,
        Mantissa<R, Br>: Cast<Mantissa<R, B>>,
    {
        if B::I32 < Br::I32 {
            self.into_bits::<Br>().into_exp::<Er>()
        } else {
            self.into_exp::<Er>().into_bits::<Br>()
        }
    }
}
