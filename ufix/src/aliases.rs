use super::Fix;

/// Base-2 types.
pub mod bin {
    use typenum::*;

    pub type UFix<B, E> = super::Fix<U2, B, E>;
    pub type Fix<B, E> = super::Fix<P2, B, E>;

    pub type UFix8<E> = UFix<P8, E>;
    pub type UFix16<E> = UFix<P16, E>;
    pub type UFix32<E> = UFix<P32, E>;
    pub type UFix64<E> = UFix<P64, E>;

    #[cfg(feature = "i128")]
    pub type UFix128<E> = UFix<P128, E>;

    pub type Fix8<E> = Fix<P8, E>;
    pub type Fix16<E> = Fix<P16, E>;
    pub type Fix32<E> = Fix<P32, E>;
    pub type Fix64<E> = Fix<P64, E>;

    #[cfg(feature = "i128")]
    pub type Fix128<E> = Fix<P128, E>;
}

/// Base-10 types.
pub mod dec {
    use typenum::*;

    pub type UFix<B, E> = super::Fix<U10, B, E>;
    pub type Fix<B, E> = super::Fix<P10, B, E>;

    pub type UFix8<E> = UFix<P8, E>;
    pub type UFix16<E> = UFix<P16, E>;
    pub type UFix32<E> = UFix<P32, E>;
    pub type UFix64<E> = UFix<P64, E>;

    #[cfg(feature = "i128")]
    pub type UFix128<E> = UFix<P128, E>;

    pub type Fix8<E> = Fix<P8, E>;
    pub type Fix16<E> = Fix<P16, E>;
    pub type Fix32<E> = Fix<P32, E>;
    pub type Fix64<E> = Fix<P64, E>;

    #[cfg(feature = "i128")]
    pub type Fix128<E> = Fix<P128, E>;
}

/// SI prefixes.
pub mod si {
    use super::dec::{Fix, UFix};
    use typenum::{
        N1, N12, N15, N18, N2, N21, N24, N3, N6, N9, P1, P12, P15, P18, P2, P21, P24, P3, P6, P9,
        Z0,
    };

    /** Signed fixed ×10<sup>-24</sup> */
    pub type Yocto<B> = Fix<B, N24>;
    /** Signed fixed ×10<sup>-21</sup> */
    pub type Zepto<B> = Fix<B, N21>;
    /** Signed fixed ×10<sup>-18</sup> */
    pub type Atto<B> = Fix<B, N18>;
    /** Signed fixed ×10<sup>-15</sup> */
    pub type Femto<B> = Fix<B, N15>;
    /** Signed fixed ×10<sup>-12</sup> */
    pub type Pico<B> = Fix<B, N12>;
    /** Signed fixed ×10<sup>-9</sup> */
    pub type Nano<B> = Fix<B, N9>;
    /** Signed fixed ×10<sup>-6</sup> */
    pub type Micro<B> = Fix<B, N6>;
    /** Signed fixed ×10<sup>-3</sup> */
    pub type Milli<B> = Fix<B, N3>;
    /** Signed fixed ×10<sup>-2</sup> */
    pub type Centi<B> = Fix<B, N2>;
    /** Signed fixed ×10<sup>-1</sup> */
    pub type Deci<B> = Fix<B, N1>;

    /** Signed fixed ×10<sup>0</sup> */
    pub type Unit<B> = Fix<B, Z0>;

    /** Signed fixed ×10<sup>1</sup> */
    pub type Deca<B> = Fix<B, P1>;
    /** Signed fixed ×10<sup>2</sup> */
    pub type Hecto<B> = Fix<B, P2>;
    /** Signed fixed ×10<sup>3</sup> */
    pub type Kilo<B> = Fix<B, P3>;
    /** Signed fixed ×10<sup>6</sup> */
    pub type Mega<B> = Fix<B, P6>;
    /** Signed fixed ×10<sup>9</sup> */
    pub type Giga<B> = Fix<B, P9>;
    /** Signed fixed ×10<sup>12</sup> */
    pub type Tera<B> = Fix<B, P12>;
    /** Signed fixed ×10<sup>15</sup> */
    pub type Peta<B> = Fix<B, P15>;
    /** Signed fixed ×10<sup>18</sup> */
    pub type Exa<B> = Fix<B, P18>;
    /** Signed fixed ×10<sup>21</sup> */
    pub type Zeta<B> = Fix<B, P21>;
    /** Signed fixed ×10<sup>24</sup> */
    pub type Yotta<B> = Fix<B, P24>;

