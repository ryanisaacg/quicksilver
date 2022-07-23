//! A simple utility to serve you files on a `platter`
//!
//! `platter` works on both desktop and web, and returns a byte buffer of the file's contents.
//! On desktop, `load_file` is backed by native file system APIs. On web, it is backed by an
//! HTTP 'GET' request.
//!
//! To use `platter` on the web, you'll need to choose either the `stdweb` or `web-sys` feature and
//! enable it. This determines which method of binding to browser APIs `platter` will use.

#![deny(
    bare_trait_objects,
    missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

use std::{future::Future, io::Error as IOError, path::Path};

/// Create a Future that loads a file into an owned Vec of bytes
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading.
///
/// On desktop, the file will be loaded from disk relative to the current directory, and the Future
/// will return instantly. On the web, a GET request will be made to the stringified version of the
/// `path` and the Future will return when the HTTP request resolves.
pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    platform::load_file(path)
}

// Select which platform implementation to use based on provided features

#[cfg(not(target_arch = "wasm32"))]
#[path = "desktop.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod platform;
