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
use typenum::U3;
use ufix::si::Centi; // Fix<_, U10, N2>

assert_eq!(Centi::<U3>::new(0_30), Centi::<U3>::new(0_10) + Centi::<U3>::new(0_20));
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

use super::{BitsType, Cast, FromUnsigned, Pow};
use core::{
    marker::PhantomData,
    ops::{Div, Mul, Sub},
};
use typenum::{Abs, AbsVal, Bit, Diff, Integer, IsLess, Le, Unsigned, Z0};

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
pub struct Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
{
    /// The underlying integer.
    pub bits: Bits::Type,

    marker: PhantomData<(Base, Exp)>,
}

impl<Bits, Base, Exp> Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
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
    pub fn new(bits: Bits::Type) -> Self {
        Fix {
            bits,
            marker: PhantomData,
        }
    }

    /// Converts to another _Exp_.
    fn into_exp<ToExp>(self) -> Fix<Bits, Base, ToExp>
    where
        Bits::Type: FromUnsigned + Pow + Mul<Output = Bits::Type> + Div<Output = Bits::Type>,
        Base: Unsigned,
        Exp: Sub<ToExp>,
        Diff<Exp, ToExp>: Abs + IsLess<Z0>,
        AbsVal<Diff<Exp, ToExp>>: Integer,
    {
        let base = Bits::Type::from_unsigned::<Base>();
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

    /// Convert to another _Bits_.
    fn into_bits<ToBits>(self) -> Fix<ToBits, Base, Exp>
    where
        ToBits: BitsType<Base>,
        ToBits::Type: Cast<Bits::Type>,
    {
        Fix::new(ToBits::Type::cast(self.bits))
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
    pub fn convert<ToBits, ToExp>(self) -> Fix<ToBits, Base, ToExp>
    where
        Bits: IsLess<ToBits>,
        Bits::Type: FromUnsigned + Pow + Mul<Output = Bits::Type> + Div<Output = Bits::Type>,
        Base: Unsigned,
        ToBits: BitsType<Base>,
        ToBits::Type: FromUnsigned
            + Pow
            + Mul<Output = ToBits::Type>
            + Div<Output = ToBits::Type>
            + Cast<Bits::Type>,
        Exp: Sub<ToExp>,
        Diff<Exp, ToExp>: Abs + IsLess<Z0>,
        AbsVal<Diff<Exp, ToExp>>: Integer,
    {
        if Le::<Bits, ToBits>::to_bool() {
            self.into_bits::<ToBits>().into_exp()
        } else {
            self.into_exp::<ToExp>().into_bits()
        }
    }
}
