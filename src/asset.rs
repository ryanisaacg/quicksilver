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

impl<T: Asset> Asset for Vec<T> {
    type Loading = Vec<LoadingAsset<T>>;
    type Error = Vec<T::Error>;
    
    fn update(mut loading: Vec<LoadingAsset<T>>) -> LoadingAsset<Self> {
        let (all_loaded, errored) = loading.iter_mut()
            .fold((true, false), |(mut loaded, mut errored), asset| {
                asset.update();
                match asset {
                    &mut LoadingAsset::Loaded(_) => (),
                    &mut LoadingAsset::Errored(_) => errored = true,
                    &mut LoadingAsset::Loading(_) => loaded = false
                }
                (loaded, errored)
            });
        if errored {
            let mut list = Vec::new();
            for asset in loading.iter() {
                if let &LoadingAsset::Errored(ref err) = asset {
                    list.push(err.clone());
                }
            }
            LoadingAsset::Errored(list)
        } else if all_loaded {
            LoadingAsset::Loaded(loading.iter()
                .map(|asset| if let &LoadingAsset::Loaded(ref asset) = asset {
                    asset.clone()
                } else {
                    unreachable!();
                }).collect())
        } else {
            LoadingAsset::Loading(loading)
        }
    }
}
