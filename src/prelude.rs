//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic wrapper tuple struct rof newtype pattern
pub struct W<T>(pub T);

// pub use ste::format as f;