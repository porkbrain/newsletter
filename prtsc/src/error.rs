//! There are two kinds of error in this service:
//! 1. Recoverable errors which are caused by e.g. malformed message body. When
//!    a recoverable error occurs, the service logs it, skips current message
//!    and polls for next one.
//! 2. Fatal errors which are cause by e.g. lost SQS or Browser connection. When
//!    a fatal error occurs, it is propagated to the main thread which throws is
//!    as it exists. We rely on supervision, such as k8s controller, that
//!    restarts failed jobs.

use image::ImageError;

use {
    fantoccini::error::CmdError,
    rusoto_core::RusotoError,
    std::{
        error::Error as StdError,
        fmt::{self, Debug, Display},
    },
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

impl From<CmdError> for Error {
    fn from(e: CmdError) -> Self {
        // likely a broken connection, we'd have to restart the client, but we
        // take advantage of supervision and just crash the service
        Self::fatal(e)
    }
}

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

impl From<ImageError> for Error {
    fn from(e: ImageError) -> Self {
        Self::fatal(e)
    }
}
