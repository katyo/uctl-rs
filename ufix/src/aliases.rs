use super::Fix;

/// Base-2 types.
pub mod bin {
    use typenum::*;

    pub type Fix<B, E> = super::Fix<U2, B, E>;

    pub type UFix8<E> = Fix<U8, E>;
    pub type UFix16<E> = Fix<U16, E>;
    pub type UFix32<E> = Fix<U32, E>;
    pub type UFix64<E> = Fix<U64, E>;

    #[cfg(feature = "i128")]
    pub type UFix128<E> = Fix<U128, E>;

    pub type IFix8<E> = Fix<P8, E>;
    pub type IFix16<E> = Fix<P16, E>;
    pub type IFix32<E> = Fix<P32, E>;
    pub type IFix64<E> = Fix<P64, E>;

    #[cfg(feature = "i128")]
    pub type IFix128<E> = Fix<P128, E>;
}

/// Base-10 types.
pub mod dec {
    use typenum::*;

    pub type Fix<B, E> = super::Fix<U10, B, E>;

    pub type UFix8<E> = Fix<U8, E>;
    pub type UFix16<E> = Fix<U16, E>;
    pub type UFix32<E> = Fix<U32, E>;
    pub type UFix64<E> = Fix<U64, E>;

    #[cfg(feature = "i128")]
    pub type UFix128<E> = Fix<U128, E>;

    pub type IFix8<E> = Fix<P8, E>;
    pub type IFix16<E> = Fix<P16, E>;
    pub type IFix32<E> = Fix<P32, E>;
    pub type IFix64<E> = Fix<P64, E>;

    #[cfg(feature = "i128")]
    pub type IFix128<E> = Fix<P128, E>;
}

/// SI prefixes.
pub mod si {
    use super::dec::Fix;
    use typenum::{
        N1, N12, N15, N18, N2, N21, N24, N3, N6, N9, P1, P12, P15, P18, P2, P21, P24, P3, P6, P9,
        Z0,
    };

    /** 10<sup>-24</sup> */
    pub type Yocto<B> = Fix<B, N24>;
    /** 10<sup>-21</sup> */
    pub type Zepto<B> = Fix<B, N21>;
    /** 10<sup>-18</sup> */
    pub type Atto<B> = Fix<B, N18>;
    /** 10<sup>-15</sup> */
    pub type Femto<B> = Fix<B, N15>;
    /** 10<sup>-12</sup> */
    pub type Pico<B> = Fix<B, N12>;
    /** 10<sup>-9</sup> */
    pub type Nano<B> = Fix<B, N9>;
    /** 10<sup>-6</sup> */
    pub type Micro<B> = Fix<B, N6>;
    /** 10<sup>-3</sup> */
    pub type Milli<B> = Fix<B, N3>;
    /** 10<sup>-2</sup> */
    pub type Centi<B> = Fix<B, N2>;
    /** 10<sup>-1</sup> */
    pub type Deci<B> = Fix<B, N1>;

    /** 10<sup>0</sup> */
    pub type Unit<B> = Fix<B, Z0>;

    /** 10<sup>1</sup> */
    pub type Deca<B> = Fix<B, P1>;
    /** 10<sup>2</sup> */
    pub type Hecto<B> = Fix<B, P2>;
    /** 10<sup>3</sup> */
    pub type Kilo<B> = Fix<B, P3>;
    /** 10<sup>6</sup> */
    pub type Mega<B> = Fix<B, P6>;
    /** 10<sup>9</sup> */
    pub type Giga<B> = Fix<B, P9>;
    /** 10<sup>12</sup> */
    pub type Tera<B> = Fix<B, P12>;
    /** 10<sup>15</sup> */
    pub type Peta<B> = Fix<B, P15>;
    /** 10<sup>18</sup> */
    pub type Exa<B> = Fix<B, P18>;
    /** 10<sup>21</sup> */
    pub type Zeta<B> = Fix<B, P21>;
    /** 10<sup>24</sup> */
    pub type Yotta<B> = Fix<B, P24>;
}

/// IEC prefixes.
pub mod iec {
    use super::bin::Fix;
    use typenum::{P10, P20, P30, P40, P50, P60, P70, P80, Z0};

    /** 2<sup>0</sup> */
    pub type Unit<B> = Fix<B, Z0>;

    /** 2<sup>10</sup> */
    pub type Kibi<B> = Fix<B, P10>;
    /** 2<sup>20</sup> */
    pub type Mebi<B> = Fix<B, P20>;
    /** 2<sup>30</sup> */
    pub type Gibi<B> = Fix<B, P30>;
    /** 2<sup>40</sup> */
    pub type Tebi<B> = Fix<B, P40>;
    /** 2<sup>50</sup> */
    pub type Pebi<B> = Fix<B, P50>;
    /** 2<sup>60</sup> */
    pub type Exbi<B> = Fix<B, P60>;
    /** 2<sup>70</sup> */
    pub type Zebi<B> = Fix<B, P70>;
    /** 2<sup>80</sup> */
    pub type Yobi<B> = Fix<B, P80>;
}
