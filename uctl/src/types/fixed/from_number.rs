use typenum::{
    Z0,
    Bit, Integer, Unsigned,
    AbsVal, Le, Abs, IsLess,
};

use super::{Fix, FromUnsigned};

macro_rules! from_num {
    ($BITS: ty) => {
        from_num!($BITS, i8, int);
        from_num!($BITS, i16, int);
        from_num!($BITS, i32, int);
        from_num!($BITS, i64, int);
        #[cfg(feature = "i128")]
        from_num!($BITS, i128, int);

        from_num!($BITS, u8, int);
        from_num!($BITS, u16, int);
        from_num!($BITS, u32, int);
        from_num!($BITS, u64, int);
        #[cfg(feature = "i128")]
        from_num!($BITS, u128, int);

        from_num!($BITS, f32, float);
        from_num!($BITS, f64, float);
    };

    ($BITS: ty, $TYPE: ty, $KIND: tt) => {
        impl<Base, Exp> From<$TYPE> for Fix<$BITS, Base, Exp>
        where Base: Unsigned,
              Z0: IsLess<Exp>,
              Exp: Abs,
              AbsVal<Exp>: Integer,
        {
            fn from(value: $TYPE) -> Self {
                let base = <$BITS>::from_unsigned::<Base>(); // e
                let abs_exp = AbsVal::<Exp>::to_i32() as u32; // |exp|
                let exp_pos = Le::<Z0, Exp>::to_bool(); // 0 < exp

                // FIXME: Would like to do this with typenum::Pow, but that
                // seems to result in overflow evaluating requirements.
                let ratio = base.pow(abs_exp); // base^|exp|

                // TODO: Add rounding

                Self::new(if exp_pos {
                    from_num!(@$KIND, /, $TYPE, $BITS, value, ratio)
                } else {
                    from_num!(@$KIND, *, $TYPE, $BITS, value, ratio)
                })
            }
        }
    };

    (@float, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        (($value $OP ($ratio as $TYPE)) as $BITS)
    };

    (@int, $OP: tt, $TYPE: ty, $BITS: ty, $value: ident, $ratio: ident) => {
        (($value as $BITS) $OP ($ratio as $BITS))
    };
}

from_num!(i8);
from_num!(i16);
from_num!(i32);
from_num!(i64);
#[cfg(feature = "i128")]
from_num!(i128);

from_num!(u8);
from_num!(u16);
from_num!(u32);
from_num!(u64);
#[cfg(feature = "i128")]
from_num!(u128);

#[cfg(test)]
mod test {
    use super::super::si::Milli;

    #[test]
    fn from_u16() {
        let a = Milli::<i16>::from(9u16);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i16() {
        let a = Milli::<i16>::from(-11i16);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_u32() {
        let a = Milli::<i32>::from(9u32);
        assert_eq!(a, Milli::new(9_000));
    }

    #[test]
    fn from_i32() {
        let a = Milli::<i16>::from(-11i32);
        assert_eq!(a, Milli::new(-11_000));
    }

    #[test]
    fn from_f32() {
        let a = Milli::<i32>::from(0.1f32);
        assert_eq!(a, Milli::new(0_100));

        let a = Milli::<i16>::from(-0.5f32);
        assert_eq!(a, -Milli::new(0_500));
    }

    #[test]
    fn from_f64() {
        let a = Milli::<i32>::from(0.1f32);
        assert_eq!(a, Milli::new(0_100));

        let a = Milli::<i16>::from(-0.5f64);
        assert_eq!(a, -Milli::new(0_500));
    }
}
