use core::ops::{Mul, Div};
use typenum::{Z0, Bit, Integer, Unsigned, AbsVal, Le, Abs, IsLess};
use super::{FromUnsigned, BitsType, Pow, Fix, Cast};

macro_rules! into_num {
    ($TYPE: ty, $KIND: tt) => {
        impl<Bits, Base, Exp> From<Fix<Bits, Base, Exp>> for $TYPE
        where
            $TYPE: Cast<Bits::Type>,
            Bits: BitsType,
            Bits::Type: FromUnsigned + Pow + Mul<Bits::Type, Output = Bits::Type> + Div<Bits::Type, Output = Bits::Type>,
            Base: Unsigned,
            Z0: IsLess<Exp>,
            Exp: Abs,
            AbsVal<Exp>: Integer,
        {
            fn from(value: Fix<Bits, Base, Exp>) -> Self {
                let base = Bits::Type::from_unsigned::<Base>(); // e
                let abs_exp = AbsVal::<Exp>::to_i32() as u32; // |exp|
                let exp_pos = Le::<Z0, Exp>::to_bool(); // 0 < exp

                // FIXME: Would like to do this with typenum::Pow, but that
                // seems to result in overflow evaluating requirements.
                let ratio = base.pow(abs_exp); // base^|exp|
                let value = value.bits;

                // TODO: Add rounding

                if exp_pos {
                    into_num!(@$KIND, *, $TYPE, value, ratio)
                } else {
                    into_num!(@$KIND, /, $TYPE, value, ratio)
                }
            }
        }
    };

    (@int, $OP: tt, $TYPE: ty, $value: ident, $ratio: ident) => {
        <$TYPE>::cast($value $OP $ratio)
    };

    (@float, $OP: tt, $TYPE: ty, $value: ident, $ratio: ident) => {
        <$TYPE>::cast($value) $OP <$TYPE>::cast($ratio)
    };
}

into_num!(i8, int);
into_num!(i16, int);
into_num!(i32, int);
into_num!(i64, int);
#[cfg(feature = "i128")]
into_num!(i128, int);

into_num!(u8, int);
into_num!(u16, int);
into_num!(u32, int);
into_num!(u64, int);
#[cfg(feature = "i128")]
into_num!(u128, int);

into_num!(f32, float);
into_num!(f64, float);

#[cfg(test)]
mod test {
    use crate::si::Milli;
    use typenum::*;

    #[test]
    fn into_u16() {
        let a = Milli::<P16>::new(9_000);
        assert_eq!(u16::from(a), 9);
    }

    #[test]
    fn into_u16_neg() {
        let a = Milli::<P16>::new(-9_000);
        assert_eq!(u16::from(a), 65527);
    }

    #[test]
    fn into_i16() {
        let a = Milli::<P16>::new(9_000);
        assert_eq!(i16::from(a), 9);
    }

    #[test]
    fn into_i16_neg() {
        let a = Milli::<P16>::new(-9_000);
        assert_eq!(i16::from(a), -9);
    }

    #[test]
    fn into_u32() {
        let a = Milli::<P32>::new(9_000);
        assert_eq!(u32::from(a), 9);
    }

    #[test]
    fn into_i32_neg() {
        let a = Milli::<P32>::new(-9_000);
        assert_eq!(i32::from(a), -9);
    }

    #[test]
    fn into_u64() {
        let a = Milli::<P64>::new(9_000);
        assert_eq!(u64::from(a), 9);
    }

    #[test]
    fn into_i64_neg() {
        let a = Milli::<P64>::new(-9_000);
        assert_eq!(i64::from(a), -9);
    }

    #[test]
    fn into_f32() {
        let a = Milli::<P32>::new(0_100);
        assert_eq!(f32::from(a), 0.1);

        let a = Milli::<P32>::new(-2_500);
        assert_eq!(f32::from(a), -2.5);
    }

    #[test]
    fn into_f64() {
        let a = Milli::<P32>::new(0_100);
        assert_eq!(f64::from(a), 0.1);

        let a = Milli::<P32>::new(-2_500);
        assert_eq!(f64::from(a), -2.5);
    }
}
