use super::{Fix, BitsType};

use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign, Add, Div, Mul, Neg, Rem, Sub};
use typenum::{Diff, Sum};

// Arithmetic.

impl<Bits, Base, Exp> Neg for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: Neg<Output = Bits::Type>
{
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.bits)
    }
}

impl<Bits, Base, Exp> Add for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: Add<Output = Bits::Type>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.bits + rhs.bits)
    }
}

impl<Bits, Base, Exp> Sub for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: Sub<Output = Bits::Type>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.bits - rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Mul<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where
    Bits: BitsType,
    Bits::Type: Mul<Output = Bits::Type>,
    LExp: Add<RExp>,
{
    type Output = Fix<Bits, Base, Sum<LExp, RExp>>;
    fn mul(self, rhs: Fix<Bits, Base, RExp>) -> Self::Output {
        Self::Output::new(self.bits * rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Div<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where
    Bits: BitsType,
    Bits::Type: Div<Output = Bits::Type>,
    LExp: Sub<RExp>
{
    type Output = Fix<Bits, Base, Diff<LExp, RExp>>;
    fn div(self, rhs: Fix<Bits, Base, RExp>) -> Self::Output {
        Self::Output::new(self.bits / rhs.bits)
    }
}

impl<Bits, Base, Exp> Rem for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
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
    Bits: BitsType,
    Bits::Type: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.bits += rhs.bits;
    }
}

impl<Bits, Base, Exp> SubAssign for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.bits -= rhs.bits;
    }
}

impl<Type, Bits, Base, Exp> MulAssign<Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: MulAssign<Type>,
{
    fn mul_assign(&mut self, rhs: Type) {
        self.bits *= rhs;
    }
}

impl<Type, Bits, Base, Exp> DivAssign<Type> for Fix<Bits, Base, Exp>
where
    Bits: BitsType,
    Bits::Type: DivAssign<Type>,
{
    fn div_assign(&mut self, rhs: Type) {
        self.bits /= rhs;
    }
}

impl<Bits, Base, LExp, RExp> RemAssign<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where
    Bits: BitsType,
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
        assert_eq!(Kilo::<i32>::new(15), Milli::new(15_000_000).convert());
    }

    #[test]
    fn convert_kilo_to_milli() {
        assert_eq!(Milli::<U32>::new(15_000_000), Kilo::new(15).convert());
    }

    #[test]
    fn cmp() {
        assert!(Kilo::<U3>::new(1) < Kilo::new(2));
    }

    #[test]
    fn neg() {
        assert_eq!(Kilo::<P32>::new(-1), -Kilo::new(1));
    }

    #[test]
    fn add() {
        assert_eq!(Kilo::<P8>::new(3), Kilo::new(1) + Kilo::new(2));
    }

    #[test]
    fn sub() {
        assert_eq!(Kilo::<i8>::new(1), Kilo::new(3) - Kilo::new(2));
    }

    #[test]
    fn mul() {
        assert_eq!(Unit::<P10>::new(6), Kilo::new(2) * Milli::new(3));
    }

    #[test]
    fn div() {
        assert_eq!(Unit::<U7>::new(3), Kilo::new(6) / Kilo::new(2));
    }

    #[test]
    fn rem() {
        assert_eq!(Kilo::<P16>::new(1), Kilo::new(6) % Kilo::new(5));
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
        let mut a = Kilo::<P18>::new(1);
        a += Kilo::new(2);
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn sub_assign() {
        let mut a = Kilo::<U18>::new(3);
        a -= Kilo::new(2);
        assert_eq!(Kilo::new(1), a);
    }

    #[test]
    fn mul_assign_bits() {
        let mut a = Kilo::<P18>::new(2);
        a *= 3;
        assert_eq!(Kilo::new(6), a);
    }

    #[test]
    fn div_assign_bits() {
        let mut a = Kilo::<P15>::new(6);
        a /= 2;
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn rem_assign() {
        let mut a = Kilo::<P15>::new(6);
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
