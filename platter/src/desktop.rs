use std::{fs::read, future::Future, io::Error as IOError, path::Path};

pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    futures_util::future::ready(read(path))
}
