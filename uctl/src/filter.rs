pub mod ewma;
//pub mod lqe;
pub mod fir;

/// Filter trait
///
/// Each filter should implement this trait.
pub trait Filter {
    /// Input values type
    type Input;
    /// Output values type
    type Output;

    /// Apply filter to value and get result
    fn apply(&mut self, value: Self::Input) -> Self::Output;
}
