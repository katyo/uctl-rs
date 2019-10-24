// Allow due to unexpected behavior on it
#![allow(clippy::type_repetition_in_bounds)]

use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign, Add, Div, Mul, Neg, Rem, Sub};
use typenum::{Diff, Sum, IsLess, Min, Max, Minimum, Maximum, Unsigned, Integer, Abs, AbsVal, Z0};
use super::{Fix, BitsType, FromUnsigned, Pow, Cast};

// Arithmetic.

impl<Bits, Base, Exp> Neg for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: Neg<Output = Bits::Type>
{
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.bits)
    }
}

/// Fixed-point addition
///
/// Fix<LBits, Base, LExp> + Fix<RBits, Base, RExp> = Fix<Maximum<LBits, RBits>, Base, Minimum<LExp, RExp>>
///
impl<LBits, RBits, Base, LExp, RExp> Add<Fix<RBits, Base, RExp>> for Fix<LBits, Base, LExp>
where
    LBits: BitsType<Base> + IsLess<Maximum<LBits, RBits>> + Max<RBits>,
    LBits::Type: FromUnsigned + Pow + Mul<Output = LBits::Type> + Div<Output = LBits::Type>,

    RBits: BitsType<Base> + IsLess<Maximum<LBits, RBits>>,
    RBits::Type: FromUnsigned + Pow + Mul<Output = RBits::Type> + Div<Output = RBits::Type>,

    Maximum<LBits, RBits>: BitsType<Base>,
    <Maximum<LBits, RBits> as BitsType<Base>>::Type: FromUnsigned + Pow +
        Cast<LBits::Type> + Cast<RBits::Type> +
        Mul<Output = <Maximum<LBits, RBits> as BitsType<Base>>::Type> +
        Div<Output = <Maximum<LBits, RBits> as BitsType<Base>>::Type> +
        Add<Output = <Maximum<LBits, RBits> as BitsType<Base>>::Type>,

    Base: Unsigned,

    LExp: Sub<Minimum<LExp, RExp>> + Min<RExp>,
    Diff<LExp, Minimum<LExp, RExp>>: Abs + IsLess<Z0>,
    AbsVal<Diff<LExp, Minimum<LExp, RExp>>>: Integer,

    RExp: Sub<Minimum<LExp, RExp>> + Min<LExp>,
    Diff<RExp, Minimum<LExp, RExp>>: Abs + IsLess<Z0>,
    AbsVal<Diff<RExp, Minimum<LExp, RExp>>>: Integer,
{
    type Output = Fix<Maximum<LBits, RBits>, Base, Minimum<LExp, RExp>>;

    fn add(self, rhs: Fix<RBits, Base, RExp>) -> Self::Output {
        Self::Output::new(self.convert::<Maximum<LBits, RBits>, Minimum<LExp, RExp>>().bits +
                          rhs.convert::<Maximum<LBits, RBits>, Minimum<LExp, RExp>>().bits)
    }
}

