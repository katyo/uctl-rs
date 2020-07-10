// Allow due to unexpected behavior on it
#![allow(clippy::type_repetition_in_bounds)]

use super::{Cast, Fix, Mantissa, Positive, Radix};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use typenum::{Diff, Integer, Max, Maximum, Min, Minimum, Sum, P1};

pub trait Add1: Add<P1> {}

impl<T: Add<P1>> Add1 for T {}

type Sum1<T> = <T as Add<P1>>::Output;

// Arithmetic.

impl<R, B, E> Neg for Fix<R, B, E>
where
    R: Radix<B>,
    B: Positive,
    E: Integer,
    Mantissa<R, B>: Neg<Output = Mantissa<R, B>>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.bits)
    }
}

type SumE<E1, E2> = Minimum<E1, E2>;
type SumBovf<B1, E1, B2, E2> = Diff<Maximum<Sum<B1, E1>, Sum<B2, E2>>, SumE<E1, E2>>;
type SumB<B1, E1, B2, E2> = Sum1<SumBovf<B1, E1, B2, E2>>;
type SumT<R, B1, E1, B2, E2> = Fix<R, SumB<B1, E1, B2, E2>, SumE<E1, E2>>;

/// Fixed-point addition
///
/// Fix<R, B1, E1> + Fix<R, B2, E2> = Fix<R, max(B1 + E1, B2 + E2) - min(E1, E2), min(E1, E2)>
///
/// BFix<32, -16> (Q16.16) + BFix<32, -24> (Q8.24) = BFix<40, -24> (Q40.24)
/// BFix<32, 16> + BFix<32, 8> = BFix<40, 8>
///
impl<R, B1, E1, B2, E2> Add<Fix<R, B2, E2>> for Fix<R, B1, E1>
where
    R: Radix<B1> + Radix<B2> + Radix<SumB<B1, E1, B2, E2>>,
    B1: Positive + Add<E1>,
    E1: Integer + Min<E2>,
    B2: Positive + Add<E2>,
    E2: Integer,
    Sum<B1, E1>: Max<Sum<B2, E2>>,
    SumBovf<B1, E1, B2, E2>: Add1,
    SumB<B1, E1, B2, E2>: Positive,
    SumE<E1, E2>: Integer,
    Maximum<Sum<B1, E1>, Sum<B2, E2>>: Sub<Minimum<E1, E2>>,
    Mantissa<R, SumB<B1, E1, B2, E2>>: Cast<Mantissa<R, B1>>
        + Cast<Mantissa<R, B2>>
        + Add<Output = Mantissa<R, SumB<B1, E1, B2, E2>>>,
{
    type Output = SumT<R, B1, E1, B2, E2>;

    fn add(self, other: Fix<R, B2, E2>) -> Self::Output {
        let a: SumT<R, B1, E1, B2, E2> = self.convert();
        let b: SumT<R, B1, E1, B2, E2> = other.convert();

        Self::Output::new(a.bits + b.bits)
    }
}

/// Fixed-point substraction
///
/// Fix<R, B1, E1> - Fix<R, B2, E2> = Fix<R, max(B1 + E1, B2 + E2) - min(E1, E2), min(E1, E2)>
///
impl<R, B1, E1, B2, E2> Sub<Fix<R, B2, E2>> for Fix<R, B1, E1>
where
    R: Radix<B1> + Radix<B2> + Radix<SumB<B1, E1, B2, E2>>,
    B1: Positive + Add<E1>,
    E1: Integer + Min<E2>,
    B2: Positive + Add<E2>,
    E2: Integer,
    Sum<B1, E1>: Max<Sum<B2, E2>>,
    SumBovf<B1, E1, B2, E2>: Add1,
    SumB<B1, E1, B2, E2>: Positive,
    SumE<E1, E2>: Integer,
    Maximum<Sum<B1, E1>, Sum<B2, E2>>: Sub<Minimum<E1, E2>>,
    Mantissa<R, SumB<B1, E1, B2, E2>>: Cast<Mantissa<R, B1>>
        + Cast<Mantissa<R, B2>>
        + Sub<Output = Mantissa<R, SumB<B1, E1, B2, E2>>>,
{
    type Output = SumT<R, B1, E1, B2, E2>;

    fn sub(self, other: Fix<R, B2, E2>) -> Self::Output {
        let a: SumT<R, B1, E1, B2, E2> = self.convert();
        let b: SumT<R, B1, E1, B2, E2> = other.convert();

        Self::Output::new(a.bits - b.bits)
    }
}

