extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Error;

///Save some arbitrary data to the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn save<T: Serialize>(profile: &str, data: T) {
    //todo: save the data
}

///Load some data from the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn load<T: Deserialize>(profile: &str) -> Result<T, Error> {
    //todo: load the data
}
