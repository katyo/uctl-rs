pub trait BitsType {
    type Type;
}

/*
macro_rules! type_type {
    ($type: tt) => {
        impl super::BitsType for $type {
            type Type = $type;
        }
    }
}

type_type!(u8);
type_type!(u16);
type_type!(u32);
type_type!(u64);
#[cfg(feature = "i128")]
type_type!(u128);
type_type!(usize);

type_type!(i8);
type_type!(i16);
type_type!(i32);
type_type!(i64);
#[cfg(feature = "i128")]
type_type!(i128);
type_type!(isize);
*/

macro_rules! bits_type {
    ($bits: tt, $type: tt) => {
        impl super::BitsType for typenum::$bits {
            type Type = $type;
        }
    }
}

// 8
#[cfg(feature = "word8")]
mod _bw_8 {
    bits_type!(U1, u8);
    bits_type!(U2, u8);
    bits_type!(U3, u8);
    bits_type!(U4, u8);
    bits_type!(U5, u8);
    bits_type!(U6, u8);
    bits_type!(U7, u8);
    bits_type!(U8, u8);

    bits_type!(P1, i8);
    bits_type!(P2, i8);
    bits_type!(P3, i8);
    bits_type!(P4, i8);
    bits_type!(P5, i8);
    bits_type!(P6, i8);
    bits_type!(P7, i8);
    bits_type!(P8, i8);
}

#[cfg(all(not(feature = "word8"), feature = "word16"))]
mod _bw_8 {
    bits_type!(U1, u16);
    bits_type!(U2, u16);
    bits_type!(U3, u16);
    bits_type!(U4, u16);
    bits_type!(U5, u16);
    bits_type!(U6, u16);
    bits_type!(U7, u16);
    bits_type!(U8, u16);

    bits_type!(P1, i16);
    bits_type!(P2, i16);
    bits_type!(P3, i16);
    bits_type!(P4, i16);
    bits_type!(P5, i16);
    bits_type!(P6, i16);
    bits_type!(P7, i16);
    bits_type!(P8, i16);
}

#[cfg(not(any(feature = "word8", feature = "word16")))]
mod _bw_8 {
    bits_type!(U1, u32);
    bits_type!(U2, u32);
    bits_type!(U3, u32);
    bits_type!(U4, u32);
    bits_type!(U5, u32);
    bits_type!(U6, u32);
    bits_type!(U7, u32);
    bits_type!(U8, u32);

    bits_type!(P1, i32);
    bits_type!(P2, i32);
    bits_type!(P3, i32);
    bits_type!(P4, i32);
    bits_type!(P5, i32);
    bits_type!(P6, i32);
    bits_type!(P7, i32);
    bits_type!(P8, i32);
}

// 16
#[cfg(any(feature = "word8", feature = "word16"))]
mod _bw_16 {
    bits_type!(U9, u16);
    bits_type!(U10, u16);
    bits_type!(U11, u16);
    bits_type!(U12, u16);
    bits_type!(U13, u16);
    bits_type!(U14, u16);
    bits_type!(U15, u16);
    bits_type!(U16, u16);

    bits_type!(P9, i16);
    bits_type!(P10, i16);
    bits_type!(P11, i16);
    bits_type!(P12, i16);
    bits_type!(P13, i16);
    bits_type!(P14, i16);
    bits_type!(P15, i16);
    bits_type!(P16, i16);
}

#[cfg(not(any(feature = "word8", feature = "word16")))]
mod _bw_16 {
    bits_type!(U9, u32);
    bits_type!(U10, u32);
    bits_type!(U11, u32);
    bits_type!(U12, u32);
    bits_type!(U13, u32);
    bits_type!(U14, u32);
    bits_type!(U15, u32);
    bits_type!(U16, u32);

    bits_type!(P9, i32);
    bits_type!(P10, i32);
    bits_type!(P11, i32);
    bits_type!(P12, i32);
    bits_type!(P13, i32);
    bits_type!(P14, i32);
    bits_type!(P15, i32);
    bits_type!(P16, i32);
}

