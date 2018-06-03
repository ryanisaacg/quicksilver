//! A collection of general utilities

use error::QuicksilverError;
use futures::{Async, Future, Poll};
use std::path::Path;
#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;
#[cfg(target_arch="wasm32")]
use {
    std::{
        error::Error,
        fmt,
        io::{Error as IOError, ErrorKind},
    },
    stdweb::{
        web::TypedArray,
        Value
    }
};

/// A Future that loads a file into an owned Vec of bytes
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading
#[derive(Debug)]
pub struct FileLoader {
    #[cfg(not(target_arch="wasm32"))]
    path: PathBuf, 
    #[cfg(target_arch="wasm32")]
    xhr: Value
}

impl FileLoader {
    /// Create a FileLoader for a given path
    pub fn load<P: AsRef<Path>>(path: P) -> FileLoader {
        FileLoader::new_impl(path)
    }

    #[cfg(not(target_arch="wasm32"))]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        FileLoader {
            path: PathBuf::from(path.as_ref())
        }
    }
    
    #[cfg(target_arch="wasm32")]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        let xhr = js! {
            let xhr = new XMLHttpRequest();
            xhr.open("GET", @{path.as_ref().to_str().unwrap()});
            xhr.send();
            xhr.responseType = "arraybuffer";
            return xhr;
        };
        FileLoader { xhr }
    }
}

impl Future for FileLoader {
    type Item = Vec<u8>;
    type Error = QuicksilverError;

    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use std::fs::File;
        use std::io::Read;
        let mut data = Vec::new();
        File::open(&self.path)?.read_to_end(&mut data)?;
        Ok(Async::Ready(data))
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let status = js! {
            return Math.floor(@{self.xhr}.status / 100);
        }; 
        let status = if let Value::Number(value) = status { value } else { unreachable!() };
        if status == 0 {
            Ok(Async::NotReady)
        } else if status == 2 { 
            let response = js! { 
                return new Uint8Array(@{self.xhr}.response);
            };
            let response = if let Value::Reference(reference) = response { reference } else { unreachable!() };
            let array: TypedArray<u8> = response.downcast().unwrap();
            Ok(Async::Ready(array.to_vec()))
        } else {
            Err(IOError::new(ErrorKind::NotFound, Box::new(WasmIOError)).into())
        }
    }
}

#[derive(Debug)]
#[cfg(target_arch="wasm32")]
struct WasmIOError;

#[cfg(target_arch="wasm32")]
impl fmt::Display for WasmIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(target_arch="wasm32")]
impl Error for WasmIOError {
    fn description(&self) -> &str {
        "An error occurred during a file IO operation"
    }
}
