use crate::{Digits, Exponent, Fix, Mantissa, Radix};
use core::hash::{Hash, Hasher};

// Hash, PartialEq, Eq, PartialOrd, Ord

impl<R, B, E> Hash for Fix<R, B, E>
where
    R: Radix<B>,
    B: Digits,
    E: Exponent,
    Mantissa<R, B>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        R::U32.hash(state);
        B::I32.hash(state);
        E::I32.hash(state);
        self.bits.hash(state);
    }
}

#[cfg(feature = "hash32")]
mod __hash32 {
    use super::{Digits, Exponent, Fix, Mantissa, Radix};
    use hash32::{Hash, Hasher};

    impl<R, B, E> Hash for Fix<R, B, E>
    where
        R: Radix<B>,
        B: Digits,
        E: Exponent,
        Mantissa<R, B>: Hash,
    {
        fn hash<H: Hasher>(&self, state: &mut H) {
            R::U32.hash(state);
            B::I32.hash(state);
            E::I32.hash(state);
            self.bits.hash(state);
        }
    }
}
