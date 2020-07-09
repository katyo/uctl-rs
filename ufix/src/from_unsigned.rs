use typenum::Unsigned;

/// Conversion from type-level [`Unsigned`] integers.
///
/// Enables being generic over types which can be created from type-level integers. It should
/// probably be in `typenum` itself...
///
/// [`Unsigned`]: ../typenum/marker_traits/trait.Unsigned.html
pub trait FromUnsigned {
    /// Creates a value from a type.
    fn from_unsigned<U>() -> Self
    where
        U: Unsigned;
}

macro_rules! from_unsigned {
    ($TYPE: ty, $METHOD: ident) => {
        impl FromUnsigned for $TYPE {
            fn from_unsigned<U: Unsigned>() -> Self {
                U::$METHOD()
            }
        }
    };
}

from_unsigned!(u8, to_u8);
from_unsigned!(u16, to_u16);
from_unsigned!(u32, to_u32);
from_unsigned!(u64, to_u64);
#[cfg(feature = "i128")]
from_unsigned!(u128, to_u128);
from_unsigned!(usize, to_usize);

from_unsigned!(i8, to_i8);
from_unsigned!(i16, to_i16);
from_unsigned!(i32, to_i32);
from_unsigned!(i64, to_i64);
#[cfg(feature = "i128")]
from_unsigned!(i128, to_i128);
from_unsigned!(isize, to_isize);