// 32
mod _bw_32 {
    bits_type!(U17, u32);
    bits_type!(U18, u32);
    bits_type!(U19, u32);
    bits_type!(U20, u32);
    bits_type!(U21, u32);
    bits_type!(U22, u32);
    bits_type!(U23, u32);
    bits_type!(U24, u32);
    bits_type!(U25, u32);
    bits_type!(U26, u32);
    bits_type!(U27, u32);
    bits_type!(U28, u32);
    bits_type!(U29, u32);
    bits_type!(U30, u32);
    bits_type!(U31, u32);
    bits_type!(U32, u32);

    bits_type!(P17, i32);
    bits_type!(P18, i32);
    bits_type!(P19, i32);
    bits_type!(P20, i32);
    bits_type!(P21, i32);
    bits_type!(P22, i32);
    bits_type!(P23, i32);
    bits_type!(P24, i32);
    bits_type!(P25, i32);
    bits_type!(P26, i32);
    bits_type!(P27, i32);
    bits_type!(P28, i32);
    bits_type!(P29, i32);
    bits_type!(P30, i32);
    bits_type!(P31, i32);
    bits_type!(P32, i32);
}

// 64
mod _bw_64 {
    bits_type!(U33, u64);
    bits_type!(U34, u64);
    bits_type!(U35, u64);
    bits_type!(U36, u64);
    bits_type!(U37, u64);
    bits_type!(U38, u64);
    bits_type!(U39, u64);
    bits_type!(U40, u64);
    bits_type!(U41, u64);
    bits_type!(U42, u64);
    bits_type!(U43, u64);
    bits_type!(U44, u64);
    bits_type!(U45, u64);
    bits_type!(U46, u64);
    bits_type!(U47, u64);
    bits_type!(U48, u64);
    bits_type!(U49, u64);
    bits_type!(U50, u64);
    bits_type!(U51, u64);
    bits_type!(U52, u64);
    bits_type!(U53, u64);
    bits_type!(U54, u64);
    bits_type!(U55, u64);
    bits_type!(U56, u64);
    bits_type!(U57, u64);
    bits_type!(U58, u64);
    bits_type!(U59, u64);
    bits_type!(U60, u64);
    bits_type!(U61, u64);
    bits_type!(U62, u64);
    bits_type!(U63, u64);
    bits_type!(U64, u64);

    bits_type!(P33, i64);
    bits_type!(P34, i64);
    bits_type!(P35, i64);
    bits_type!(P36, i64);
    bits_type!(P37, i64);
    bits_type!(P38, i64);
    bits_type!(P39, i64);
    bits_type!(P40, i64);
    bits_type!(P41, i64);
    bits_type!(P42, i64);
    bits_type!(P43, i64);
    bits_type!(P44, i64);
    bits_type!(P45, i64);
    bits_type!(P46, i64);
    bits_type!(P47, i64);
    bits_type!(P48, i64);
    bits_type!(P49, i64);
    bits_type!(P50, i64);
    bits_type!(P51, i64);
    bits_type!(P52, i64);
    bits_type!(P53, i64);
    bits_type!(P54, i64);
    bits_type!(P55, i64);
    bits_type!(P56, i64);
    bits_type!(P57, i64);
    bits_type!(P58, i64);
    bits_type!(P59, i64);
    bits_type!(P60, i64);
    bits_type!(P61, i64);
    bits_type!(P62, i64);
    bits_type!(P63, i64);
    bits_type!(P64, i64);
}

