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
    std::io::{Error as IOError, ErrorKind},
    stdweb::{
        Reference,
        unstable::TryInto,
        web::{XmlHttpRequest, TypedArray},
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
        future::result(create_request(path.as_ref().to_str().expect("The path must be able to be stringified")))
            .and_then(|xhr| future::poll_fn(move || {
                let status = xhr.status();
                match status / 100 {
                    0 => Ok(Async::NotReady),
                    2 => {
                        let response: Reference = xhr.raw_response().try_into().expect("The response will always be a JS object");
                        let array: TypedArray<u8> = match response.downcast() {
                            Some(array) => array,
                            None => return Err(new_wasm_error("Failed to cast file into bytes"))
                        };
                        Ok(Async::Ready(array.to_vec()))
                    },
                    _ => Err(new_wasm_error("Non-200 status code returned"))
                }
            }))
    };
}

fn create_request(path: &str) -> Result<XmlHttpRequest, QuicksilverError> {
    let xhr = XmlHttpRequest::new();
    web_try(xhr.open("GET", path), "Failed to create a GET request")?;
    web_try(xhr.send(), "Failed to send a GET request")?;
    js! { @{&xhr}.responseType = "arraybuffer"; }
    Ok(xhr)
}

#[cfg(target_arch="wasm32")]
fn web_try<T, E>(result: Result<T, E>, error: &str) -> Result<T, QuicksilverError> {
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(new_wasm_error(error))
    }
}


#[cfg(target_arch="wasm32")]
fn new_wasm_error(string: &str) -> QuicksilverError {
    IOError::new(ErrorKind::NotFound, string).into()
}

#[cfg(not(target_arch="wasm32"))]
fn load(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;
    Ok(data)
}

