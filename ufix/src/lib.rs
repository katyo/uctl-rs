/*!

# Fixed-point numbers

This crate intended to simplify fixed-point calculations especially on FPU-less hardware.
To make it possible it introduces generic fixed-point type with usable and flexible operations on it.

## Overview

The introduced numeric type is generic with three type parameters:

* `Bits` - the number of valuable bits which represents __mantissa__
* `Base` - the __base__ of type (2 for binary point, 10 for decimal point, etc.)
* `Exp` - the __exponent__ of type

So the value of type can be represented as _2 <sup>`Bits`</sup> × `Base` <sup>`Exp`</sup>_.

```
# use ufix::{Fix, bin, dec};
# use typenum::*;
#
// Signed binary fixed with 5 bits mantissa and -3 exponent.
// [5]*2^-3
type BF1 = Fix<P5, U2, N3>;

// Also as previous but using type alias.
type BF2 = bin::Fix<P5, N3>;

// Signed decimal fixed with 12 bits mantissa and -7 exponent.
// [12]*10^-7
type DF1 = Fix<P12, U10, N7>;
type DF2 = dec::Fix<P12, N7>;
```

The `P*` type parameter means signed type.
To create unsigned types you can use `U*` instead.

Unlike well known and widely used __Qn.m__ representation the exponent is not constrained by mantissa bits.

```
# use ufix::Fix;
# use typenum::*;
#
// [5]*2^-7
// 0b0.0000000 .. 0b0.0011111
type BF1 = Fix<U5, U2, N7>;

// [5]*2^7
// 0b0000000 .. 0b1111100
type BF2 = Fix<U5, U2, P7>;
```

### Optimization techniques

When you targeted to [FPU](https://en.wikipedia.org/wiki/Floating-point_unit)-less hardware in order
to get best possible performance and reduce firmware size you should use only binary fixed point arithmetic
because internally it operates with integers, and exponent adjustement operations requires only
bitwise shifting.
Also you should avoid exceeding platform word size when it is possible without lossing required precision.

By default this crate use 32-bit integers as optimal to use on 32-bit processors.
When you targeted to 16-bit or 8-bit processor you should use *word16* or *word8* features respectively.

### Safe usage

Fixed point arithmetic has well known problems with overflowing especially on multiplication.
Also it has well known problems with precision loss on division.

The simple way to avoid overflow is using value types of double bit-width in operation with following reducing to original width.

For example, in case of multiplication we can cast 32-bit fixed-point number to 64-bit with same base and exponent.
As result we get 64-bit fixed-point number with exponent, which equals a sum of arguments exponents.

In case of division to prevent lossing precision we can cast 32-bit numerator to 64-bit with double exponent and keep 32-bit denominator as is.
In result we get 32-bit number with exponent, which equals a difference of numerator (after cast) and denominator exponents.

See examples below:

```rust
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P16, N8>::from(78.9);

// The multiplication without overflow
let c = a * b;

assert_eq!(c, Fix::<P32, N16>::from(9739.95047));
```

```
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P16, N8>::from(78.9);

// The division without precision loss
let c = Fix::<P32, N16>::cast(a) / b;

assert_eq!(c, Fix::<P16, N8>::from(1.5647));
```

 */

#![cfg_attr(feature = "no_std", no_std)]

mod cast;
mod bits_type;
mod from_unsigned;
mod unsigned_pow;
mod fixed;
mod operators;
mod aliases;
mod from_number;
mod cast_fixed;
mod format;

pub use self::cast::Cast;
pub use self::bits_type::BitsType;
pub use self::from_unsigned::FromUnsigned;
pub use self::unsigned_pow::UnsignedPow as Pow;
pub use self::fixed::Fix;
pub use self::aliases::*;
