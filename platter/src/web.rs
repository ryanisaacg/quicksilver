use std::{
    future::Future,
    io::{Error as IOError, ErrorKind},
    path::Path,
};

#[cfg(all(feature = "stdweb", feature = "web_sys"))]
compile_error!("stdweb and web_sys may not both be enabled at once, please pick one");

#[cfg(all(not(feature = "stdweb"), not(feature = "web_sys")))]
compile_error!("Please enable one of stdweb or web_sys to compile for wasm");

#[cfg(feature = "stdweb")]
#[path = "web/stdweb.rs"]
mod backend;

#[cfg(feature = "web_sys")]
#[path = "web/web_sys.rs"]
mod backend;

pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    let path = path
        .as_ref()
        .to_str()
        .expect("The path must be able to be stringified");

    backend::make_request(path)
}

fn web_try<T, E>(result: Result<T, E>, error: &str) -> Result<T, IOError> {
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(new_wasm_error(error)),
    }
}

fn new_wasm_error(string: &str) -> IOError {
    IOError::new(ErrorKind::NotFound, string)
}
