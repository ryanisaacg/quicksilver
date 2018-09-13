//! Quicksilver's sound capabilities are currently somewhat limited: sound can be loaded and played
//! at various volumes.
//!
//! `Sound::load` creates a `Future` that resolves to a `Sound`, and each
//! `Sound` instance provides `volume` to query volume, `set_volume` to set the volume, and `play`
//! to play the sound. Volume will not be applied to currently-playing sounds, and multiple
//! different sound clips played will overlap.
//!
//! Quicksilver's sound capabilities (and by extension this tutorial) are planned to be expanded in
//! an upcoming release.
