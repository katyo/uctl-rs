use crate::{Cast, Digits, Exponent, Fix, Mantissa, Radix, Result, TryCast, TryMul};

macro_rules! cast_impls {
    ($kind:ident: $($type:ident),*) => {
        $(
            impl<R, B, E> Cast<$type> for Fix<R, B, E>
            where
                R: Radix<B>,
                B: Digits,
                E: Exponent,
                $type: Cast<Mantissa<R, B>>,
                Mantissa<R, B>: Cast<$type>,
            {
                fn cast(value: $type) -> Self {
                    // radix^|exp|
                    let ratio = R::ratio(E::I32.unsigned_abs());
                    // TODO: Add rounding
                    Self::new(if 0 < E::I32 {
                        cast_impls!(@from $kind, /, $type, Mantissa<R, B>, value, ratio)
                    } else {
                        cast_impls!(@from $kind, *, $type, Mantissa<R, B>, value, ratio)
                    })
                }
            }

            impl<R, B, E> Cast<Fix<R, B, E>> for $type
            where
                R: Radix<B>,
                B: Digits,
                E: Exponent,
                $type: Cast<Mantissa<R, B>>,
            {
                fn cast(Fix { bits: value, .. }: Fix<R, B, E>) -> $type {
                    // radix^|exp|
                    let ratio = R::ratio(E::I32.unsigned_abs());
                    // TODO: Add rounding
                    if 0 < E::I32 {
                        cast_impls!(@into $kind, *, $type, value, ratio)
                    } else {
                        cast_impls!(@into $kind, /, $type, value, ratio)
                    }
                }
            }

            impl<R, B, E> TryCast<$type> for Fix<R, B, E>
            where
                R: Radix<B>,
                B: Digits,
                E: Exponent,
                $type: TryCast<Mantissa<R, B>>,
                Mantissa<R, B>: TryCast<$type>,
            {
                fn try_cast(value: $type) -> Result<Self> {
                    // radix^|exp|
                    let ratio = R::ratio(E::I32.unsigned_abs());
                    // TODO: Add rounding
                    Ok(Self::new(if 0 < E::I32 {
                        cast_impls!(@tryfrom $kind, /, $type, Mantissa<R, B>, value, ratio)
                    } else {
                        cast_impls!(@tryfrom $kind, *, $type, Mantissa<R, B>, value, ratio)
                    }))
                }
            }

            impl<R, B, E> TryCast<Fix<R, B, E>> for $type
            where
                R: Radix<B>,
                B: Digits,
                E: Exponent,
                $type: TryCast<Mantissa<R, B>>,
            {
                fn try_cast(Fix { bits: value, .. }: Fix<R, B, E>) -> Result<$type> {
                    // radix^|exp|
                    let ratio = R::ratio(E::I32.unsigned_abs());
                    // TODO: Add rounding
                    Ok(if 0 < E::I32 {
                        cast_impls!(@tryinto $kind, *, $type, value, ratio)
                    } else {
                        cast_impls!(@tryinto $kind, /, $type, value, ratio)
                    })
                }
            }
        )*
    };

    (@into int, $op:tt, $type:ident, $value:ident, $ratio:ident) => {
        <$type>::cast($value $op $ratio)
    };

    (@into float, $op:tt, $type:ident, $value:ident, $ratio:ident) => {
        <$type>::cast($value) $op <$type>::cast($ratio)
    };

    (@from int, $op:tt, $type:ident, $bits:ty, $value:ident, $ratio:ident) => {
        <$bits>::cast($value) $op $ratio
    };

    (@from float, $op:tt, $type:ident, $bits:ty, $value:ident, $ratio:ident) => {
        <$bits>::cast($value $op <$type>::cast($ratio))
    };

    (@tryinto int, /, $type:ident, $value:ident, $ratio:ident) => {
        <$type>::try_cast($value / $ratio)?
    };

    (@tryinto int, *, $type:ident, $value:ident, $ratio:ident) => {
        <$type>::try_cast($value.try_mul($ratio)?)?
    };

    (@tryinto float, $op:tt, $type:ident, $value:ident, $ratio:ident) => {
        <$type>::try_cast($value)? $op <$type>::try_cast($ratio)?
    };

    (@tryfrom int, /, $type:ident, $bits:ty, $value:ident, $ratio:ident) => {
        <$bits>::try_cast($value)? / $ratio
    };

    (@tryfrom int, *, $type:ident, $bits:ty, $value:ident, $ratio:ident) => {
        <$bits>::try_cast($value)?.try_mul($ratio)?
    };

    (@tryfrom float, $op:tt, $type:ident, $bits:ty, $value:ident, $ratio:ident) => {
        <$bits>::try_cast($value $op <$type>::try_cast($ratio)?)?
    };
}

cast_impls!(int: u8, u16, u32, u64, i8, i16, i32, i64);
cast_impls!(float: f32, f64);
#[cfg(feature = "i128")]
cast_impls!(int: u128, i128);

impl<R, B, Br, E, Er> Cast<Fix<R, B, E>> for Fix<R, Br, Er>
where
    R: Radix<B> + Radix<Br>,
    B: Digits,
    E: Exponent,
    Br: Digits,
    Er: Exponent,
    Mantissa<R, Br>: Cast<Mantissa<R, B>>,
{
    fn cast(value: Fix<R, B, E>) -> Self {
        value.convert()
    }
}

impl<R, B, Br, E, Er> TryCast<Fix<R, B, E>> for Fix<R, Br, Er>
where
    R: Radix<B> + Radix<Br>,
    B: Digits,
    E: Exponent,
    Br: Digits,
    Er: Exponent,
    Mantissa<R, Br>: TryCast<Mantissa<R, B>>,
{
    fn try_cast(value: Fix<R, B, E>) -> Result<Self> {
        value.try_convert()
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
