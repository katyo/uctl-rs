use crate::{Digits, Error, Exponent, Fix, Mantissa, Radix, TryCast};
use core::fmt;
use serde::{
    de::{Deserializer, Error as DeError, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

impl<R, B, E> Serialize for Fix<R, B, E>
where
    R: Radix<B>,
    R::Type: SealedSer,
    B: Digits,
    E: Exponent,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        R::Type::sealed_ser(*self, serializer)
    }
}

impl<'de, R, B, E> Deserialize<'de> for Fix<R, B, E>
where
    R: Radix<B>,
    R::Type: SealedDe,
    B: Digits,
    E: Exponent,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        R::Type::sealed_de(deserializer)
    }
}

trait SealedSer {
    fn sealed_ser<R, B, E, S>(val: Fix<R, B, E>, ser: S) -> Result<S::Ok, S::Error>
    where
        R: Radix<B, Type = Self>,
        B: Digits,
        E: Exponent,
        S: Serializer;
}

trait SealedDe {
    fn sealed_de<'de, R, B, E, D>(de: D) -> Result<Fix<R, B, E>, D::Error>
    where
        R: Radix<B, Type = Self>,
        B: Digits,
        E: Exponent,
        D: Deserializer<'de>;
}

macro_rules! sealed_impls {
    ($($s:ident, $d:ident: $($t:ident),*;)*) => {
        $(
            $(
                impl SealedSer for $t
                {
                    fn sealed_ser<R, B, E, S>(val: Fix<R, B, E>, ser: S) -> Result<S::Ok, S::Error>
                    where
                        R: Radix<B, Type = Self>,
                        B: Digits,
                        E: Exponent,
                        S: Serializer,
                    {
                        ser.$s(val.into())
                    }
                }

                impl SealedDe for $t {
                    fn sealed_de<'de, R, B, E, D>(de: D) -> Result<Fix<R, B, E>, D::Error>
                    where
                        R: Radix<B, Type = Self>,
                        B: Digits,
                        E: Exponent,
                        D: Deserializer<'de>
                    {
                        de.$d(SealedVis::<R, B, E>::new())
                    }
                }
            )*
        )*
    }
}

sealed_impls! {
    serialize_f32, deserialize_f32: u8, u16, i8, i16;
    serialize_f64, deserialize_f64: i32, u32, i64, u64;
}

struct SealedVis<R, B, E> {
    _phantom: core::marker::PhantomData<(R, B, E)>,
}

impl<R, B, E> SealedVis<R, B, E> {
    fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

fn err_to_de<E: DeError>(error: Error) -> E {
    DeError::custom(error)
}

macro_rules! visitor_impl {
    ($($type:ident: $func:ident;)*) => {
        impl<R, B, E> Visitor<'_> for SealedVis<R, B, E>
        where
            R: Radix<B>,
            B: Digits,
            E: Exponent,
            $(
                $type: TryCast<Mantissa<R, B>>,
                Mantissa<R, B>: TryCast<$type>,
            )*
        {
            type Value = Fix<R, B, E>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a numeric value")
            }

            $(
                fn $func<F>(self, value: $type) -> Result<Self::Value, F>
                where
                    F: DeError,
                {
                    Self::Value::try_cast(value).map_err(err_to_de)
                }
            )*
        }
    };
}

visitor_impl! {
    u8: visit_u8;
    u16: visit_u16;
    u32: visit_u32;
    u64: visit_u64;
    i8: visit_i8;
    i16: visit_i16;
    i32: visit_i32;
    i64: visit_i64;
    f32: visit_f32;
    f64: visit_f64;
}

#[cfg(test)]
mod test {
    use crate::{bin, si};
    use serde_json::{from_str, to_string};
    use typenum::{N6, P9};

    type Uf = bin::UFix32<N6>;
    type Micro = si::Micro<P9>;

    #[test]
    fn serialize() {
        assert_eq!(to_string(&Micro::from(0.0)).unwrap(), "0.0");
        assert_eq!(to_string(&Micro::from(0.5)).unwrap(), "0.5");
        assert_eq!(to_string(&Micro::from(-0.5)).unwrap(), "-0.5");
        assert_eq!(to_string(&Micro::from(-111.25)).unwrap(), "-111.25");
        assert_eq!(to_string(&Micro::from(321.125)).unwrap(), "321.125");

        assert_eq!(to_string(&Uf::from(0.0)).unwrap(), "0.0");
        assert_eq!(to_string(&Uf::from(123.125)).unwrap(), "123.125");
        assert_eq!(to_string(&Uf::from(-0.5)).unwrap(), "0.0");
    }

    #[test]
    fn deserialize() {
        assert_eq!(from_str::<Micro>("0.0").unwrap(), Micro::from(0.0));
        assert_eq!(from_str::<Micro>("0.5").unwrap(), Micro::from(0.5));
        assert_eq!(from_str::<Micro>("-0.5").unwrap(), Micro::from(-0.5));
        assert_eq!(from_str::<Micro>("-111.25").unwrap(), Micro::from(-111.25));
        assert_eq!(from_str::<Micro>("321.125").unwrap(), Micro::from(321.125));

        assert_eq!(from_str::<Uf>("0.0").unwrap(), Uf::from(0.0));
        assert_eq!(from_str::<Uf>("123.125").unwrap(), Uf::from(123.125));
        assert!(from_str::<Uf>("-0.5").is_err());
    }
}
