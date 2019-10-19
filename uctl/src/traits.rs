use core::marker::PhantomData;

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

    /// Params type
    type Param;
    /// State type
    type State;

    /// Apply transformation to the input value and output result
    fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output;
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
            type Param = ($type0::Param, $($typeN::Param),+);
            type State = ($type0::State, $($typeN::State),+);

            fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output {
                let value = $type0::apply(&param.$field0, &mut state.$field0, value);
                $(
                    let value = $typeN::apply(&param.$fieldN, &mut state.$fieldN, value);
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

/*
#[derive(Debug, Clone, Copy)]
pub struct FnTransducer<I, O, F>(PhantomData<(I, O, F)>);

impl<I, O, F: Fn(I) -> O> Transducer for FnTransducer<I, O, F> {
    type Input = I;
    type Output = O;
    type Param = F;
    type State = ();

    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        param(value)
    }
}
 */

pub struct FnTransducer<I, O>(PhantomData<(I, O)>);

impl<I, O> Transducer for FnTransducer<I, O> {
    type Input = I;
    type Output = O;
    type Param = fn(I) -> O;
    type State = ();

    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        param(value)
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
        type C = FnTransducer<i8, i16>;

        assert_eq!(C::apply(&(inc as fn(_) -> _), &mut (), 1), 2);
    }

    #[test]
    fn pipe2() {
        type C = (FnTransducer<i8, i16>, FnTransducer<i16, i32>);

        assert_eq!(C::apply(&(inc, dbl), &mut ((), ()), 1), 4);
    }
}
