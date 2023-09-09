use std::fmt::Debug;

use thiserror::Error;
use veilid_core::VeilidAPIError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to access or find the home directory")]
    HomeDir,
    #[error("failed to convert {from:?} into {into}")]
    Conversion { from: String, into: String },
    #[error("{whats_missing} does not exist")]
    NotFound { whats_missing: String },
}

impl From<Error> for VeilidAPIError {
    fn from(value: Error) -> Self {
        VeilidAPIError::Generic {
            message: value.to_string(),
        }
    }
}