type ProdB<B1, B2> = Sum<B1, B2>;
type ProdE<E1, E2> = Sum<E1, E2>;
type ProdM<R, B1, B2> = Mantissa<R, ProdB<B1, B2>>;
type ProdT<R, B1, E1, B2, E2> = Fix<R, ProdB<B1, B2>, ProdE<E1, E2>>;

/// Fixed-point multiplication
///
/// Fix<R, B1, E1> * Fix<R, B2, E2> = Fix<R, B1 + B2, E1 + E2>
///
impl<R, B1, E1, B2, E2> Mul<Fix<R, B2, E2>> for Fix<R, B1, E1>
where
    R: Radix<B1> + Radix<B2> + Radix<ProdB<B1, B2>>,
    B1: Positive + Add<B2>,
    E1: Integer + Add<E2>,
    B2: Positive,
    E2: Integer,
    ProdB<B1, B2>: Positive,
    ProdE<E1, E2>: Integer,
    ProdM<R, B1, B2>:
        Cast<Mantissa<R, B1>> + Cast<Mantissa<R, B2>> + Mul<Output = ProdM<R, B1, B2>>,
{
    type Output = ProdT<R, B1, E1, B2, E2>;
    fn mul(self, other: Fix<R, B2, E2>) -> Self::Output {
        let a = ProdM::<R, B1, B2>::cast(self.bits);
        let b = ProdM::<R, B1, B2>::cast(other.bits);

        Self::Output::new(a * b)
    }
}

type QuotB<B1, B2> = Diff<B1, B2>;
type QuotE<E1, E2> = Diff<E1, E2>;
type QuotM<R, B1, B2> = Mantissa<R, QuotB<B1, B2>>;
type QuotT<R, B1, E1, B2, E2> = Fix<R, QuotB<B1, B2>, QuotE<E1, E2>>;

/// Fixed-point division
///
/// Fix<R, B1, E1> / Fix<R, B2, E2> = Fix<R, B1 - B2, Base, E1 - E2>
///
impl<R, B1, E1, B2, E2> Div<Fix<R, B2, E2>> for Fix<R, B1, E1>
where
    R: Radix<B1> + Radix<B2> + Radix<QuotB<B1, B2>>,
    B1: Positive + Sub<B2>,
    E1: Integer + Sub<E2>,
    B2: Positive,
    E2: Integer,
    QuotB<B1, B2>: Positive,
    QuotE<E1, E2>: Integer,
    QuotM<R, B1, B2>: Cast<Mantissa<R, B1>>,
    Mantissa<R, B1>: Cast<Mantissa<R, B2>> + Div<Output = Mantissa<R, B1>>,
{
    type Output = QuotT<R, B1, E1, B2, E2>;
    fn div(self, other: Fix<R, B2, E2>) -> Self::Output {
        let a = self.bits;
        let b = Mantissa::<R, B1>::cast(other.bits);
        let c = QuotM::<R, B1, B2>::cast(a / b);

        Self::Output::new(c)
    }
}

/// Fixed-point reminder
///
impl<R, B, E> Rem for Fix<R, B, E>
where
    R: Radix<B>,
    B: Positive,
    E: Integer,
    Mantissa<R, B>: Rem<Output = Mantissa<R, B>>,
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.bits % rhs.bits)
    }
}

// Assignment.

impl<R, B, E, T> AddAssign<T> for Fix<R, B, E>
where
    R: Radix<B>,
    Mantissa<R, B>: AddAssign,
    Fix<R, B, E>: Cast<T>,
{
    fn add_assign(&mut self, other: T) {
        let b = Fix::<R, B, E>::cast(other);
        self.bits += b.bits;
    }
}

impl<R, B, E, T> SubAssign<T> for Fix<R, B, E>
where
    R: Radix<B>,
    Mantissa<R, B>: SubAssign,
    Fix<R, B, E>: Cast<T>,
{
    fn sub_assign(&mut self, other: T) {
        let b = Fix::<R, B, E>::cast(other);
        self.bits -= b.bits;
    }
}

