use std::borrow::Cow;
use std::fmt;

use rocket::http::RawStr;
use rocket::request::FromParam;

use rand::{self, Rng};

use serde::{Deserialize, Serialize};

/// Table to retrieve base62 values from.
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// A _probably_ unique paste ID.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UniqueId(pub String);

impl UniqueId {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> UniqueId {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        UniqueId(id)
    }
}

/// Returns `true` if `id` is a valid paste ID and `false` otherwise.
fn valid_id(id: &str) -> bool {
    id.chars()
        .all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9'))
}

/// Returns an instance of `UniqueId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for UniqueId {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<UniqueId, &'a RawStr> {
        match valid_id(param) {
            true => Ok(UniqueId(param.to_string())),
            false => Err(param),
        }
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
