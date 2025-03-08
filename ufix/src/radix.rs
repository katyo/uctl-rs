use crate::{FromPositive, Positive, UnsignedPow};
use core::ops::{Div, Mul};

/// The trait which infers type for store the value according to given radix parameter
pub trait Radix<B>: Positive + Copy {
    /// The integer type which can hold required number of digits with this radix
    type Type: Sized
        + Copy
        + FromPositive
        + UnsignedPow
        + Mul<Output = Self::Type>
        + Div<Output = Self::Type>;

    /// Minimum value of integer type
    const MIN: Self::Type;
    /// Maximum value of integer type
    const MAX: Self::Type;

    /// Get ratio of integer type which can be used to adjust mantissa value with given exponent
    fn ratio(exp: u32) -> Self::Type
    where
        Self: Sized,
    {
        Self::Type::from_positive::<Self>().unsigned_pow(exp)
    }
}

/// The mantissa type for given radix and number of digits
///
/// Alias of `<R as Radix<B>>::Type`.
pub type Mantissa<R, B> = <R as Radix<B>>::Type;

macro_rules! radix_impl {
    ( $($radix: ident: $($type: ty: $($width: ident)+),+;)+ ) => { $($($(
        impl Radix<typenum::$width> for typenum::$radix {
            type Type = $type;
            const MIN: Self::Type = <$type>::MIN;
            const MAX: Self::Type = <$type>::MAX;
        }
    )+)+)+ };
}

#[cfg(feature = "word8")]
mod _8 {
    pub type U = u8;
    pub type I = i8;
}

#[cfg(all(not(feature = "word8"), feature = "word16"))]
mod _8 {
    pub type U = u16;
    pub type I = i16;
}

#[cfg(all(not(feature = "word8"), not(feature = "word16")))]
mod _8 {
    pub type U = u32;
    pub type I = i32;
}

#[cfg(any(feature = "word8", feature = "word16"))]
mod _16 {
    pub type U = u16;
    pub type I = i16;
}

#[cfg(not(any(feature = "word8", feature = "word16")))]
mod _16 {
    pub type U = u32;
    pub type I = i32;
}

mod _32 {
    pub type U = u32;
    pub type I = i32;
}

mod _64 {
    pub type U = u64;
    pub type I = i64;
}

radix_impl! {
    U2:
    _8::U: P1 P2 P3 P4 P5 P6 P7 P8,
    _16::U: P9 P10 P11 P12 P13 P14 P15 P16,
    _32::U: P17 P18 P19 P20 P21 P22 P23 P24 P25 P26 P27 P28 P29 P30 P31 P32,
    _64::U: P33 P34 P35 P36 P37 P38 P39 P40 P41 P42 P43 P44 P45 P46 P47 P48 P49 P50 P51 P52 P53 P54 P55 P56 P57 P58 P59 P60 P61 P62 P63 P64;

    P2:
    _8::I: P1 P2 P3 P4 P5 P6 P7 P8,
    _16::I: P9 P10 P11 P12 P13 P14 P15 P16,
    _32::I: P17 P18 P19 P20 P21 P22 P23 P24 P25 P26 P27 P28 P29 P30 P31 P32,
    _64::I: P33 P34 P35 P36 P37 P38 P39 P40 P41 P42 P43 P44 P45 P46 P47 P48 P49 P50 P51 P52 P53 P54 P55 P56 P57 P58 P59 P60 P61 P62 P63 P64;

    U10:
    _8::U: P1 P2, // 0 .. 255
    _16::U: P3 P4, // 0 .. 65_535
    _32::U: P5 P6 P7 P8 P9, // 0 .. 4_294_967_295
    _64::U: P10 P11 P12 P13 P14 P15 P16 P17 P18 P19; // 0 .. 18_446_744_073_709_551_615

    P10:
    _8::I: P1 P2, // -128 .. 127
    _16::I: P3 P4, // -32_768 .. 32_767
    _32::I: P5 P6 P7 P8 P9, // -2_147_483_648 .. 2_147_483_647
    _64::I: P10 P11 P12 P13 P14 P15 P16 P17 P18; // -9_223_372_036_854_775_808 .. 9_223_372_036_854_775_807
}

// 128
#[cfg(feature = "i128")]
mod _128 {
    use super::Radix;

    mod _128 {
        pub type U = u128;
        pub type I = i128;
    }