// 128
#[cfg(feature = "i128")]
mod _bw_128 {
    bits_type!(U65, u128);
    bits_type!(U66, u128);
    bits_type!(U67, u128);
    bits_type!(U68, u128);
    bits_type!(U69, u128);
    bits_type!(U70, u128);
    bits_type!(U71, u128);
    bits_type!(U72, u128);
    bits_type!(U73, u128);
    bits_type!(U74, u128);
    bits_type!(U75, u128);
    bits_type!(U76, u128);
    bits_type!(U77, u128);
    bits_type!(U78, u128);
    bits_type!(U79, u128);
    bits_type!(U80, u128);
    bits_type!(U81, u128);
    bits_type!(U82, u128);
    bits_type!(U83, u128);
    bits_type!(U84, u128);
    bits_type!(U85, u128);
    bits_type!(U86, u128);
    bits_type!(U87, u128);
    bits_type!(U88, u128);
    bits_type!(U89, u128);
    bits_type!(U90, u128);
    bits_type!(U91, u128);
    bits_type!(U92, u128);
    bits_type!(U93, u128);
    bits_type!(U94, u128);
    bits_type!(U95, u128);
    bits_type!(U96, u128);
    bits_type!(U97, u128);
    bits_type!(U98, u128);
    bits_type!(U99, u128);
    bits_type!(U100, u128);
    bits_type!(U101, u128);
    bits_type!(U102, u128);
    bits_type!(U103, u128);
    bits_type!(U104, u128);
    bits_type!(U105, u128);
    bits_type!(U106, u128);
    bits_type!(U107, u128);
    bits_type!(U108, u128);
    bits_type!(U109, u128);
    bits_type!(U110, u128);
    bits_type!(U111, u128);
    bits_type!(U112, u128);
    bits_type!(U113, u128);
    bits_type!(U114, u128);
    bits_type!(U115, u128);
    bits_type!(U116, u128);
    bits_type!(U117, u128);
    bits_type!(U118, u128);
    bits_type!(U119, u128);
    bits_type!(U120, u128);
    bits_type!(U121, u128);
    bits_type!(U122, u128);
    bits_type!(U123, u128);
    bits_type!(U124, u128);
    bits_type!(U125, u128);
    bits_type!(U126, u128);
    bits_type!(U127, u128);
    bits_type!(U128, u128);

    bits_type!(P65, i128);
    bits_type!(P66, i128);
    bits_type!(P67, i128);
    bits_type!(P68, i128);
    bits_type!(P69, i128);
    bits_type!(P70, i128);
    bits_type!(P71, i128);
    bits_type!(P72, i128);
    bits_type!(P73, i128);
    bits_type!(P74, i128);
    bits_type!(P75, i128);
    bits_type!(P76, i128);
    bits_type!(P77, i128);
    bits_type!(P78, i128);
    bits_type!(P79, i128);
    bits_type!(P80, i128);
    bits_type!(P81, i128);
    bits_type!(P82, i128);
    bits_type!(P83, i128);
    bits_type!(P84, i128);
    bits_type!(P85, i128);
    bits_type!(P86, i128);
    bits_type!(P87, i128);
    bits_type!(P88, i128);
    bits_type!(P89, i128);
    bits_type!(P90, i128);
    bits_type!(P91, i128);
    bits_type!(P92, i128);
    bits_type!(P93, i128);
    bits_type!(P94, i128);
    bits_type!(P95, i128);
    bits_type!(P96, i128);
    bits_type!(P97, i128);
    bits_type!(P98, i128);
    bits_type!(P99, i128);
    bits_type!(P100, i128);
    bits_type!(P101, i128);
    bits_type!(P102, i128);
    bits_type!(P103, i128);
    bits_type!(P104, i128);
    bits_type!(P105, i128);
    bits_type!(P106, i128);
    bits_type!(P107, i128);
    bits_type!(P108, i128);
    bits_type!(P109, i128);
    bits_type!(P110, i128);
    bits_type!(P111, i128);
    bits_type!(P112, i128);
    bits_type!(P113, i128);
    bits_type!(P114, i128);
    bits_type!(P115, i128);
    bits_type!(P116, i128);
    bits_type!(P117, i128);
    bits_type!(P118, i128);
    bits_type!(P119, i128);
    bits_type!(P120, i128);
    bits_type!(P121, i128);
    bits_type!(P122, i128);
    bits_type!(P123, i128);
    bits_type!(P124, i128);
    bits_type!(P125, i128);
    bits_type!(P126, i128);
    bits_type!(P127, i128);
    bits_type!(P128, i128);
}

