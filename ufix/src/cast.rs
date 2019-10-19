/// Similar to `core::convert::From`
pub trait Cast<T> {
    fn cast(value: T) -> Self;
}

macro_rules! cast_impl {
    ($TYPE: ty, $FROM: ty) => {
        impl Cast<$FROM> for $TYPE {
            fn cast(value: $FROM) -> Self {
                value as Self
            }
        }
    };

    ($TYPE: ty) => {
        cast_impl!($TYPE, u8);
        cast_impl!($TYPE, u16);
        cast_impl!($TYPE, u32);
        cast_impl!($TYPE, u64);
        #[cfg(feature = "i128")]
        cast_impl!($TYPE, u128);
        cast_impl!($TYPE, usize);

        cast_impl!($TYPE, i8);
        cast_impl!($TYPE, i16);
        cast_impl!($TYPE, i32);
        cast_impl!($TYPE, i64);
        #[cfg(feature = "i128")]
        cast_impl!($TYPE, i128);
        cast_impl!($TYPE, isize);

        cast_impl!($TYPE, f32);
        cast_impl!($TYPE, f64);
    };
}

cast_impl!(u8);
cast_impl!(u16);
cast_impl!(u32);
cast_impl!(u64);
#[cfg(feature = "i128")]
cast_impl!(u128);
cast_impl!(usize);

cast_impl!(i8);
cast_impl!(i16);
cast_impl!(i32);
cast_impl!(i64);
#[cfg(feature = "i128")]
cast_impl!(i128);
cast_impl!(isize);

cast_impl!(f32);
cast_impl!(f64);
