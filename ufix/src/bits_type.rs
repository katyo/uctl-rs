pub trait BitsType<Base> {
    type Type: Sized;
}

pub type TypeBits<Bits, Base> = <Bits as BitsType<Base>>::Type;

macro_rules! bits_type {
    ( $($base: ident: $($type: ty: $($bits: ident)+),+;)+ ) => { $($($(
        impl BitsType<typenum::$base> for typenum::$bits {
            type Type = $type;
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

bits_type! {
    U2:
    _8::U: U1 U2 U3 U4 U5 U6 U7 U8,
    _8::I: P1 P2 P3 P4 P5 P6 P7 P8,
    _16::U: U9 U10 U11 U12 U13 U14 U15 U16,
    _16::I: P9 P10 P11 P12 P13 P14 P15 P16,
    _32::U: U17 U18 U19 U20 U21 U22 U23 U24 U25 U26 U27 U28 U29 U30 U31 U32,
    _32::I: P17 P18 P19 P20 P21 P22 P23 P24 P25 P26 P27 P28 P29 P30 P31 P32,
    _64::U: U33 U34 U35 U36 U37 U38 U39 U40 U41 U42 U43 U44 U45 U46 U47 U48 U49 U50 U51 U52 U53 U54 U55 U56 U57 U58 U59 U60 U61 U62 U63 U64,
    _64::I: P33 P34 P35 P36 P37 P38 P39 P40 P41 P42 P43 P44 P45 P46 P47 P48 P49 P50 P51 P52 P53 P54 P55 P56 P57 P58 P59 P60 P61 P62 P63 P64;

    U10:
    _8::U: U1 U2, // 0 .. 255
    _8::I: P1 P2, // -128 .. 127
    _16::U: U3 U4, // 0 .. 65_535
    _16::I: P3 P4, // -32_768 .. 32_767
    _32::U: U5 U6 U7 U8 U9, // 0 .. 4_294_967_295
    _32::I: P5 P6 P7 P8 P9, // -2_147_483_648 .. 2_147_483_647
    _64::U: U10 U11 U12 U13 U14 U15 U16 U17 U18 U19, // 0 .. 18_446_744_073_709_551_615
    _64::I: P10 P11 P12 P13 P14 P15 P16 P17 P18; // -9_223_372_036_854_775_808 .. 9_223_372_036_854_775_807
}

// 128
#[cfg(feature = "i128")]
mod _128 {
    use super::BitsType;

    mod _128 {
        pub type U = u128;
        pub type I = i128;
    }

    bits_type! {
        U2:
        _128::U: U65 U66 U67 U68 U69 U70 U71 U72 U73 U74 U75 U76 U77 U78 U79 U80 U81 U82 U83 U84 U85 U86 U87 U88 U89 U90 U91 U92 U93 U94 U95 U96 U97 U98 U99 U100 U101 U102 U103 U104 U105 U106 U107 U108 U109 U110 U111 U112 U113 U114 U115 U116 U117 U118 U119 U120 U121 U122 U123 U124 U125 U126 U127 U128,
        _128::I: P65 P66 P67 P68 P69 P70 P71 P72 P73 P74 P75 P76 P77 P78 P79 P80 P81 P82 P83 P84 P85 P86 P87 P88 P89 P90 P91 P92 P93 P94 P95 P96 P97 P98 P99 P100 P101 P102 P103 P104 P105 P106 P107 P108 P109 P110 P111 P112 P113 P114 P115 P116 P117 P118 P119 P120 P121 P122 P123 P124 P125 P126 P127 P128;

        U10:
        // 0 .. 340_282_366_920_938_463_463_374_607_431_768_211_455
        _128::U: U20 U21 U22 U23 U24 U25 U26 U27 U28 U29 U30 U31 U32 U33 U34 U35 U36 U37 U38,
        // -170_141_183_460_469_231_731_687_303_715_884_105_728 .. 170_141_183_460_469_231_731_687_303_715_884_105_727
        _128::I: P19 P20 P21 P22 P23 P24 P25 P26 P27 P28 P29 P30 P31 P32 P33 P34 P35 P36 P37 P38;
    }
}

#[cfg(test)]
mod test {
    use super::TypeBits;
    use core::mem::size_of;
    use typenum::*;

    type Bits2<T> = TypeBits<T, U2>;

    #[test]
    fn size_of_type() {
        // 8 bit
        #[cfg(feature = "word8")]
        {
            assert_eq!(size_of::<Bits2<P1>>(), 1);
            assert_eq!(size_of::<Bits2<P4>>(), 1);
            assert_eq!(size_of::<Bits2<P8>>(), 1);
        }

        #[cfg(all(not(feature = "word8"), feature = "word16"))]
        {
            assert_eq!(size_of::<Bits2<P1>>(), 2);
            assert_eq!(size_of::<Bits2<P4>>(), 2);
            assert_eq!(size_of::<Bits2<P8>>(), 2);
        }

        #[cfg(not(any(feature = "word8", feature = "word16")))]
        {
            assert_eq!(size_of::<Bits2<P1>>(), 4);
            assert_eq!(size_of::<Bits2<P4>>(), 4);
            assert_eq!(size_of::<Bits2<P8>>(), 4);
        }

        // 16 bit
        #[cfg(any(feature = "word8", feature = "word16"))]
        {
            assert_eq!(size_of::<Bits2<P9>>(), 2);
            assert_eq!(size_of::<Bits2<P12>>(), 2);
            assert_eq!(size_of::<Bits2<P16>>(), 2);
        }

        #[cfg(not(any(feature = "word8", feature = "word16")))]
        {
            assert_eq!(size_of::<Bits2<P9>>(), 4);
            assert_eq!(size_of::<Bits2<P12>>(), 4);
            assert_eq!(size_of::<Bits2<P16>>(), 4);
        }

        // 32 bit
        assert_eq!(size_of::<Bits2<P17>>(), 4);
        assert_eq!(size_of::<Bits2<P24>>(), 4);
        assert_eq!(size_of::<Bits2<P32>>(), 4);

        // 64 bit
        assert_eq!(size_of::<Bits2<P33>>(), 8);
        assert_eq!(size_of::<Bits2<P45>>(), 8);
        assert_eq!(size_of::<Bits2<P64>>(), 8);

        #[cfg(feature = "i128")]
        {
            assert_eq!(size_of::<Bits2<P65>>(), 16);
            assert_eq!(size_of::<Bits2<P88>>(), 16);
            assert_eq!(size_of::<Bits2<P128>>(), 16);
        }

        assert_eq!(size_of::<Bits2<P1>>(), size_of::<Bits2<P3>>());
        assert_eq!(size_of::<Bits2<P4>>(), size_of::<Bits2<P8>>());
        assert_eq!(size_of::<Bits2<P3>>(), size_of::<Bits2<P7>>());

        assert_eq!(size_of::<Bits2<P9>>(), size_of::<Bits2<P12>>());
        assert_eq!(size_of::<Bits2<P11>>(), size_of::<Bits2<P16>>());
        assert_eq!(size_of::<Bits2<P12>>(), size_of::<Bits2<P15>>());

        assert_eq!(size_of::<Bits2<P17>>(), size_of::<Bits2<P24>>());
    }
}
