//! The module that manages control flow of Quicksilver applications

mod application;
mod asset;
mod event;
mod gamepad_provider;
mod run;
mod state;

pub use self::{
    asset::Asset,
    event::Event,
    run::run,
    state::State
};
pub(crate) use self::{
    application::Application,
    gamepad_provider::GamepadProvider,
};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use self::event::EventProvider;