    /** Unsigned fixed ×10<sup>-24</sup> */
    pub type UYocto<B> = UFix<B, N24>;
    /** Unsigned fixed ×10<sup>-21</sup> */
    pub type UZepto<B> = UFix<B, N21>;
    /** Unsigned fixed ×10<sup>-18</sup> */
    pub type UAtto<B> = UFix<B, N18>;
    /** Unsigned fixed ×10<sup>-15</sup> */
    pub type UFemto<B> = UFix<B, N15>;
    /** Unsigned fixed ×10<sup>-12</sup> */
    pub type UPico<B> = UFix<B, N12>;
    /** Unsigned fixed ×10<sup>-9</sup> */
    pub type UNano<B> = UFix<B, N9>;
    /** Unsigned fixed ×10<sup>-6</sup> */
    pub type UMicro<B> = UFix<B, N6>;
    /** Unsigned fixed ×10<sup>-3</sup> */
    pub type UMilli<B> = UFix<B, N3>;
    /** Unsigned fixed ×10<sup>-2</sup> */
    pub type UCenti<B> = UFix<B, N2>;
    /** Unsigned fixed ×10<sup>-1</sup> */
    pub type UDeci<B> = UFix<B, N1>;

    /** Unsigned fixed ×10<sup>0</sup> */
    pub type UUnit<B> = UFix<B, Z0>;

    /** Unsigned fixed ×10<sup>1</sup> */
    pub type UDeca<B> = UFix<B, P1>;
    /** Unsigned fixed ×10<sup>2</sup> */
    pub type UHecto<B> = UFix<B, P2>;
    /** Unsigned fixed ×10<sup>3</sup> */
    pub type UKilo<B> = UFix<B, P3>;
    /** Unsigned fixed ×10<sup>6</sup> */
    pub type UMega<B> = UFix<B, P6>;
    /** Unsigned fixed ×10<sup>9</sup> */
    pub type UGiga<B> = UFix<B, P9>;
    /** Unsigned fixed ×10<sup>12</sup> */
    pub type UTera<B> = UFix<B, P12>;
    /** Unsigned fixed ×10<sup>15</sup> */
    pub type UPeta<B> = UFix<B, P15>;
    /** Unsigned fixed ×10<sup>18</sup> */
    pub type UExa<B> = UFix<B, P18>;
    /** Unsigned fixed ×10<sup>21</sup> */
    pub type UZeta<B> = UFix<B, P21>;
    /** Unsigned fixed ×10<sup>24</sup> */
    pub type UYotta<B> = UFix<B, P24>;
}

/// IEC prefixes.
pub mod iec {
    use super::bin::{Fix, UFix};
    use typenum::{P10, P20, P30, P40, P50, P60, P70, P80, Z0};

    /** Signed fixed ×2<sup>0</sup> */
    pub type Unit<B> = Fix<B, Z0>;

    /** Signed fixed ×2<sup>10</sup> */
    pub type Kibi<B> = Fix<B, P10>;
    /** Signed fixed ×2<sup>20</sup> */
    pub type Mebi<B> = Fix<B, P20>;
    /** Signed fixed ×2<sup>30</sup> */
    pub type Gibi<B> = Fix<B, P30>;
    /** Signed fixed ×2<sup>40</sup> */
    pub type Tebi<B> = Fix<B, P40>;
    /** Signed fixed ×2<sup>50</sup> */
    pub type Pebi<B> = Fix<B, P50>;
    /** Signed fixed ×2<sup>60</sup> */
    pub type Exbi<B> = Fix<B, P60>;
    /** Signed fixed ×2<sup>70</sup> */
    pub type Zebi<B> = Fix<B, P70>;
    /** Signed fixed ×2<sup>80</sup> */
    pub type Yobi<B> = Fix<B, P80>;

    /** Unsigned fixed ×2<sup>0</sup> */
    pub type UUnit<B> = UFix<B, Z0>;

    /** Unsigned fixed ×2<sup>10</sup> */
    pub type UKibi<B> = UFix<B, P10>;
    /** Unsigned fixed ×2<sup>20</sup> */
    pub type UMebi<B> = UFix<B, P20>;
    /** Unsigned fixed ×2<sup>30</sup> */
    pub type UGibi<B> = UFix<B, P30>;
    /** Unsigned fixed ×2<sup>40</sup> */
    pub type UTebi<B> = UFix<B, P40>;
    /** Unsigned fixed ×2<sup>50</sup> */
    pub type UPebi<B> = UFix<B, P50>;
    /** Unsigned fixed ×2<sup>60</sup> */
    pub type UExbi<B> = UFix<B, P60>;
    /** Unsigned fixed ×2<sup>70</sup> */
    pub type UZebi<B> = UFix<B, P70>;
    /** Unsigned fixed ×2<sup>80</sup> */
    pub type UYobi<B> = UFix<B, P80>;
}
