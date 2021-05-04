use shared::rusoto_core::RusotoError;
use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display},
    io,
};

#[derive(Debug)]
pub struct Error {
    message: String,
    is_recoverable: bool,
}

impl Error {
    /// This error gets logged but service continues polling sqs.
    pub fn new(reason: impl Display) -> Self {
        Self {
            message: reason.to_string(),
            is_recoverable: true,
        }
    }

    /// The service terminates.
    pub fn fatal(reason: impl Display) -> Self {
        Self {
            message: reason.to_string(),
            is_recoverable: false,
        }
    }

    pub fn is_recoverable(&self) -> bool {
        self.is_recoverable
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::new(e)
    }
}

impl<E: Debug> From<RusotoError<E>> for Error {
    fn from(e: RusotoError<E>) -> Self {
        Self::fatal(format!("AWS err: {:?}", e))
    }
}

impl From<envy::Error> for Error {
    fn from(e: envy::Error) -> Self {
        Self::fatal(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::fatal(e)
    }
}

impl From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Self {
        Self::fatal(e)
    }
}
