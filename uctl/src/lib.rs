/*!

# Generic control library for low-end hadrware

This crate intended to simplify developing control algorithms for bare-metal low-end hardware such as microcontrollers.

## Overview

This library consist of several independent components which grouped to filters, regulators and convertors.
Also it includes some useful utilities, such as clampers, scalers and etc.

The filters and regulators can be configured in a human-friendly way without using obscure artifical coefficients.

All components can operate both with floating point and fixed point values.


### Optimization techniques

When you targeted to [FPU](https://en.wikipedia.org/wiki/Floating-point_unit)-less hardware in order
to get best possible performance and reduce firmware size you should use only binary fixed point arithmetic
(_Bits × 2 <sup>Exp</sup>_) because internally it operates with integers, and exponent adjustement operations requires only
bitwise shifting.
Also you should avoid exceeding platform word size when it is possible without lossing required precision.

### Safe usage

Fixed point arithmetic has well known problems with overflowing especially on multiplication.
Also it has well known problems with precision loss on division.

The simple way to avoid overflow is using value types of double bit-width in operation with following reducing to original width.

For example, in case of multiplication you can cast 32-bit fixed-point number to 64-bit with same base and exponent.
As result you get 64-bit fixed-point number with exponent, which equals a sum of arguments exponents.

In case of division to prevent lossing precision you can cast 32-bit numerator to 64-bit with double exponent and keep 32-bit denominator as is.
In result you get 32-bit number with exponent, which equals a difference of numerator (after cast) and denominator exponents.

See examples below:

```rust
use uctl::{FromOther, bin::{IFix32, IFix64}};
use typenum::{N16, N32};

// The ordinary value type
type F32 = IFix32<N16>;

// The type for multiplication and denominator type for division
type F64 = IFix64<N16>;

// The numerator type for division
type F64D = IFix64<N32>;

let a = F32::from(123.456);
let b = F32::from(78.9);

// The multiplication without overflow
let c = F32::from_other(F64::from_other(a) * F64::from_other(b));

assert_eq!(c, F32::from(9740.67715));

// The division without precision loss
let d = F32::from_other(F64D::from_other(c) / F64::from_other(b));

assert_eq!(d, F32::from(123.45599));
```

The components has additional parameters which represent internal types for operations.
You can select this types to get desired precision and fit to required range.

In any case you should create specific tests to be sure in correctness of operation.

 */

#![cfg_attr(feature = "no_std", no_std)]

mod basic;
mod types;
mod traits;
mod filter;
mod regulator;
mod transform;

pub(crate) use basic::*;
pub use types::*;
pub use traits::*;
pub use filter::*;
pub use regulator::*;
pub use transform::*;
