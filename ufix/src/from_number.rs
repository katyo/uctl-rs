use super::{Cast, Digits, Exponent, Fix, Mantissa, Radix};

macro_rules! from_impls {
    ($kind:ident: $($type:ident),*) => {
        $(
            impl<R, B, E> From<$type> for Fix<R, B, E>
            where
                R: Radix<B>,
                B: Digits,
                E: Exponent,
                $type: Cast<Mantissa<R, B>>,
                Mantissa<R, B>: Cast<$type>,
            {
                fn from(value: $type) -> Self {
                    // radix^|exp|
                    let ratio = R::ratio(E::I32.unsigned_abs());
                    // TODO: Add rounding
                    Self::new(if 0 < E::I32 {
                        from_impls!(@$kind, /, $type, Mantissa<R, B>, value, ratio)
                    } else {
                        from_impls!(@$kind, *, $type, Mantissa<R, B>, value, ratio)
                    })
                }
            }
        )*
    };

    (@float, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        <$BITS>::cast($value $OP <$TYPE>::cast($ratio))
    };

    (@int, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        <$BITS>::cast($value) $OP $ratio
    };
}

from_impls!(int: u8, u16, u32, u64, i8, i16, i32, i64);
from_impls!(float: f32, f64);

#[cfg(feature = "i128")]
from_impls!(int: u128, i128);

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
        assert_eq!(a, Milli::new(100));

        let a = Milli::<P4>::from(-2.5f32);
        assert_eq!(a, -Milli::new(2_500));
    }

    #[test]
    fn from_f64() {
        let a = Milli::<P4>::from(0.1f64);
        assert_eq!(a, Milli::new(100));

        let a = Milli::<P4>::from(-2.5f64);
        assert_eq!(a, -Milli::new(2_500));
    }
}
