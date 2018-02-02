extern crate futures;

use futures::{Async, Future, Poll};
use std::path::Path;

pub struct FileLoader {
    #[cfg(not(target_arch="wasm32"))]
    data: Result<String, ()>,
    #[cfg(target_arch="wasm32")]
    id: u32
}

impl FileLoader {
    pub fn new<P: AsRef<Path>>(path: P) -> FileLoader {
        FileLoader::new_impl(path)
    }

    #[cfg(not(target_arch="wasm32"))]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        use std::fs::File;
        use std::io::Read;
        let data = String::new();
        if let Ok(file) = File::open(path) {
            if let Ok(_) = file.read_to_string(&mut data) {
                FileLoader { data: Ok(data) }
            } else {
                FileLoader { data: Err(()) }
            }
        } else {
            FileLoader { data: Err(()) }
        }        
    }
    
    #[cfg(target_arch="wasm32")]
    fn new_impl<P: AsRef<Path>>(path: P) -> FileLoader {
        use std::ffi::CString;
        use std::os::raw::c_char;
        extern "C" {
            fn load_text_file(path: *mut c_char) -> u32;
        }
        ImageLoader {
            id: unsafe { load_text_file(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
        }
    }
}

impl Future for FileLoader {
    type Item = String;
    type Error = ();

    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<String, ()> {
        match self.data {
            Ok(ref data) => Ok(Async::Ready(data.clone())),
            Err(_) => Err(())
        }
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<String, ()> {
        use std::ffi::CString;
        use std::os::raw::c_char;
        extern "C" {
            fn is_text_file_loaded(id: u32);
            fn is_text_file_errored(id: u32);
            fn text_file_contents(id: u32) -> *mut c_char;
        }
        if is_text_file_loaded(self.id) {
            if is_text_file_errored(self.id) {
                Err(())
            } else {
                Ok(Async::Ready(unsafe { 
                    CString::from_raw(text_file_contents(self.id)) 
                }.into_string().unwrap()))
            }
        } else {
            Ok(Async::NotReady)
        }
    }
}
