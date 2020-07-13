use crate::{Digits, Exponent, Fix, Mantissa, Radix};
use core::cmp::Ordering;

impl<R, B, E> PartialEq for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    Mantissa<R, B>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.bits.eq(&other.bits)
    }
}

impl<R, B, E> Eq for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    Mantissa<R, B>: PartialEq,
{
}

impl<R, B, E> PartialOrd for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    Mantissa<R, B>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<R, B, E> Ord for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    Mantissa<R, B>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits.cmp(&other.bits)
    }
}
