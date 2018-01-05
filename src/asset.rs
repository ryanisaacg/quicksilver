use std::path::Path;

#[cfg(target_arch="wasm32")]
extern "C" {
    fn is_loaded(index: u32) -> bool;
    fn is_errored(index: u32) -> bool;
}

pub trait Loadable: Sized + Clone {
    type Error: Clone;

    fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Self>;
    #[cfg(target_arch="wasm32")]
    fn parse_result(handle: LoadingHandle, loaded: bool, errored: bool) -> LoadingAsset<Self>;
}

#[derive(Clone)]
pub struct LoadingHandle(pub(crate) u32);

#[derive(Clone)]
pub enum LoadingAsset<Asset: Loadable> {
    Loading(LoadingHandle), Loaded(Asset), Errored(Asset::Error)
}

impl<T: Loadable> LoadingAsset<T> {
    #[cfg(not(target_arch="wasm32"))]
    pub fn update(&mut self) {}
    #[cfg(target_arch="wasm32")]
    pub fn update(&mut self) {
        *self = match self.clone() {
            LoadingAsset::Loading(handle) =>  {
                let loaded = unsafe { is_loaded(handle.0) };
                let errored = unsafe { is_errored(handle.0) };
                T::parse_result(handle, loaded, errored)
            },
            LoadingAsset::Errored(error) => LoadingAsset::Errored(error),
            LoadingAsset::Loaded(result) => LoadingAsset::Loaded(result)
        }
    }
}


