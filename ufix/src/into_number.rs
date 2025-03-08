use super::{Cast, Digits, Exponent, Fix, Mantissa, Radix};

macro_rules! into_num {
    ($TYPE: ty, $KIND: tt) => {
        impl<R, B, E> From<Fix<R, B, E>> for $TYPE
        where
            R: Radix<B>,
            B: Digits,
            E: Exponent,
            $TYPE: Cast<Mantissa<R, B>>,
        {
            fn from(Fix { bits: value, .. }: Fix<R, B, E>) -> Self {
                // radix^|exp|
                let ratio = R::ratio(E::I32.unsigned_abs());

                // TODO: Add rounding

                if 0 < E::I32 {
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
        let a = Milli::<P4>::new(9_000);
        assert_eq!(u16::from(a), 9);
    }

    #[test]
    fn into_u16_neg() {
        let a = Milli::<P4>::new(-9_000);
        assert_eq!(u16::from(a), 65527);
    }

    #[test]
    fn into_i16() {
        let a = Milli::<P4>::new(9_000);
        assert_eq!(i16::from(a), 9);
    }

    #[test]
    fn into_i16_neg() {
        let a = Milli::<P4>::new(-9_000);
        assert_eq!(i16::from(a), -9);
    }

    #[test]
    fn into_u32() {
        let a = Milli::<P8>::new(9_000);
        assert_eq!(u32::from(a), 9);
    }

    #[test]
    fn into_i32_neg() {
        let a = Milli::<P8>::new(-9_000);
        assert_eq!(i32::from(a), -9);
    }

    #[test]
    fn into_u64() {
        let a = Milli::<P16>::new(9_000);
        assert_eq!(u64::from(a), 9);
    }

    #[test]
    fn into_i64_neg() {
        let a = Milli::<P16>::new(-9_000);
        assert_eq!(i64::from(a), -9);
    }

    #[test]
    fn into_f32() {
        let a = Milli::<P4>::new(100);
        assert_eq!(f32::from(a), 0.1);

        let a = Milli::<P4>::new(-2_500);
        assert_eq!(f32::from(a), -2.5);
    }

    #[test]
    fn into_f64() {
        let a = Milli::<P4>::new(100);
        assert_eq!(f64::from(a), 0.1);

        let a = Milli::<P4>::new(-2_500);
        assert_eq!(f64::from(a), -2.5);
    }
}
