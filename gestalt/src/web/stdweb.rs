use crate::SaveError;
use stdweb::web::window;

pub fn set_storage(is_local: bool, profile: &str, value: &str) -> Result<(), SaveError> {
    let storage = if is_local {
        window().local_storage()
    } else {
        window().session_storage()
    };

    storage
        .insert(profile, value)
        .map_err(|_| SaveError::SaveWriteFailed)
}

pub fn get_storage(is_local: bool, profile: &str) -> Result<String, SaveError> {
    let storage = if is_local {
        window().local_storage()
    } else {
        window().session_storage()
    };

    storage
        .get(profile)
        .ok_or_else(|| SaveError::SaveNotFound(profile.to_string()))
}
