use crate::{Cast, Fix, Mantissa, Positive, Radix};
use typenum::Integer;

macro_rules! cast_from {
    ($type: ty) => {
        impl<R, B, E> Cast<$type> for Fix<R, B, E>
        where
            R: Radix<B>,
            B: Positive,
            E: Integer,
            $type: Cast<Mantissa<R, B>>,
            Mantissa<R, B>: Cast<$type>,
        {
            fn cast(value: $type) -> Self {
                Self::from(value)
            }
        }

        impl<R, B, E> Cast<Fix<R, B, E>> for $type
        where
            R: Radix<B>,
            B: Positive,
            E: Integer,
            $type: Cast<Mantissa<R, B>>,
        {
            fn cast(val: Fix<R, B, E>) -> $type {
                val.into()
            }
        }
    };
}

cast_from!(u8);
cast_from!(u16);
cast_from!(u32);
cast_from!(u64);
#[cfg(feature = "i128")]
cast_from!(u128);

cast_from!(i8);
cast_from!(i16);
cast_from!(i32);
cast_from!(i64);
#[cfg(feature = "i128")]
cast_from!(i128);

cast_from!(f32);
cast_from!(f64);

impl<R, B, Br, E, Er> Cast<Fix<R, B, E>> for Fix<R, Br, Er>
where
    R: Radix<B> + Radix<Br>,
    B: Positive,
    Br: Positive,
    E: Integer,
    Er: Integer,
    Mantissa<R, Br>: Cast<Mantissa<R, B>>,
{
    fn cast(value: Fix<R, B, E>) -> Self {
        value.convert()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bin::{Fix32, Fix64},
        Cast,
    };
    use typenum::*;

    type F32 = Fix32<N16>;
    type F64 = Fix64<N32>;

    #[test]
    fn mul() {
        let a = F32::from(123.456);
        let b = F32::from(78.9);
        let c = F32::cast(a * b);

        assert_eq!(c, F32::from(9740.67715));
    }

    #[test]
    fn div() {
        let a = F32::from(6789.12);
        let b = F32::from(12.345);
        let c = F32::cast(F64::cast(a) / b);

        assert_eq!(c, F32::from(549.9496));
    }
}
