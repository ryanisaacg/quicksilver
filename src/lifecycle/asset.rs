use {Async, Future, error::QuicksilverError, Result};

/// A structure to manage the loading and use of a future
pub struct Asset<T>(AssetData<T>);

enum AssetData<T> {
    Loading(Box<dyn Future<Item = T, Error = QuicksilverError>>),
    Loaded(T)
}

impl<T> Asset<T> {
    /// Create a new asset from a future
    pub fn new(future: impl Future<Item = T, Error = QuicksilverError> + 'static) -> Asset<T> {
        Asset(AssetData::Loading(Box::new(future)))
    }

    /// Run a function if the loading is complete
    pub fn execute(&mut self, loaded: impl FnOnce(&mut T) -> Result<()>) -> Result<()> {
        self.execute_or(loaded, || Ok(()))
    }

    /// Run a function if the loading is complete, or a different function if it isn't
    pub fn execute_or(&mut self, loaded: impl FnOnce(&mut T) -> Result<()>, loading: impl FnOnce() -> Result<()>) -> Result<()> {
        let result = match self.0 {
            AssetData::Loading(ref mut asset) => asset.poll()?,
            _ => Async::NotReady
        };
        if let Async::Ready(asset) = result {
            self.0 = AssetData::Loaded(asset);
        }
        match self.0 {
            AssetData::Loading(_) => loading(),
            AssetData::Loaded(ref mut data) => loaded(data)
        }
    }
}