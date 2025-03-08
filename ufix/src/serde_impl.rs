use crate::{Cast, Digits, Exponent, Fix, Mantissa, Radix};
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
    serialize_f32, deserialize_f32: u8, u16, u32, i8, i16, i32, f32;
    serialize_f64, deserialize_f64: i64, u64, f64;
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

impl<'de, R, B, E> Visitor<'de> for SealedVis<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    u8: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<u8>,
    i8: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<i8>,
    u16: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<u16>,
    i16: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<i16>,
    u32: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<u32>,
    i32: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<i32>,
    u64: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<u64>,
    i64: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<i64>,
    f32: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<f32>,
    f64: Cast<Mantissa<R, B>>,
    Mantissa<R, B>: Cast<f64>,
{
    type Value = Fix<R, B, E>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a floating-point value")
    }

    fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < u8::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > u8::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < i8::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > i8::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < u16::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > u16::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_i16<F>(self, value: i16) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < i16::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > i16::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < u32::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > u32::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < i32::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > i32::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < u64::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > u64::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < i64::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > i64::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < f32::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > f32::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }

    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: DeError,
    {
        if value < f64::from(Self::Value::MIN) {
            Err(F::custom("Value too low"))
        } else if value > f64::from(Self::Value::MAX) {
            Err(F::custom("Value too high"))
        } else {
            Ok(Self::Value::from(value))
        }
    }
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
