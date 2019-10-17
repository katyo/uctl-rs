mod fixed;

/// Similar to `core::convert::From`
pub trait FromOther<T> {
    fn from_other(value: T) -> Self;
}

macro_rules! from_other {
    ($TYPE: ty, $FROM: ty) => {
        impl FromOther<$FROM> for $TYPE {
            fn from_other(value: $FROM) -> Self {
                value as Self
            }
        }
    };

    ($TYPE: ty) => {
        from_other!($TYPE, u8);
        from_other!($TYPE, u16);
        from_other!($TYPE, u32);
        from_other!($TYPE, u64);
        #[cfg(feature = "i128")]
        from_other!($TYPE, u128);
        from_other!($TYPE, usize);

        from_other!($TYPE, i8);
        from_other!($TYPE, i16);
        from_other!($TYPE, i32);
        from_other!($TYPE, i64);
        #[cfg(feature = "i128")]
        from_other!($TYPE, i128);
        from_other!($TYPE, isize);

        from_other!($TYPE, f32);
        from_other!($TYPE, f64);
    };
}

from_other!(u8);
from_other!(u16);
from_other!(u32);
from_other!(u64);
#[cfg(feature = "i128")]
from_other!(u128);
from_other!(usize);

from_other!(i8);
from_other!(i16);
from_other!(i32);
from_other!(i64);
#[cfg(feature = "i128")]
from_other!(i128);
from_other!(isize);

from_other!(f32);
from_other!(f64);
