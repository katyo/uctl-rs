# Generic control library for low-end hadrware

[![github](https://img.shields.io/badge/github-katyo/uctl--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/uctl-rs)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/katyo/uctl-rs/ci.yml?branch=master&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/ufix-rs/actions?query=workflow%3ARust)

This library intended to simplify developing control algorithms for bare-metal low-end hardware such as
microcontrollers.

## Current status

Currently Rust is quite restricted in using constants as a type parameters. To get the original ideas works well it requires support for floating-point operations at type level which implement only the languages with fullfeatured compile-time expression evaluation like D and C++ (with restrictions and quirks).
Due to reasons above the development currently focussed on [uctl-d](https://github.com/katyo/uctl-d).

## Overview

This library consist of several independent components which grouped to filters, regulators and convertors.
Also it includes some useful utilities, such as clampers, scalers and etc.

The filters and regulators can be configured in a human-friendly way without using obscure artifical coefficients.

All components can operate both with floating point and fixed point values.

### Optimization techniques

When you targeted to FPU-less hardware in order to get best possible performance and reduce firmware size you should use only binary fixed point arithmetic because internally it operates with integers.
Also you should avoid exceeding platform word size when it is possible without lossing required precision.

### Safe usage

Fixed point arithmetic has well known problems with overflowing especially on multiplication.
Also it has well known problems with precision loss on division.

## Links

* [ufix crate](https://crates.io/crates/ufix)
* [ufix API](https://docs.rs/ufix)
* [uctl crate](https://crates.io/crates/uctl)
* [uctl API](https://docs.rs/uctl)
