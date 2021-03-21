use std::error;
use std::fmt;
use std::io;
use toml;

pub type InternalResult<T> = Result<T, InternalError>;

#[derive(Debug)]
pub enum InternalError {
    Generic(String),
    IOError(io::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(desc) => f.pad(desc.as_ref()),
            Self::IOError(e) => e.fmt(f),
            Self::TomlDeError(e) => e.fmt(f),
            Self::TomlSerError(e) => e.fmt(f),
        }
    }
}

impl error::Error for InternalError {
    fn description(&self) -> &str {
        match self {
            Self::Generic(desc) => desc.as_ref(),
            Self::IOError(_) => "IO error",
            Self::TomlDeError(_) => "Deserialization error from toml",
            Self::TomlSerError(_) => "Serialization error into toml",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Self::Generic(_desc) => None,
            Self::IOError(e) => Some(e),
            Self::TomlDeError(e) => Some(e),
            Self::TomlSerError(e) => Some(e),
        }
    }
}

impl From<io::Error> for InternalError {
    fn from(io_err: io::Error) -> Self {
        Self::IOError(io_err)
    }
}

impl From<toml::de::Error> for InternalError {
    fn from(toml_err: toml::de::Error) -> Self {
        Self::TomlDeError(toml_err)
    }
}

impl From<toml::ser::Error> for InternalError {
    fn from(toml_err: toml::ser::Error) -> Self {
        Self::TomlSerError(toml_err)
    }
}