impl<R, B, E, T> MulAssign<T> for Fix<R, B, E>
where
    R: Radix<B>,
    Mantissa<R, B>: MulAssign<T>,
{
    fn mul_assign(&mut self, other: T) {
        self.bits *= other;
    }
}

impl<R, B, E, T> DivAssign<T> for Fix<R, B, E>
where
    R: Radix<B>,
    Mantissa<R, B>: DivAssign<T>,
{
    fn div_assign(&mut self, other: T) {
        self.bits /= other;
    }
}

impl<R, B, E1, E2> RemAssign<Fix<R, B, E2>> for Fix<R, B, E1>
where
    R: Radix<B>,
    Mantissa<R, B>: RemAssign,
{
    fn rem_assign(&mut self, other: Fix<R, B, E2>) {
        self.bits %= other.bits;
    }
}

#[cfg(test)]
mod tests {
    use super::super::si::{Centi, Kilo, Milli, Unit};
    use typenum::*;

    #[test]
    fn convert_milli_to_kilo() {
        assert_eq!(Kilo::<P2>::new(15), Milli::<P8>::new(15_000_000).convert());
    }

    #[test]
    fn convert_kilo_to_milli() {
        assert_eq!(Milli::<U8>::new(15_000_000), Kilo::<U2>::new(15).convert());
    }

    #[test]
    fn cmp_unsigned() {
        assert!(Kilo::<U1>::new(1) < Kilo::new(2));
    }

    #[test]
    fn cmp_signed() {
        assert!(Kilo::<P1>::new(1) < Kilo::new(2));
        assert!(Kilo::<P1>::new(-2) < Kilo::new(-1));
    }

    #[test]
    fn neg_signed() {
        assert_eq!(-Kilo::<P1>::new(1), Kilo::new(-1));
        assert_eq!(-Kilo::<P1>::new(-1), Kilo::new(1));
    }

    #[test]
    fn add_signed() {
        assert_eq!(Kilo::<P2>::new(3), Kilo::<P1>::new(1) + Kilo::<P1>::new(2));
        assert_eq!(
            Centi::<P4>::new(0_30),
            Centi::<P3>::new(0_10) + Centi::<P3>::new(0_20)
        );
    }

    /*#[test]
    fn add_unsigned() {
        assert_eq!(Kilo::<U1>::new(1) + Kilo::<U1>::new(2), Kilo::<U1>::new(3));
        assert_eq!(
            Centi::<P3>::new(0_10) + Centi::<U3>::new(0_20),
            Centi::<P3>::new(0_30)
        );
    }*/

    #[test]
    fn sub_signed() {
        assert_eq!(Kilo::<P2>::new(1), Kilo::<P1>::new(3) - Kilo::<P1>::new(2));
    }

    #[test]
    fn mul_signed() {
        assert_eq!(Unit::new(6), Kilo::<P1>::new(2) * Milli::<P1>::new(3));
    }

    #[test]
    fn div_signed() {
        assert_eq!(Unit::new(3), Kilo::<P2>::new(6) / Kilo::<P1>::new(2));
    }

    #[test]
    fn rem_signed() {
        assert_eq!(Kilo::<P1>::new(1), Kilo::new(6) % Kilo::new(5));
    }

    #[test]
    fn add_assign_signed() {
        let mut a = Kilo::<P5>::new(1);
        a += Kilo::<P5>::new(2);
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn sub_assign_signed() {
        let mut a = Kilo::<U5>::new(3);
        a -= Kilo::<U5>::new(2);
        assert_eq!(Kilo::new(1), a);
    }

    #[test]
    fn mul_assign_signed_bits() {
        let mut a = Kilo::<P5>::new(2);
        a *= 3;
        assert_eq!(Kilo::new(6), a);
    }

    #[test]
    fn div_assign_signed_bits() {
        let mut a = Kilo::<P9>::new(6);
        a /= 2;
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn rem_assign_signed() {
        let mut a = Kilo::<P9>::new(6);
        a %= Milli::new(5);
        assert_eq!(Kilo::new(1), a);
    }
}
