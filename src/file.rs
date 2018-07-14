//! A collection of general utilities

use error::QuicksilverError;
use futures::{Future, future};
use std::path::Path;
#[cfg(not(target_arch="wasm32"))]
use {
    Result,
    std::{
        fs::File,
        io::Read,
    }
};
#[cfg(target_arch="wasm32")]
use {
    futures::Async,
    std::{
        error::Error,
        fmt,
        io::{Error as IOError, ErrorKind},
    },
    stdweb::{
        unstable::TryInto,
        web::TypedArray,
        Value
    }
};

/// Create a Future that loads a file into an owned Vec of bytes
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading
pub fn load_file(path: impl AsRef<Path>) -> impl Future<Item = Vec<u8>, Error = QuicksilverError> { 
    #[cfg(not(target_arch="wasm32"))]
    return future::result(load(path));
    
    #[cfg(target_arch="wasm32")]
    return {
        let xhr = js! {
            let xhr = new XMLHttpRequest();
            xhr.open("GET", @{path.as_ref().to_str().unwrap()});
            xhr.send();
            xhr.responseType = "arraybuffer";
            return xhr;
        };
        future::poll_fn(move || {
            let status = js! ( @{&xhr}.status );
            let status: i32 = status.try_into().unwrap();
            match status / 100 {
                0 => Ok(Async::NotReady),
                2 => {
                    let response = js! { 
                        return new Uint8Array(@{&xhr}.response);
                    };
                    let response = if let Value::Reference(reference) = response { reference } else { unreachable!() };
                    let array: TypedArray<u8> = response.downcast().unwrap();
                    Ok(Async::Ready(array.to_vec()))
                },
                _ => Err(IOError::new(ErrorKind::NotFound, Box::new(WasmIOError)).into())
            }
        })
    };
}

#[cfg(not(target_arch="wasm32"))]
fn load(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;
    Ok(data)
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
