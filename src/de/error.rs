use std::fmt;

use serde::de;
use yaml_rust2::ScanError;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    YamlError(ScanError),
    Custom(String),
    ParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::YamlError(s) => {
                write!(f, "Parse Error {s:?}")
            }
            Error::Custom(s) => {
                write!(f, "{s}")
            }
            Error::ParseError => {
                write!(f, "parse error")
            }
        }
    }
}

impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.err.code {
            ErrorCode::Io(err) => err.source(),
            _ => None,
        }
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::YamlError(s) => {
                write!(f, "Parse Error {s:?}")
            }
            Error::Custom(s) => {
                write!(f, "{s}")
            }
            Error::ParseError => {
                write!(f, "parse error")
            }
        }
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(format!("Error: {}", msg))
    }

    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp,))
    }

    #[cold]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!("invalid value: {}, expected {}", unexp, exp,))
    }
}
