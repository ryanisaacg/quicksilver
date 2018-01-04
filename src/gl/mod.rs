#[cfg(not(target_arch="wasm32"))]
mod desktop;
#[cfg(target_arch="wasm32")]
mod wasm;

#[cfg(not(target_arch="wasm32"))]
pub use self::desktop::*;
#[cfg(target_arch="wasm32")]
pub use self::wasm::*;
