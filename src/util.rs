//! A collection of general utilities

extern crate futures;

use futures::{Async, Future, Poll};
use std::io::ErrorKind as IOError;
use std::path::Path;

/// A Future that loads a file into an owned string
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading
pub struct FileLoader {
    #[cfg(not(target_arch="wasm32"))]
    data: Result<String, IOError>,
    #[cfg(target_arch="wasm32")]
    id: u32
}

impl FileLoader {
    /// Create a FileLoader for a given path
    pub fn load<P: AsRef<Path>>(path: P) -> FileLoader {
        FileLoader::new_impl(path)
    }

    #[cfg(not(target_arch="wasm32"))]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        use std::fs::File;
        use std::io::Read;
        let mut data = String::new();
        let data = match File::open(path) {
            Ok(ref mut file) => match file.read_to_string(&mut data) {
                Ok(_) => Ok(data),
                Err(err) => Err(err.kind())
            },
            Err(err) => Err(err.kind())
        };
        FileLoader { data }
    }
    
    #[cfg(target_arch="wasm32")]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        use std::ffi::CString;
        use ffi::wasm;
        FileLoader {
            id: unsafe { wasm::load_text_file(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
        }
    }
}

impl Future for FileLoader {
    type Item = String;
    type Error = IOError;

    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<String, IOError> {
        match self.data {
            Ok(ref data) => Ok(Async::Ready(data.clone())),
            Err(err) => Err(err)
        }
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<String, IOError> {
        use std::ffi::CString;
        use ffi::wasm;
        Ok(match wasm::asset_status(self.id)? {
            false => Async::NotReady,
            true => Async::Ready(unsafe { CString::from_raw(wasm::text_file_contents(self.id)) }.into_string().unwrap())
        })
    }
}
