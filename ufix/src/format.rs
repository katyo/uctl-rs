use super::{Fix, Mantissa, Positive, Radix};
use core::fmt::{Debug, Error, Formatter};
use typenum::Integer;

impl<R, B, E> Debug for Fix<R, B, E>
where
    R: Radix<B>,
    B: Positive,
    E: Integer,
    Mantissa<R, B>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}[{}]x{}^{}", self.bits, B::U32, R::U32, E::I32)
    }
}
