use typenum::{Bit, NonZero, PInt, UInt, Unsigned};

pub trait Positive {
    const U32: u32;
    const USIZE: usize;
}

impl<U: Unsigned, B: Bit> Positive for UInt<U, B> {
    const U32: u32 = B::U8 as u32 | U::U32 << 1;
    const USIZE: usize = B::U8 as usize | U::USIZE << 1;
}

impl<U: Unsigned + NonZero> Positive for PInt<U> {
    const U32: u32 = U::U32;
    const USIZE: usize = U::USIZE;
}