/// Fixed-point substraction
///
/// Fix<LBits, Base, LExp> - Fix<RBits, Base, RExp> = Fix<Maximum<LBits, RBits>, Base, Minimum<LExp, RExp>>
///
impl<LBits, RBits, Base, LExp, RExp> Sub<Fix<RBits, Base, RExp>> for Fix<LBits, Base, LExp>
where
    LBits: BitsType<Base> + IsLess<Maximum<LBits, RBits>> + Max<RBits>,
    LBits::Type: FromUnsigned + Pow + Mul<Output = LBits::Type> + Div<Output = LBits::Type>,

    RBits: BitsType<Base> + IsLess<Maximum<LBits, RBits>>,
    RBits::Type: FromUnsigned + Pow + Mul<Output = RBits::Type> + Div<Output = RBits::Type>,

    Maximum<LBits, RBits>: BitsType<Base>,
    <Maximum<LBits, RBits> as BitsType<Base>>::Type: FromUnsigned + Pow +
        Cast<LBits::Type> + Cast<RBits::Type> +
        Mul<Output = <Maximum<LBits, RBits> as BitsType<Base>>::Type> +
        Div<Output = <Maximum<LBits, RBits> as BitsType<Base>>::Type> +
        Sub<Output = <Maximum<LBits, RBits> as BitsType<Base>>::Type>,

    Base: Unsigned,

    LExp: Sub<Minimum<LExp, RExp>> + Min<RExp>,
    Diff<LExp, Minimum<LExp, RExp>>: Abs + IsLess<Z0>,
    AbsVal<Diff<LExp, Minimum<LExp, RExp>>>: Integer,

    RExp: Sub<Minimum<LExp, RExp>> + Min<LExp>,
    Diff<RExp, Minimum<LExp, RExp>>: Abs + IsLess<Z0>,
    AbsVal<Diff<RExp, Minimum<LExp, RExp>>>: Integer,
{
    type Output = Fix<Maximum<LBits, RBits>, Base, Minimum<LExp, RExp>>;

    fn sub(self, rhs: Fix<RBits, Base, RExp>) -> Self::Output {
        Self::Output::new(self.convert::<Maximum<LBits, RBits>, Minimum<LExp, RExp>>().bits -
                          rhs.convert::<Maximum<LBits, RBits>, Minimum<LExp, RExp>>().bits)
    }
}

/// Fixed-point multiplication
///
/// Fix<LBits, Base, LExp> * Fix<RBits, Base, RExp> = Fix<LBits + RBits, Base, LExp + RExp>
///
impl<LBits, RBits, Base, LExp, RExp> Mul<Fix<RBits, Base, RExp>> for Fix<LBits, Base, LExp>
where
    LBits: BitsType<Base> + Add<RBits>,
    RBits: BitsType<Base>,
    Sum<LBits, RBits>: BitsType<Base>,
    <Sum<LBits, RBits> as BitsType<Base>>::Type: Cast<LBits::Type> + Cast<RBits::Type> +
      Mul<Output = <Sum<LBits, RBits> as BitsType<Base>>::Type>,
    LExp: Add<RExp>,
{
    type Output = Fix<Sum<LBits, RBits>, Base, Sum<LExp, RExp>>;
    fn mul(self, rhs: Fix<RBits, Base, RExp>) -> Self::Output {
        Self::Output::new(<Sum<LBits, RBits> as BitsType<Base>>::Type::cast(self.bits) *
                          <Sum<LBits, RBits> as BitsType<Base>>::Type::cast(rhs.bits))
    }
}

/// Fixed-point division
///
/// Fix<LBits, Base, LExp> / Fix<RBits, Base, RExp> = Fix<LBits - RBits, Base, LExp - RExp>
///
impl<LBits, RBits, Base, LExp, RExp> Div<Fix<RBits, Base, RExp>> for Fix<LBits, Base, LExp>
where
    LBits: BitsType<Base> + Sub<RBits>,
    RBits: BitsType<Base>,
    Diff<LBits, RBits>: BitsType<Base>,
    LBits::Type: Cast<RBits::Type> + Div<Output = LBits::Type>,
    LExp: Sub<RExp>,
    <Diff<LBits, RBits> as BitsType<Base>>::Type: Cast<LBits::Type>,
{
    type Output = Fix<Diff<LBits, RBits>, Base, Diff<LExp, RExp>>;
    fn div(self, rhs: Fix<RBits, Base, RExp>) -> Self::Output {
        Self::Output::new(
            <Diff<LBits, RBits> as BitsType<Base>>::Type::cast(
                self.bits / LBits::Type::cast(rhs.bits)))
    }
}

/// Fixed-point reminder
///
impl<Bits, Base, Exp> Rem for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: Rem<Output = Bits::Type>
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.bits % rhs.bits)
    }
}

