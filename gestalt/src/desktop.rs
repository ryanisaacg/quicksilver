use super::{Location, SaveError};
use serde::{Deserialize, Serialize};
use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::PathBuf,
};

pub fn save<T: Serialize>(
    location: Location,
    appname: &str,
    profile: &str,
    data: &T,
) -> Result<(), SaveError> {
    DirBuilder::new()
        .recursive(true)
        .create(get_save_folder(location, appname)?)?;

    Ok(serde_json::to_writer(
        File::create(get_save_location(location, appname, profile)?)?,
        data,
    )?)
}

pub fn save_raw(
    location: Location,
    appname: &str,
    profile: &str,
    data: &[u8],
) -> Result<(), SaveError> {
    DirBuilder::new()
        .recursive(true)
        .create(get_save_folder(location, appname)?)?;

    Ok(File::create(get_save_location(location, appname, profile)?)?.write_all(data)?)
}

pub fn load<T>(location: Location, appname: &str, profile: &str) -> Result<T, SaveError>
where
    for<'de> T: Deserialize<'de>,
{
    Ok(serde_json::from_reader(File::open(get_save_location(
        location, appname, profile,
    )?)?)?)
}

pub fn load_raw(location: Location, appname: &str, profile: &str) -> Result<Vec<u8>, SaveError> {
    let mut buf = Vec::new();
    File::open(get_save_location(location, appname, profile)?)?.read_to_end(&mut buf)?;

    Ok(buf)
}

fn get_save_folder(location: Location, appname: &str) -> Result<PathBuf, SaveError> {
    let mut path = match location {
        Location::Cache => dirs::cache_dir(),
        Location::Config => dirs::config_dir(),
        Location::Data => dirs::data_dir(),
    }
    .ok_or(SaveError::SaveLocationNotFound)?;
    path.push(appname);

    Ok(path)
}

fn get_save_location(
    location: Location,
    appname: &str,
    profile: &str,
) -> Result<PathBuf, SaveError> {
    let mut path = get_save_folder(location, appname)?;
    path.push(profile);
    Ok(path)
}
