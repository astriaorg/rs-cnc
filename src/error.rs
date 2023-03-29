use std::result::Result as StdResult;

use anyhow::Error as AnyhowError;

/// A specialized `Result` type for rs-cnc.
pub type Result<T> = StdResult<T, AnyhowError>;
