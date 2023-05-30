//! Crate prelude

pub use std::error::Error;

pub type Result<T> = core::result::Result<T, dyn Error>;

// Generic wrapper tuple struct rof newtype pattern
pub struct W<T>(pub T);

// pub use ste::format as f;