/*
impl<Type, Bits, Base, Exp> Mul<Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: Mul<Bits::Type, Output = Bits::Type>
{
    type Output = Self;
    fn mul(self, rhs: Bits::Type) -> Self {
        Self::new(self.bits * rhs)
    }
}

impl<Bits, Base, Exp> Div<<Bits as BitsType>::Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: Div<Output = Bits::Type>
{
    type Output = Self;
    fn div(self, rhs: Bits) -> Self {
        Self::new(self.bits / rhs)
    }
}

impl<Bits, Base, Exp> Rem<Bits> for Fix<Bits, Base, Exp>
where Bits: Rem<Output = Bits> {
    type Output = Self;
    fn rem(self, rhs: Bits) -> Self {
        Self::new(self.bits % rhs)
    }
}
*/

// Assignment.

impl<Bits, Base, Exp> AddAssign for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.bits += rhs.bits;
    }
}

impl<Bits, Base, Exp> SubAssign for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.bits -= rhs.bits;
    }
}

impl<Type, Bits, Base, Exp> MulAssign<Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: MulAssign<Type>,
{
    fn mul_assign(&mut self, rhs: Type) {
        self.bits *= rhs;
    }
}

impl<Type, Bits, Base, Exp> DivAssign<Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType<Base>,
    Bits::Type: DivAssign<Type>,
{
    fn div_assign(&mut self, rhs: Type) {
        self.bits /= rhs;
    }
}

impl<Bits, Base, LExp, RExp> RemAssign<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where
    Bits: BitsType<Base>,
    Bits::Type: RemAssign<Bits::Type>,
{
    fn rem_assign(&mut self, rhs: Fix<Bits, Base, RExp>) {
        self.bits %= rhs.bits;
    }
}

/*
impl<Type, Bits, Base, Exp> RemAssign<Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: RemAssign<Type>,
{
    fn rem_assign(&mut self, rhs: Type) {
        self.bits %= rhs;
    }
}
*/

#[cfg(test)]
mod tests {
    use super::super::si::{Kilo, Milli, Unit};
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
    fn cmp() {
        assert!(Kilo::<U1>::new(1) < Kilo::new(2));
    }

    #[test]
    fn neg() {
        assert_eq!(Kilo::<P1>::new(-1), -Kilo::new(1));
    }

    #[test]
    fn add() {
        assert_eq!(Kilo::<P1>::new(3), Kilo::<P1>::new(1) + Kilo::<P1>::new(2));
    }

    #[test]
    fn sub() {
        assert_eq!(Kilo::<P1>::new(1), Kilo::<P1>::new(3) - Kilo::<P1>::new(2));
    }

    #[test]
    fn mul() {
        assert_eq!(Unit::new(6), Kilo::<P1>::new(2) * Milli::<P1>::new(3));
    }

    #[test]
    fn div() {
        assert_eq!(Unit::new(3), Kilo::<P2>::new(6) / Kilo::<P1>::new(2));
    }

    #[test]
    fn rem() {
        assert_eq!(Kilo::<P1>::new(1), Kilo::new(6) % Kilo::new(5));
    }

    /*
    #[test]
    fn mul_bits() {
        assert_eq!(Kilo::new(6), Kilo::new(2) * 3);
    }

    #[test]
    fn div_bits() {
        assert_eq!(Kilo::new(3), Kilo::new(6) / 2);
    }

    #[test]
    fn rem_bits() {
        assert_eq!(Kilo::new(1), Kilo::new(6) % 5);
    }
     */

    #[test]
    fn add_assign() {
        let mut a = Kilo::<P5>::new(1);
        a += Kilo::new(2);
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn sub_assign() {
        let mut a = Kilo::<U5>::new(3);
        a -= Kilo::new(2);
        assert_eq!(Kilo::new(1), a);
    }

    #[test]
    fn mul_assign_bits() {
        let mut a = Kilo::<P5>::new(2);
        a *= 3;
        assert_eq!(Kilo::new(6), a);
    }

    #[test]
    fn div_assign_bits() {
        let mut a = Kilo::<P9>::new(6);
        a /= 2;
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn rem_assign() {
        let mut a = Kilo::<P9>::new(6);
        a %= Milli::new(5);
        assert_eq!(Kilo::new(1), a);
    }

    /*
    #[test]
    fn rem_assign_bits() {
        let mut a = Kilo::new(6);
        a %= 5;
        assert_eq!(Kilo::new(1), a);
    }
     */
}