#[cfg(test)]
mod test {
    use super::BitsType;
    use typenum::*;
    use core::mem::size_of;

    #[test]
    fn size_of_type() {
        // 8 bit
        #[cfg(feature = "word8")]
        {
            assert_eq!(size_of::<<P1 as BitsType>::Type>(), 1);
            assert_eq!(size_of::<<P4 as BitsType>::Type>(), 1);
            assert_eq!(size_of::<<P8 as BitsType>::Type>(), 1);
        }

        #[cfg(all(not(feature = "word8"), feature = "word16"))]
        {
            assert_eq!(size_of::<<P1 as BitsType>::Type>(), 2);
            assert_eq!(size_of::<<P4 as BitsType>::Type>(), 2);
            assert_eq!(size_of::<<P8 as BitsType>::Type>(), 2);
        }

        #[cfg(not(any(feature = "word8", feature = "word16")))]
        {
            assert_eq!(size_of::<<P1 as BitsType>::Type>(), 4);
            assert_eq!(size_of::<<P4 as BitsType>::Type>(), 4);
            assert_eq!(size_of::<<P8 as BitsType>::Type>(), 4);
        }

        // 16 bit
        #[cfg(any(feature = "word8", feature = "word16"))]
        {
            assert_eq!(size_of::<<P9 as BitsType>::Type>(), 2);
            assert_eq!(size_of::<<P12 as BitsType>::Type>(), 2);
            assert_eq!(size_of::<<P16 as BitsType>::Type>(), 2);
        }

        #[cfg(not(any(feature = "word8", feature = "word16")))]
        {
            assert_eq!(size_of::<<P9 as BitsType>::Type>(), 4);
            assert_eq!(size_of::<<P12 as BitsType>::Type>(), 4);
            assert_eq!(size_of::<<P16 as BitsType>::Type>(), 4);
        }

        // 32 bit
        assert_eq!(size_of::<<P17 as BitsType>::Type>(), 4);
        assert_eq!(size_of::<<P24 as BitsType>::Type>(), 4);
        assert_eq!(size_of::<<P32 as BitsType>::Type>(), 4);

        // 64 bit
        assert_eq!(size_of::<<P33 as BitsType>::Type>(), 8);
        assert_eq!(size_of::<<P45 as BitsType>::Type>(), 8);
        assert_eq!(size_of::<<P64 as BitsType>::Type>(), 8);

        #[cfg(feature = "i128")]
        {
            assert_eq!(size_of::<<P65 as BitsType>::Type>(), 16);
            assert_eq!(size_of::<<P88 as BitsType>::Type>(), 16);
            assert_eq!(size_of::<<P128 as BitsType>::Type>(), 16);
        }

        assert_eq!(size_of::<<P1 as BitsType>::Type>(), size_of::<<P3 as BitsType>::Type>());
        assert_eq!(size_of::<<P4 as BitsType>::Type>(), size_of::<<P8 as BitsType>::Type>());
        assert_eq!(size_of::<<P3 as BitsType>::Type>(), size_of::<<P7 as BitsType>::Type>());

        assert_eq!(size_of::<<P9 as BitsType>::Type>(), size_of::<<P12 as BitsType>::Type>());
        assert_eq!(size_of::<<P11 as BitsType>::Type>(), size_of::<<P16 as BitsType>::Type>());
        assert_eq!(size_of::<<P12 as BitsType>::Type>(), size_of::<<P15 as BitsType>::Type>());

        assert_eq!(size_of::<<P17 as BitsType>::Type>(), size_of::<<P24 as BitsType>::Type>());
    }
}
