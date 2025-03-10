use crate::{Error, Result};

/// Safe multiply trait
pub trait TryMul<Rhs = Self> {
    /// Output value
    type Output;

    /// Required method
    fn try_mul(self, rhs: Rhs) -> Result<Self::Output>;
}

macro_rules! try_mul_impls {
    ($($type:ident),*) => {
        $(
            impl TryMul for $type {
                type Output = Self;
                fn try_mul(self, rhs: Self) -> Result<Self> {
                    self.checked_mul(rhs).ok_or(Error::TooBig)
                }
            }
        )*
    };
}

try_mul_impls!(u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(feature = "i128")]
try_mul_impls!(u128, i128);
