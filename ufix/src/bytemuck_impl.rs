use crate::{Digits, Exponent, Fix, Radix};
use bytemuck::{Pod, Zeroable};

unsafe impl<R, B, E> Zeroable for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
{
}

unsafe impl<R, B, E> Pod for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
{
}
