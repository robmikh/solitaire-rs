#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Error(pub windows::core::Error);

impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Error(error)
    }
}

impl From<Error> for windows::core::Error {
    fn from(error: Error) -> Self {
        error.0
    }
}
