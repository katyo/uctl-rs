use super::{Digits, Exponent, Fix, Mantissa, Radix};
use core::fmt::{Debug, Error, Formatter};

impl<R, B, E> Debug for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    Mantissa<R, B>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}[{}]x{}^{}", self.bits, B::I32, R::U32, E::I32)
    }
}
