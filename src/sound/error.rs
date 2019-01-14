#[derive(Debug)]
/// An error generated when loading a sound
pub enum SoundError {
    /// The sound file is not in an format that can be played
    UnrecognizedFormat,
    /// No output device was found to play the sound
    NoOutputAvailable,
    /// The Sound was not found or could not be loaded
    IOError(IOError)
}

impl fmt::Display for SoundError  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SoundError {
    fn description(&self) -> &str {
        match self {
            SoundError::UnrecognizedFormat => "The sound file format was not recognized",
            SoundError::NoOutputAvailable => "There was no output device available for playing",
            SoundError::IOError(err) => err.description()
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            SoundError::UnrecognizedFormat
                | SoundError::NoOutputAvailable => None,
            SoundError::IOError(err) => Some(err)
        }
    }

}

#[doc(hidden)]
#[cfg(not(target_arch="wasm32"))]
impl From<DecoderError> for SoundError {
    fn from(err: DecoderError) -> SoundError {
        match err {
            DecoderError::UnrecognizedFormat => SoundError::UnrecognizedFormat
        }
    }
}

#[doc(hidden)]
impl From<IOError> for SoundError {
    fn from(err: IOError) -> SoundError {
        SoundError::IOError(err)
    }
}
