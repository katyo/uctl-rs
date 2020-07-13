/*!

# Flexible fixed-point numbers

This crate intended to simplify fixed-point types usage especially on FPU-less hardware.
To make it possible it introduces generic fixed-point type with usable and flexible operations on it.

## Overview

The introduced numeric type is generic with three type parameters:

* `Radix` - the __base__ of type, 2 for binary point, 10 for decimal point
* `Digits` - the number of valuable digits which represents the __mantissa__
* `Exponent` - the static __exponent__ value of type

So the value of type can be represented as _`mantissa` × `radix` <sup>`exponent`</sup>_.

```
# use ufix::{Fix, bin, dec};
# use typenum::*;
#
// Signed binary fixed with 5 bits mantissa and -3 as exponent.
// [5]*2^-3
type BF1 = Fix<P2, P5, N3>;
// or using type alias
type BF2 = bin::Fix<P5, N3>;

// Unsigned binary fixed with 5 bits mantissa and 3 as exponent.
// [5]*2^3
type UBF1 = Fix<U2, P5, P3>;
// or using type alias
type UBF2 = bin::UFix<P5, P3>;

// Signed decimal fixed with 12 digits mantissa and -7 as exponent.
// [12]*10^-7
type DF1 = Fix<P10, P12, N7>;
type DF2 = dec::Fix<P12, N7>;
```

The `P*` as the radix type parameter means signed type. To create unsigned types you can use `U*` instead.

Unlike well known and widely used __Qn.m__ representation the exponent is not constrained by mantissa bits. It can be less to represent more precisive small values. Also it can be greater than zero to represent less precision bigger values.

```
# use ufix::Fix;
# use typenum::*;
#
// [5]*2^-7
// 0b0.0000000 .. 0b0.0011111
type BF1 = Fix<U2, U5, N7>;

// [5]*2^7
// 0b0000000 .. 0b1111100
type BF2 = Fix<U2, U5, P7>;
```

Fixed point arithmetic has well known problems with overflowing especially on multiplication. Also it has well known problems with precision loss on division.

This crate avoids both overflows and precision losses by adjusting mantissa width and exponent value according to specific operations.

See examples below:

```
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P12, N6>::from(78.9);

// The addition without overflow
let c = a + b; // Fix<P17, N8>

assert_eq!(c, Fix::<P17, N8>::from(202.34));
```

```
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P16, N8>::from(78.9);

// The multiplication without overflow
let c = a * b; // Fix<P32, N16>

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

### Optimization techniques

When you targeted to [FPU](https://en.wikipedia.org/wiki/Floating-point_unit)-less hardware in order to get best possible performance and reduce firmware size you should use only binary fixed point arithmetic because internally it operates with integers, and exponent adjusting operations requires only bitwise shifting. Also you should avoid exceeding platform word size when it is possible without lossing required precision.

By default this crate use 32-bit integers as optimal to use on 32-bit processors. When you targeted to 16-bit or 8-bit processor you usually should use _word16_ or _word8_ features respectively.

### Supported features

This crate is `no_std` by design.

This crate supports the mantissa up to _64-bit_. Support for up to _128-bit_ can be enabled using the __i128__ feature.

The __word8__ and __word16__ features can reduce minimum mantissa size to 8 and 16 bits respectively. By default at least 32-bit words will be used.

### Evolution of `Fix` type

0. **fix** crate by _Curtis McEnroe_.

   That crate defines a simple fixed point type which support generic *base* and *exponent*.
   This work is started as successor of **fix**.

1. Changing meaning of `Bits` parameter from *mantisa type* to *number of mantissa bits*.

   In order to get more flexibility of fixed-point type we need know number of valuable bits of mantissa.
   So we would be able select appropriate types of results of operations easy.
   Also this make it possible to operate with operands with different mantissa and exponent.

2. Changing meaning of `Bits` parameter from *number of mantissa bits* to *number of mantissa digits according to base*.

   Actually it is more easy to operate with actual number of digits in number instead of binary bits.
   Because the digits depended from base, so for binary fixed-point types it is still bits as before.

 */

#![no_std]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]

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
mod types;
mod unsigned_pow;

pub use aliases::*;
pub use cast::Cast;
pub use fixed::Fix;
pub use positive::{FromPositive, Positive};
pub use radix::{Mantissa, Radix};
pub use types::{Digits, Exponent};
pub use unsigned_pow::UnsignedPow;
