use super::{Cast, Digits, Exponent, Fix, Mantissa, Radix};

macro_rules! from_num {
    ($TYPE: ty, $KIND: tt) => {
        impl<R, B, E> From<$TYPE> for Fix<R, B, E>
        where
            R: Radix<B>,
            B: Digits,
            E: Exponent,
            $TYPE: Cast<Mantissa<R, B>>,
            Mantissa<R, B>: Cast<$TYPE>,
        {
            fn from(value: $TYPE) -> Self {
                // radix^|exp|
                let ratio = R::ratio(E::I32.abs() as u32);
                // TODO: Add rounding
                Self::new(if 0 < E::I32 {
                    from_num!(@$KIND, /, $TYPE, Mantissa<R, B>, value, ratio)
                } else {
                    from_num!(@$KIND, *, $TYPE, Mantissa<R, B>, value, ratio)
                })
            }
        }
    };

    (@float, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        <$BITS>::cast($value $OP <$TYPE>::cast($ratio))
    };

    (@int, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        <$BITS>::cast($value) $OP $ratio
    };
}

from_num!(i8, int);
from_num!(i16, int);
from_num!(i32, int);
from_num!(i64, int);
#[cfg(feature = "i128")]
from_num!(i128, int);

from_num!(u8, int);
from_num!(u16, int);
from_num!(u32, int);
from_num!(u64, int);
#[cfg(feature = "i128")]
from_num!(u128, int);

from_num!(f32, float);
from_num!(f64, float);

#[cfg(test)]
mod test {
    use crate::si::Milli;
    use typenum::*;

    #[test]
    fn from_u16() {
        let a = Milli::<P4>::from(9u16);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i16() {
        let a = Milli::<P4>::from(11i16);
        assert_eq!(a, Milli::new(11_000));
    }

    #[test]
    fn from_i16_neg() {
        let a = Milli::<P4>::from(-11i16);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_u32() {
        let a = Milli::<P8>::from(9u32);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i32_neg() {
        let a = Milli::<P8>::from(-11i32);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_u64() {
        let a = Milli::<P16>::from(9u64);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i64_neg() {
        let a = Milli::<P16>::from(-11i64);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_f32() {
        let a = Milli::<P4>::from(0.1f32);
        assert_eq!(a, Milli::new(0_100));

        let a = Milli::<P4>::from(-2.5f32);
        assert_eq!(a, -Milli::new(2_500));
    }

    #[test]
    fn from_f64() {
        let a = Milli::<P4>::from(0.1f64);
        assert_eq!(a, Milli::new(0_100));

        let a = Milli::<P4>::from(-2.5f64);
        assert_eq!(a, -Milli::new(2_500));
    }
}
