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
because internally it operates with integers, and exponent adjustement operations requires only
bitwise shifting.
Also you should avoid exceeding platform word size when it is possible without lossing required precision.

The components has additional parameters which represent internal types for operations.
You can select this types to get desired precision and fit to required range.

In any case you should create specific tests to be sure in correctness of operation.

 */

#![no_std]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]

mod consts;
mod filter;
mod regulator;
mod transform;
mod types;
mod utils;

pub use consts::*;
pub use filter::*;
pub use regulator::*;
pub use transform::*;
pub use types::*;
pub use utils::*;
