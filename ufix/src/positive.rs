use typenum::{Bit, NonZero, PInt, UInt, Unsigned};

/// The trait which implemented for type-level numbers which is greater than zero.
pub trait Positive {
    /// The type as u8
    const U8: u8;
    /// The type as u16
    const U16: u16;
    /// The type as u32
    const U32: u32;
    /// The type as u64
    const U64: u64;
    #[cfg(feature = "i128")]
    /// The type as u128
    const U128: u128;
    /// The type as usize
    const USIZE: usize;

    /// The type as i8
    const I8: i8;
    /// The type as i16
    const I16: i16;
    /// The type as i32
    const I32: i32;
    /// The type as i64
    const I64: i64;
    #[cfg(feature = "i128")]
    /// The type as i128
    const I128: i128;
    /// The type as isize
    const ISIZE: isize;
}

impl<U: Unsigned + NonZero> Positive for PInt<U> {
    const U8: u8 = U::U8;
    const U16: u16 = U::U16;
    const U32: u32 = U::U32;
    const U64: u64 = U::U64;
    #[cfg(feature = "i128")]
    const U128: u128 = U::U128;
    const USIZE: usize = U::USIZE;

    const I8: i8 = U::U8 as i8;
    const I16: i16 = U::U16 as i16;
    const I32: i32 = U::U32 as i32;
    const I64: i64 = U::U64 as i64;
    #[cfg(feature = "i128")]
    const I128: i128 = U::U128 as i128;
    const ISIZE: isize = U::USIZE as isize;
}

impl<U: Unsigned, B: Bit> Positive for UInt<U, B> {
    const U8: u8 = PInt::<Self>::U8;
    const U16: u16 = PInt::<Self>::U16;
    const U32: u32 = PInt::<Self>::U32;
    const U64: u64 = PInt::<Self>::U64;
    #[cfg(feature = "i128")]
    const U128: u128 = PInt::<Self>::U128;
    const USIZE: usize = PInt::<Self>::USIZE;

    const I8: i8 = PInt::<Self>::I8;
    const I16: i16 = PInt::<Self>::I16;
    const I32: i32 = PInt::<Self>::I32;
    const I64: i64 = PInt::<Self>::I64;
    #[cfg(feature = "i128")]
    const I128: i128 = PInt::<Self>::I128;
    const ISIZE: isize = PInt::<Self>::ISIZE;
}

/// Conversion from type-level [`Positive`] integers.
///
/// Enables being generic over types which can be created from type-level integers.
/// It should probably be in `typenum` itself.
pub trait FromPositive {
    /// Creates a value from a type.
    fn from_positive<N>() -> Self
    where
        N: Positive;
}

macro_rules! from_positive_impl {
    ($($type: ty: $const: ident,)*) => {
        $(
            impl FromPositive for $type {
                fn from_positive<N>() -> Self
                where
                    N: Positive,
                {
                    N::$const
                }
            }
        )*
    };
}

from_positive_impl! {
    u8: U8,
    u16: U16,
    u32: U32,
    u64: U64,

    i8: I8,
    i16: I16,
    i32: I32,
    i64: I64,
}

#[cfg(feature = "i128")]
from_positive_impl! {
    u128: U128,
    i128: I128,
}
