use crate::SaveError;
use web_sys::window;

pub fn set_storage(is_local: bool, profile: &str, value: &str) -> Result<(), SaveError> {
    let window = window().expect("Failed to get window object");
    let storage = if is_local {
        window.local_storage()
    } else {
        window.session_storage()
    };
    let storage = storage
        .map_err(|_| SaveError::SaveLocationNotFound)?
        .ok_or(SaveError::SaveLocationNotFound)?;

    storage
        .set(profile, value)
        .map_err(|_| SaveError::SaveWriteFailed)
}

pub fn get_storage(is_local: bool, profile: &str) -> Result<String, SaveError> {
    let window = window().expect("Failed to get window object");
    let storage = if is_local {
        window.local_storage()
    } else {
        window.session_storage()
    };
    let storage = storage
        .map_err(|_| SaveError::SaveLocationNotFound)?
        .ok_or(SaveError::SaveLocationNotFound)?;

    storage
        .get(profile)
        .map_err(|_| SaveError::SaveLocationNotFound)?
        .ok_or_else(|| SaveError::SaveNotFound(profile.to_string()))
}
