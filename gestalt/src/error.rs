use serde_json::Error as SerdeError;
use std::{error::Error, fmt, io::Error as IOError};
#[derive(Debug)]
/// An error that can occur during a save or load operation
pub enum SaveError {
    /// Some serialization failed during save or load
    SerdeError(SerdeError),
    /// Save string is failed to decode (web-specific)
    DecodeError,
    /// Some IO failed during save or load
    IOError(IOError),
    /// The user has no home directory so no save or load location can be established
    SaveLocationNotFound,
    /// The save cannot be written (web-specific)
    SaveWriteFailed,
    /// The save profile with the given name was not found
    ///
    /// On desktop this will more likely be reported as an IO error, but on web it will be a
    /// SaveNotFound
    SaveNotFound(String),
}

impl From<SerdeError> for SaveError {
    fn from(err: SerdeError) -> SaveError {
        SaveError::SerdeError(err)
    }
}

impl From<IOError> for SaveError {
    fn from(err: IOError) -> SaveError {
        SaveError::IOError(err)
    }
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SaveError {
    fn description(&self) -> &str {
        match self {
            SaveError::SerdeError(err) => err.description(),
            SaveError::DecodeError => "Save is not valid base64 string",
            SaveError::IOError(err) => err.description(),
            SaveError::SaveWriteFailed => "The save could not be written to local storage",
            SaveError::SaveLocationNotFound => "The current user has no home directory",
            SaveError::SaveNotFound(_) => "The given save profile was not found",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            SaveError::SerdeError(err) => Some(err),
            SaveError::IOError(err) => Some(err),
            SaveError::SaveLocationNotFound
            | SaveError::SaveWriteFailed
            | SaveError::SaveNotFound(_)
            | SaveError::DecodeError => None,
        }
    }
}
