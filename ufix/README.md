# Flexible fixed-point numbers

[![github](https://img.shields.io/badge/github-katyo/uctl--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/uctl-rs)
[![crate](https://img.shields.io/crates/v/ufix.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/ufix)
[![docs](https://img.shields.io/badge/docs.rs-ufix-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/ufix)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/katyo/uctl-rs/ci.yml?branch=master&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/ufix-rs/actions?query=workflow%3ARust)

This crate intended to simplify fixed-point types usage especially on FPU-less hardware.
To make it possible it introduces generic fixed-point type with usable and flexible operations on it.

## Overview

The introduced numeric type is generic with three type parameters:

* `Radix` - the __base__ of type, 2 for binary point, 10 for decimal point
* `Digits` - the number of valuable digits which represents the __mantissa__
* `Exponent` - the static __exponent__ value of type

So the value of type can be represented as _`mantissa` × `radix` <sup>`exponent`</sup>_.

```rust
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
type DF1 = Fix<U10, P12, N7>;
type DF2 = dec::Fix<P12, N7>;
```

The `P*` as the radix type parameter means signed type. To create unsigned types you can use `U*` instead.

Unlike well known and widely used __Qn.m__ representation the exponent is not constrained by mantissa bits. It can be less to represent more precisive small values. Also it can be greater than zero to represent less precision bigger values.

```rust
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

```rust
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P12, N6>::from(78.9);

// The addition without overflow
let c = a + b; // Fix<P17, N8>

assert_eq!(c, Fix::<P17, N8>::from(202.34));
```

```rust
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P16, N8>::from(78.9);

// The multiplication without overflow
let c = a * b; // Fix<P32, N16>

assert_eq!(c, Fix::<P32, N16>::from(9739.95047));
```

```rust
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

By default this crate use 32-bit integers as optimal to use on 32-bit processors. When you targeted to 16-bit or 8-bit processor you usually should use __word16_ or __word8_ features respectively.

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
