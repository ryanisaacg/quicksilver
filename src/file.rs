//! A collection of general utilities

extern crate futures;

use error::QuicksilverError;
use futures::{Async, Future, Poll};
use std::path::Path;
#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;

/// A Future that loads a file into an owned Vec of bytes
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading
#[derive(Debug)]
pub struct FileLoader {
    #[cfg(not(target_arch="wasm32"))]
    path: PathBuf, 
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
        FileLoader {
            path: PathBuf::from(path.as_ref())
        }
    }
    
    #[cfg(target_arch="wasm32")]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        use std::ffi::CString;
        use ffi::wasm;
        FileLoader {
            id: unsafe { wasm::load_file(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
        }
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
        use ffi::wasm;
        Ok(match wasm::asset_status(self.id)? {
            false => Async::NotReady,
            true => unsafe {
                let data = wasm::file_contents(self.id);
                let length = wasm::file_length(self.id) as usize;
                Async::Ready(Vec::from_raw_parts(data, length, length))
            }
        })
    }
}