    radix_impl! {
        U2:
        _128::U: P65 P66 P67 P68 P69 P70 P71 P72 P73 P74 P75 P76 P77 P78 P79 P80 P81 P82 P83 P84 P85 P86 P87 P88 P89 P90 P91 P92 P93 P94 P95 P96 P97 P98 P99 P100 P101 P102 P103 P104 P105 P106 P107 P108 P109 P110 P111 P112 P113 P114 P115 P116 P117 P118 P119 P120 P121 P122 P123 P124 P125 P126 P127 P128;

        P2:
        _128::I: P65 P66 P67 P68 P69 P70 P71 P72 P73 P74 P75 P76 P77 P78 P79 P80 P81 P82 P83 P84 P85 P86 P87 P88 P89 P90 P91 P92 P93 P94 P95 P96 P97 P98 P99 P100 P101 P102 P103 P104 P105 P106 P107 P108 P109 P110 P111 P112 P113 P114 P115 P116 P117 P118 P119 P120 P121 P122 P123 P124 P125 P126 P127 P128;

        U10:
        // 0 .. 340_282_366_920_938_463_463_374_607_431_768_211_455
        _128::U: P20 P21 P22 P23 P24 P25 P26 P27 P28 P29 P30 P31 P32 P33 P34 P35 P36 P37 P38;

        P10:
        // -170_141_183_460_469_231_731_687_303_715_884_105_728 .. 170_141_183_460_469_231_731_687_303_715_884_105_727
        _128::I: P19 P20 P21 P22 P23 P24 P25 P26 P27 P28 P29 P30 P31 P32 P33 P34 P35 P36 P37 P38;
    }
}

#[cfg(test)]
mod test {
    use super::Mantissa;
    use core::mem::size_of;
    use typenum::*;

    type Mantissa2<T> = Mantissa<U2, T>;

    #[test]
    fn size_of_type() {
        // 8 bit
        #[cfg(feature = "word8")]
        {
            assert_eq!(size_of::<Mantissa2<P1>>(), 1);
            assert_eq!(size_of::<Mantissa2<P4>>(), 1);
            assert_eq!(size_of::<Mantissa2<P8>>(), 1);
        }

        #[cfg(all(not(feature = "word8"), feature = "word16"))]
        {
            assert_eq!(size_of::<Mantissa2<P1>>(), 2);
            assert_eq!(size_of::<Mantissa2<P4>>(), 2);
            assert_eq!(size_of::<Mantissa2<P8>>(), 2);
        }

        #[cfg(not(any(feature = "word8", feature = "word16")))]
        {
            assert_eq!(size_of::<Mantissa2<P1>>(), 4);
            assert_eq!(size_of::<Mantissa2<P4>>(), 4);
            assert_eq!(size_of::<Mantissa2<P8>>(), 4);
        }

        // 16 bit
        #[cfg(any(feature = "word8", feature = "word16"))]
        {
            assert_eq!(size_of::<Mantissa2<P9>>(), 2);
            assert_eq!(size_of::<Mantissa2<P12>>(), 2);
            assert_eq!(size_of::<Mantissa2<P16>>(), 2);
        }

        #[cfg(not(any(feature = "word8", feature = "word16")))]
        {
            assert_eq!(size_of::<Mantissa2<P9>>(), 4);
            assert_eq!(size_of::<Mantissa2<P12>>(), 4);
            assert_eq!(size_of::<Mantissa2<P16>>(), 4);
        }

        // 32 bit
        assert_eq!(size_of::<Mantissa2<P17>>(), 4);
        assert_eq!(size_of::<Mantissa2<P24>>(), 4);
        assert_eq!(size_of::<Mantissa2<P32>>(), 4);

        // 64 bit
        assert_eq!(size_of::<Mantissa2<P33>>(), 8);
        assert_eq!(size_of::<Mantissa2<P45>>(), 8);
        assert_eq!(size_of::<Mantissa2<P64>>(), 8);

        #[cfg(feature = "i128")]
        {
            assert_eq!(size_of::<Mantissa2<P65>>(), 16);
            assert_eq!(size_of::<Mantissa2<P88>>(), 16);
            assert_eq!(size_of::<Mantissa2<P128>>(), 16);
        }

        assert_eq!(size_of::<Mantissa2<P1>>(), size_of::<Mantissa2<P3>>());
        assert_eq!(size_of::<Mantissa2<P4>>(), size_of::<Mantissa2<P8>>());
        assert_eq!(size_of::<Mantissa2<P3>>(), size_of::<Mantissa2<P7>>());

        assert_eq!(size_of::<Mantissa2<P9>>(), size_of::<Mantissa2<P12>>());
        assert_eq!(size_of::<Mantissa2<P11>>(), size_of::<Mantissa2<P16>>());
        assert_eq!(size_of::<Mantissa2<P12>>(), size_of::<Mantissa2<P15>>());

        assert_eq!(size_of::<Mantissa2<P17>>(), size_of::<Mantissa2<P24>>());
    }
}
