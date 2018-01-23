//! A module that allows loading of assets
//!
//! Even though the desktop generally loads assets synchronously, the web loads them asynchronously, so this
//! module presents a universal API for both. Additionally, the desktop may load with async at some
//! point.

///A trait that allows an asset to be asynchronously loaded
///
///Currently on desktop assets are actually loaded synchronously, but on the web they are loaded
///async.
pub trait Asset: Sized + Clone {
    ///The intermediary loading type
    type Loading: Clone;
    ///The error type to return when an asset fails to load
    type Error: Clone;

    ///Check to see if the asset has finished loading
    fn update(loading: Self::Loading) -> LoadingAsset<Self>;
}

/// Update all assets in a given mutable slice
///
/// If all assets have finished loading, a Vec of all the assets is returned. Otherwise None is
/// returned.
pub fn update_all<T: Asset>(assets: &mut [&mut LoadingAsset<T>]) -> Option<Vec<T>> {
    let all_loaded = assets.iter_mut().fold(true, |mut loaded, asset| {
        asset.update();
        match asset {
            &mut &mut LoadingAsset::Loaded(_) => (),
            _ => loaded = false
        }
        loaded
    });
    if all_loaded {
        Some(assets.iter().map(|item| 
            match item {
                &&mut LoadingAsset::Loaded(ref asset) => asset.clone(),
                _ => unreachable!()
            }).collect())
    } else {
        None
    }
}


#[derive(Clone)]
///A wrapper for an asset that is loading, errored, or loaded
pub enum LoadingAsset<T: Asset> {
    ///An asset that is not finished loading
    Loading(T::Loading), 
    ///The asset has successfully loaded
    Loaded(T),
    ///Some error has occurred while trying to load
    Errored(T::Error)
}

impl<T: Asset> LoadingAsset<T> {
    ///Update the loading asset possibly produced a loaded state or an error
    pub fn update(&mut self) {
        *self = match self.clone() {
            LoadingAsset::Loading(handle) => T::update(handle),
            LoadingAsset::Errored(error) => LoadingAsset::Errored(error),
            LoadingAsset::Loaded(result) => LoadingAsset::Loaded(result)
        }
    }
}


