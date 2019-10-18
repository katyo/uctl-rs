use core::mem::size_of;
use super::{FromOther};
use crate::{Fix, Unsigned, FromUnsigned, BitsType, Div, Sub, Mul, Pow};
use typenum::{IsLess, Abs, AbsVal, Integer, Z0, Diff};

impl<Bits, Base, Exp> FromOther<f64> for Fix<Bits, Base, Exp>
where
    f64: FromOther<Bits::Type>,
    Bits: BitsType,
    Bits::Type: FromUnsigned + Pow + FromOther<f64> + Mul<Bits::Type, Output = Bits::Type> + Div<Bits::Type, Output = Bits::Type>,
    Base: Unsigned,
    Z0: IsLess<Exp>,
    Exp: Abs,
    AbsVal<Exp>: Integer,
{
    fn from_other(value: f64) -> Self {
        Self::from(value)
    }
}

impl<Bits, ToBits, Base, Exp, ToExp> FromOther<Fix<Bits, Base, Exp>> for Fix<ToBits, Base, ToExp>
where
    Bits: BitsType,
    Bits::Type: FromUnsigned + Pow + Mul<Output = Bits::Type> + Div<Output = Bits::Type>,
    ToBits: BitsType,
    ToBits::Type: FromUnsigned + Pow + Mul<Output = ToBits::Type> + Div<Output = ToBits::Type> + FromOther<Bits::Type>,
    Base: Unsigned,
    Exp: Sub<ToExp>,
    Diff<Exp, ToExp>: Abs + IsLess<Z0>,
    AbsVal<Diff<Exp, ToExp>>: Integer
{
    fn from_other(value: Fix<Bits, Base, Exp>) -> Self {
        if size_of::<ToBits::Type>() > size_of::<Bits::Type>() {
            Fix::<ToBits, Base, Exp>::new(ToBits::Type::from_other(value.bits)).convert()
        } else {
            Fix::new(ToBits::Type::from_other(value.convert().bits))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{N16, N32, bin::{IFix32, IFix64}};
    use super::*;

    type F32 = IFix32<N16>;
    type F64 = IFix64<N16>;
    type F64D = IFix64<N32>;

    #[test]
    fn mul() {
        let a = F32::from(123.456);
        let b = F32::from(78.9);
        let c = F32::from_other(F64::from_other(a) * F64::from_other(b));

        assert_eq!(c, F32::from(9740.67715));
    }

    #[test]
    fn div() {
        let a = F32::from(6789.12);
        let b = F32::from(12.345);
        let c = F32::from_other(F64D::from_other(a) / F64::from_other(b));

        assert_eq!(c, F32::from(549.9496));
    }
}
