#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(feature="serde_json")] 
extern crate serde_json;
#[cfg(all(not(target_arch="wasm32"), feature="rodio"))] 
extern crate rodio;

use graphics::{AtlasError, ImageError};
#[cfg(feature="rusttype")] use rusttype::Error as FontError;
#[cfg(feature="serde_json")] use serde_json::Error as SerdeError;
#[cfg(feature="sounds")] use sound::SoundError;
use std::{
    fmt,
    io::Error as IOError,
    error::Error
};

#[derive(Debug)]
/// An error generated by some Quicksilver subsystem
pub enum QuicksilverError {
    /// An error from an image atlas
    AtlasError(AtlasError),
    /// An error from loading an image
    ImageError(ImageError),
    /// An error from loading a sound
    #[cfg(feature="sounds")] SoundError(SoundError),
    /// An error from loading a file
    IOError(IOError),
    /// A serialize or deserialize error
    #[cfg(feature="serde_json")] SerdeError(SerdeError),
    /// There was an error loading a font file
    #[cfg(feature="rusttype")] FontError(FontError)
}

impl fmt::Display for QuicksilverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for QuicksilverError {
    fn description(&self) -> &str {
        match self {
            &QuicksilverError::AtlasError(ref err) => err.description(),
            &QuicksilverError::ImageError(ref err) => err.description(),
            &QuicksilverError::SoundError(ref err) => err.description(),
            &QuicksilverError::IOError(ref err) => err.description(),
            &QuicksilverError::SerdeError(ref err) => err.description(),
            &QuicksilverError::FontError(ref err) => err.description()
        }
    }
    
    fn cause(&self) -> Option<&Error> {
        match self {
            &QuicksilverError::AtlasError(ref err) => Some(err),
            &QuicksilverError::ImageError(ref err) => Some(err),
            &QuicksilverError::SoundError(ref err) => Some(err),
            &QuicksilverError::IOError(ref err) => Some(err),
            &QuicksilverError::SerdeError(ref err) => Some(err),
            &QuicksilverError::FontError(ref err) => Some(err)
        }
    }
}

#[doc(hidden)]
impl From<ImageError> for QuicksilverError {
    fn from(err: ImageError) -> QuicksilverError {
        QuicksilverError::ImageError(err)
    }
}

#[doc(hidden)]
#[cfg(feature="sounds")]
impl From<SoundError> for QuicksilverError {
    fn from(err: SoundError) -> QuicksilverError {
        QuicksilverError::SoundError(err)
    }
}

#[doc(hidden)]
impl From<AtlasError> for QuicksilverError {
    fn from(err: AtlasError) -> QuicksilverError {
        QuicksilverError::AtlasError(err)
    }
}

impl From<IOError> for QuicksilverError {
    fn from(err: IOError) -> QuicksilverError {
        QuicksilverError::IOError(err)
    }
}

#[cfg(feature="serde_json")]
impl From<SerdeError> for QuicksilverError {
    fn from(err: SerdeError) -> QuicksilverError {
        QuicksilverError::SerdeError(err)
    }
}

#[doc(hidden)]
#[cfg(not(target_arch="wasm32"))]
impl From<image::ImageError> for QuicksilverError {
    fn from(img: image::ImageError) -> QuicksilverError {
        let image_error: ImageError = img.into();
        image_error.into()
    }
}

#[doc(hidden)]
#[cfg(all(feature="sounds", not(target_arch="wasm32")))]
impl From<rodio::decoder::DecoderError> for QuicksilverError {
    fn from(snd: rodio::decoder::DecoderError) -> QuicksilverError {
        let sound_error: SoundError = snd.into();
        sound_error.into()
    }
}

#[doc(hidden)]
#[cfg(feature="rusttype")]
impl From<FontError> for QuicksilverError {
    fn from(fnt: FontError) -> QuicksilverError {
        QuicksilverError::FontError(fnt)
    }
}