use std::result::Result as StdResult;

pub use crate::{conf::Conf, error::Error};

pub type Result<T> = StdResult<T, Error>;
