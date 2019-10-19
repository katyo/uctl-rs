/// Exponentiation.
///
/// Enables being generic over integers which can be exponentiated. Why must we do this, standard
/// library?
pub trait UnsignedPow {
    /// Raises `self` to the power of `exp`.
    fn pow(self, exp: u32) -> Self;
}

macro_rules! unsigned_pow {
    ($TYPE: ty) => {
        impl UnsignedPow for $TYPE {
            #[inline] fn pow(self, exp: u32) -> Self {
                self.pow(exp)
            }
        }
    };
}

unsigned_pow!(u8);
unsigned_pow!(u16);
unsigned_pow!(u32);
unsigned_pow!(u64);
#[cfg(feature = "i128")]
unsigned_pow!(u128);
unsigned_pow!(usize);

unsigned_pow!(i8);
unsigned_pow!(i16);
unsigned_pow!(i32);
unsigned_pow!(i64);
#[cfg(feature = "i128")]
unsigned_pow!(i128);
unsigned_pow!(isize);
