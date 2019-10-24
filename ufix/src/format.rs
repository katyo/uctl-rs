use core::fmt::{Debug, Error, Formatter};
use typenum::{Integer, Unsigned};
use super::{BitsType, Fix};

impl<Bits, Base, Exp> Debug for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: Debug,
    Base: Unsigned,
    Exp: Integer
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}x{}^{}", self.bits, Base::to_u64(), Exp::to_i64())
    }
}
