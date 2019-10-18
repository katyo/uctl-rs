use super::Fix;

/// Base-2 types.
pub mod bin {
    use typenum::U2;
    use super::Fix as Fix_;

    pub type Fix<Bits, Exp> = Fix_<Bits, U2, Exp>;

    pub type UFix8<Exp> = Fix<u8, Exp>;
    pub type UFix16<Exp> = Fix<u16, Exp>;
    pub type UFix32<Exp> = Fix<u32, Exp>;
    pub type UFix64<Exp> = Fix<u64, Exp>;
    pub type UFixSize<Exp> = Fix<usize, Exp>;

    #[cfg(feature = "i128")]
    pub type UFix128<Exp> = Fix<u128, Exp>;

    pub type IFix8<Exp> = Fix<i8, Exp>;
    pub type IFix16<Exp> = Fix<i16, Exp>;
    pub type IFix32<Exp> = Fix<i32, Exp>;
    pub type IFix64<Exp> = Fix<i64, Exp>;
    pub type IFixSize<Exp> = Fix<isize, Exp>;

    #[cfg(feature = "i128")]
    pub type IFix128<Exp> = Fix<i128, Exp>;
}

/// Base-10 types.
pub mod dec {
    use typenum::U10;

    use super::Fix as Fix_;

    pub type Fix<Bits, Exp> = Fix_<Bits, U10, Exp>;

    pub type UFix8<Exp> = Fix<u8, Exp>;
    pub type UFix16<Exp> = Fix<u16, Exp>;
    pub type UFix32<Exp> = Fix<u32, Exp>;
    pub type UFix64<Exp> = Fix<u64, Exp>;
    pub type UFixSize<Exp> = Fix<usize, Exp>;

    #[cfg(feature = "i128")]
    pub type UFix128<Exp> = Fix<u128, Exp>;

    pub type IFix8<Exp> = Fix<i8, Exp>;
    pub type IFix16<Exp> = Fix<i16, Exp>;
    pub type IFix32<Exp> = Fix<i32, Exp>;
    pub type IFix64<Exp> = Fix<i64, Exp>;
    pub type IFixSize<Exp> = Fix<isize, Exp>;

    #[cfg(feature = "i128")]
    pub type IFix128<Exp> = Fix<i128, Exp>;
}

/// SI prefixes.
pub mod si {
    use typenum::{
        N1, N2, N3, N6, N9, N12, N15, N18, N21, N24,
        P1, P2, P3, P6, P9, P12, P15, P18, P21, P24,
        Z0,
    };
    use super::dec::Fix;

    /** 10<sup>-24</sup> */ pub type Yocto<Bits> = Fix<Bits, N24>;
    /** 10<sup>-21</sup> */ pub type Zepto<Bits> = Fix<Bits, N21>;
    /** 10<sup>-18</sup> */ pub type Atto<Bits> = Fix<Bits, N18>;
    /** 10<sup>-15</sup> */ pub type Femto<Bits> = Fix<Bits, N15>;
    /** 10<sup>-12</sup> */ pub type Pico<Bits> = Fix<Bits, N12>;
    /** 10<sup>-9</sup> */ pub type Nano<Bits> = Fix<Bits, N9>;
    /** 10<sup>-6</sup> */ pub type Micro<Bits> = Fix<Bits, N6>;
    /** 10<sup>-3</sup> */ pub type Milli<Bits> = Fix<Bits, N3>;
    /** 10<sup>-2</sup> */ pub type Centi<Bits> = Fix<Bits, N2>;
    /** 10<sup>-1</sup> */ pub type Deci<Bits> = Fix<Bits, N1>;

    /** 10<sup>0</sup> */ pub type Unit<Bits> = Fix<Bits, Z0>;

    /** 10<sup>1</sup> */ pub type Deca<Bits> = Fix<Bits, P1>;
    /** 10<sup>2</sup> */ pub type Hecto<Bits> = Fix<Bits, P2>;
    /** 10<sup>3</sup> */ pub type Kilo<Bits> = Fix<Bits, P3>;
    /** 10<sup>6</sup> */ pub type Mega<Bits> = Fix<Bits, P6>;
    /** 10<sup>9</sup> */ pub type Giga<Bits> = Fix<Bits, P9>;
    /** 10<sup>12</sup> */ pub type Tera<Bits> = Fix<Bits, P12>;
    /** 10<sup>15</sup> */ pub type Peta<Bits> = Fix<Bits, P15>;
    /** 10<sup>18</sup> */ pub type Exa<Bits> = Fix<Bits, P18>;
    /** 10<sup>21</sup> */ pub type Zeta<Bits> = Fix<Bits, P21>;
    /** 10<sup>24</sup> */ pub type Yotta<Bits> = Fix<Bits, P24>;
}

/// IEC prefixes.
pub mod iec {
    use typenum::{
        P10, P20, P30, P40, P50, P60, P70, P80,
        Z0,
    };

    use super::bin::Fix;

    /** 2<sup>0</sup> */ pub type Unit<Bits> = Fix<Bits, Z0>;

    /** 2<sup>10</sup> */ pub type Kibi<Bits> = Fix<Bits, P10>;
    /** 2<sup>20</sup> */ pub type Mebi<Bits> = Fix<Bits, P20>;
    /** 2<sup>30</sup> */ pub type Gibi<Bits> = Fix<Bits, P30>;
    /** 2<sup>40</sup> */ pub type Tebi<Bits> = Fix<Bits, P40>;
    /** 2<sup>50</sup> */ pub type Pebi<Bits> = Fix<Bits, P50>;
    /** 2<sup>60</sup> */ pub type Exbi<Bits> = Fix<Bits, P60>;
    /** 2<sup>70</sup> */ pub type Zebi<Bits> = Fix<Bits, P70>;
    /** 2<sup>80</sup> */ pub type Yobi<Bits> = Fix<Bits, P80>;
}
