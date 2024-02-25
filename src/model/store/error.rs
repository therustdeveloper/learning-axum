//! src/model/store/error.rs

use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    FailToCreatePool(String),
}

// region:      --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// This allows us to use the question mark
impl std::error::Error for Error {}
// endregion:   --- Error Boilerplate