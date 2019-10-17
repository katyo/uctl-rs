use crate::{PhantomData};

/// Transducer trait
///
/// When applied transducer consumes some input value and produce corresponding output result.
///
/// Each filter and regulator should implement this trait.
pub trait Transducer {
    /// Input values type
    type Input;
    /// Output values type
    type Output;

    /// Apply transformation to the input value and output result
    fn apply(&self, value: Self::Input) -> Self::Output;
}

macro_rules! transducer_tuple {
    ( $rtype:tt, $type0:tt => $field0:tt, $( $typeN:tt : $ptypeN:tt => $fieldN:tt ),+) => {
        impl<$type0, $($typeN),+> Transducer for ($type0, $($typeN),+)
        where
            $type0: Transducer,
            $($typeN: Transducer<Input = $ptypeN::Output>),+
        {
            type Input = $type0::Input;
            type Output = $rtype::Output;

            fn apply(&self, value: Self::Input) -> Self::Output {
                let value = self.$field0.apply(value);
                $(
                    let value = self.$fieldN.apply(value);
                )+
                    value
            }
        }
    }
}

transducer_tuple!(B, A => 0, B: A => 1);
transducer_tuple!(C, A => 0, B: A => 1, C: B => 2);
transducer_tuple!(D, A => 0, B: A => 1, C: B => 2, D: C => 3);
transducer_tuple!(E, A => 0, B: A => 1, C: B => 2, D: C => 3, E: D => 4);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FnTransducer<I, O, F>(F, PhantomData<(I, O)>);

impl<I, O, F: Fn(I) -> O> From<F> for FnTransducer<I, O, F> {
    fn from(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<I, O, F: Fn(I) -> O> Transducer for FnTransducer<I, O, F> {
    type Input = I;
    type Output = O;

    fn apply(&self, value: Self::Input) -> Self::Output {
        self.0(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn inc(v: i8) -> i16 {
        v as i16 + 1
    }

    fn dbl(v: i16) -> i32 {
        v as i32 * 2
    }

    #[test]
    fn func() {
        assert_eq!(FnTransducer::from(inc).apply(1), 2);
    }

    #[test]
    fn pipe2() {
        let c = (FnTransducer::from(inc), FnTransducer::from(dbl));

        assert_eq!(c.apply(1), 4);
    }
}
