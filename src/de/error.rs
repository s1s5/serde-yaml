use std::fmt;

use serde::de;
use yaml_rust2::ScanError;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    YamlError(ScanError),
    ParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
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
        // write!(
        //     f,
        //     "Error({:?}, line: {}, column: {})",
        //     self.err.code.to_string(),
        //     self.err.line,
        //     self.err.column
        // )
        write!(f, "Error")
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Error {
        // make_error(msg.to_string())
        Error::ParseError
    }

    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        // Error::custom(format_args!(
        //     "invalid type: {}, expected {}",
        //     JsonUnexpected(unexp),
        //     exp,
        // ))
        Error::ParseError
    }

    #[cold]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        // Error::custom(format_args!(
        //     "invalid value: {}, expected {}",
        //     JsonUnexpected(unexp),
        //     exp,
        // ))
        Error::ParseError
    }
}
