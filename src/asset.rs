//! A module that allows loading of assets
//!
//! Even though the desktop loads assets synchronously, the web loads them asynchronously, so this
//! module presents a universal API for both. Additionally, the desktop may load with async at some
//! point.

use std::path::Path;

#[cfg(target_arch="wasm32")]
extern "C" {
    fn is_loaded(index: u32) -> bool;
    fn is_errored(index: u32) -> bool;
}

///A trait that allows an asset to be asynchronously loaded
///
///Currently on desktop assets are actually loaded synchronously, but on the web they are loaded
///async.
pub trait Loadable: Sized + Clone {
    ///The error type to return when an asset fails to load
    type Error: Clone;

    ///Load an asset from a path
    fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Self>;

    #[cfg(target_arch="wasm32")]
    ///Parse the loaded webassembly result
    fn parse_result(handle: LoadingHandle, loaded: bool, errored: bool) -> LoadingAsset<Self>;
}

/// Update all assets in a given mutable slice
///
/// If any asset generated an error while loading, that is returned; if all assets have finished
/// loading, a Vec of their loaded states is returned; otherwise nothing is returned.
pub fn update_all<T: Loadable>(assets: &mut [&mut LoadingAsset<T>]) -> Result<Option<Vec<T>>, T::Error> {
    let (all_loaded, error) = assets.iter_mut().fold((true, None), |(mut loaded, mut error), asset| {
        asset.update();
        match asset {
            &mut &mut LoadingAsset::Loaded(_) => (),
            &mut &mut LoadingAsset::Errored(ref err) => error = Some(err.clone()),
            &mut &mut LoadingAsset::Loading(_) => loaded = false
        }
        (loaded, error)
    });
    if let Some(err) = error {
        Err(err)
    } else if all_loaded {
        Ok(Some(assets.iter().map(|item| 
            match item {
                &&mut LoadingAsset::Loaded(ref asset) => asset.clone(),
                _ => unreachable!()
            }).collect()))
    } else {
        Ok(None)
    }
}


#[derive(Clone)]
///An opaque object that represents the still-loading state of an asset
pub struct LoadingHandle(pub(crate) u32);

#[derive(Clone)]
///A wrapper for an asset that is loading, errored, or loaded
pub enum LoadingAsset<Asset: Loadable> {
    ///An asset that is not finished loading
    Loading(LoadingHandle), 
    ///The asset has successfully loaded
    Loaded(Asset),
    ///Some error has occurred while trying to load
    Errored(Asset::Error)
}

impl<T: Loadable> LoadingAsset<T> {
    ///Update the loading asset possibly produced a loaded state or an error
    pub fn update(&mut self) {
        self.update_impl();
    }

    #[cfg(not(target_arch="wasm32"))]
    fn update_impl(&mut self) {}

    #[cfg(target_arch="wasm32")]
    fn update_impl(&mut self) {
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

