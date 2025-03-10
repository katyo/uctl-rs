use crate::{Error, Result};

/// Similar to `core::convert::From`
pub trait Cast<T> {
    /// Convert value from `T`
    fn cast(value: T) -> Self;
}

/// Similar to `core::convert::TryFrom`
pub trait TryCast<T>: Sized {
    /// Convert value from `T`
    fn try_cast(value: T) -> Result<Self>;
}

macro_rules! cast_impls {
    ($($type:ident),*) => {
        $(
            cast_impls!(@$type: u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
            #[cfg(feature = "i128")]
            cast_impls!(@$type: u128, i128);
        )*
    };

    (@$type:ident: $($from:ident),*) => {
        $(
            impl Cast<$from> for $type {
                fn cast(value: $from) -> Self {
                    value as Self
                }
            }
        )*
    };
}

cast_impls!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

#[cfg(feature = "i128")]
cast_impls!(u128, i128);

macro_rules! try_cast_impls {
    ($($(#[$($meta:meta)*])* $from:ident => $type:ident $(($($check:ident),*))*;)*) => {
        $(
            $(#[$($meta)*])*
            impl TryCast<$from> for $type {
                fn try_cast(value: $from) -> Result<Self> {
                    $($(try_cast_impls!{@$check value: $from => $type})*)*
                    Ok(value as Self)
                }
            }
        )*
    };

    (@max $val:ident: $from:ident => $type:ident) => {
        if $val > $type::MAX as $from {
            return Err(Error::TooBig);
        }
    };

    (@min $val:ident: $from:ident => $type:ident) => {
        if $val < $type::MIN as $from {
            return Err(Error::TooSmall);
        }
    };
}

try_cast_impls! {
    // u8 => u
    u8 => u8;
    u8 => u16;
    u8 => u32;
    u8 => u64;

    // u8 => i
    u8 => i8 (max);
    u8 => i16;
    u8 => i32;
    u8 => i64;

    // u8 => f
    u8 => f32;
    u8 => f64;

    // i8 => u
    i8 => u8 (min);
    i8 => u16 (min);
    i8 => u32 (min);
    i8 => u64 (min);

    // i8 => i
    i8 => i8;
    i8 => i16;
    i8 => i32;
    i8 => i64;

    // u8 => f
    i8 => f32;
    i8 => f64;

    // u16 => u
    u16 => u8 (max);
    u16 => u16;
    u16 => u32;
    u16 => u64;

    // u16 => i
    u16 => i8 (max);
    u16 => i16 (max);
    u16 => i32;
    u16 => i64;

    // u16 => f
    u16 => f32;
    u16 => f64;

    // i16 => u
    i16 => u8 (min, max);
    i16 => u16 (min);
    i16 => u32 (min);
    i16 => u64 (min);

    // i16 => i
    i16 => i8 (min, max);
    i16 => i16;
    i16 => i32;
    i16 => i64;

    // i16 => f
    i16 => f32;
    i16 => f64;

    // u32 => u
    u32 => u8 (max);
    u32 => u16 (max);
    u32 => u32;
    u32 => u64;

    // u32 => i
    u32 => i8 (max);
    u32 => i16 (max);
    u32 => i32 (max);
    u32 => i64;

    // u32 => f
    u32 => f32;
    u32 => f64;

    // i32 => u
    i32 => u8 (min, max);
    i32 => u16 (min, max);
    i32 => u32 (min);
    i32 => u64 (min);

    // i32 => i
    i32 => i8 (min, max);
    i32 => i16 (min, max);
    i32 => i32;
    i32 => i64;

    // i32 => f
    i32 => f32;
    i32 => f64;

    // u64 => u
    u64 => u8 (max);
    u64 => u16 (max);
    u64 => u32 (max);
    u64 => u64;

    // u64 => i
    u64 => i8 (max);
    u64 => i16 (max);
    u64 => i32 (max);
    u64 => i64 (max);

    // u64 => f
    u64 => f32;
    u64 => f64;

    // i64 => u
    i64 => u8 (min, max);
    i64 => u16 (min, max);
    i64 => u32 (min, max);
    i64 => u64 (min);

    // i64 => i
    i64 => i8 (min, max);
    i64 => i16 (min, max);
    i64 => i32 (min, max);
    i64 => i64;

    // i64 => f
    i64 => f32;
    i64 => f64;

    /// f32 => u
    f32 => u8 (min, max);
    f32 => u16 (min, max);
    f32 => u32 (min, max);
    f32 => u64 (min, max);

    /// f32 => i
    f32 => i8 (min, max);
    f32 => i16 (min, max);
    f32 => i32 (min, max);
    f32 => i64 (min, max);

    /// f32 => f
    f32 => f32;
    f32 => f64;

    /// f32 => u
    f64 => u8 (min, max);
    f64 => u16 (min, max);
    f64 => u32 (min, max);
    f64 => u64 (min, max);

    /// f32 => i
    f64 => i8 (min, max);
    f64 => i16 (min, max);
    f64 => i32 (min, max);
    f64 => i64 (min, max);

    /// f32 => f
    f64 => f32 (min, max);
    f64 => f64;
}

#[cfg(feature = "i128")]
try_cast_impls! {
    u8 => u128;
    u8 => i128;

    i8 => u128 (min);
    i8 => i128;

    u16 => u128;
    u16 => i128;

    i16 => u128 (min);
    i16 => i128;

    u32 => u128;
    u32 => i128;

    i32 => u128 (min);
    i32 => i128;

    u64 => u128;
    u64 => i128;

    i64 => u128 (min);
    i64 => i128;

    f32 => u128 (min, max);
    f32 => i128 (min, max);

    f64 => u128 (min, max);
    f64 => i128 (min, max);

    // u128 => u
    u128 => u8 (max);
    u128 => u16 (max);
    u128 => u32 (max);
    u128 => u64 (max);
    u128 => u128;

    // u128 => i
    u128 => i8 (max);
    u128 => i16 (max);
    u128 => i32 (max);
    u128 => i64 (max);
    u128 => i128 (max);

    // u128 => f
    u128 => f32;
    u128 => f64;

    // i128 => u
    i128 => u8 (min, max);
    i128 => u16 (min, max);
    i128 => u32 (min, max);
    i128 => u64 (min, max);
    i128 => u128 (min);

    // i128 => i
    i128 => i8 (min, max);
    i128 => i16 (min, max);
    i128 => i32 (min, max);
    i128 => i64 (min, max);
    i128 => i128;

    // i128 => f
    i128 => f32;
    i128 => f64;
}
