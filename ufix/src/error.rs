use thiserror::Error;

/// Fixed-point number error
#[derive(Error, Debug)]
pub enum Error {
    /// Number is too small to convert safely
    #[error("Number is too small to convert")]
    TooSmall,
    /// Number is too big to convert safely
    #[error("Number is too big to convert")]
    TooBig,
}

/// Fixed-point number result
pub type Result<T> = core::result::Result<T, Error>;
