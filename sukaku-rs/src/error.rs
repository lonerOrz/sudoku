//! Error types for Sukaku operations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid puzzle length: got {0}, expected 81")]
    InvalidLength(usize),

    #[error("invalid digit at position {0}: {1}")]
    InvalidDigit(usize, u8),

    #[error("puzzle has no solution")]
    NoSolution,

    #[error("puzzle has multiple solutions")]
    MultipleSolutions,

    #[error("generation failed")]
    GenerationFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
