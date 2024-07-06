use std::io;

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // #[error(transparent)]
    // Tr(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    DBus(#[from] dbus::Error),

    #[error("Player Get failed: {msg}")]
    Player { msg: String },

    // #[error(transparent)]
    // ValidatorError(#[from] validator::ValidationError),

    // #[error("unknown")]
    // Unknown,
    #[error("Weather need key of api.")]
    WeatherKeyError,

    #[error("Weather get fail.")]
    WeatherFailError,

    #[error("Weather Request failed:{code}")]
    WeatherResponseError { code: StatusCode },
}

impl<T> From<Error> for crate::Result<T> {
    fn from(val: Error) -> Self {
        Err(Box::new(val))
    }
}
