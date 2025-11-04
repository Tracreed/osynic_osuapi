pub use crate::v2::model::oauth::structs::o_token::OToken;
use serde::{Deserialize, Deserializer, Serializer};
use std::sync::{Arc, Mutex};

pub fn serialize_arc_mutex_string<S>(val: &Arc<Mutex<String>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let guard = val.lock().unwrap();
    s.serialize_str(&guard)
}

pub fn deserialize_arc_mutex_string<'de, D>(d: D) -> Result<Arc<Mutex<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(Arc::new(Mutex::new(s)))
}

pub fn serialize_arc_mutex_o_token<S>(val: &Arc<Mutex<OToken>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let guard = val.lock().unwrap();
    s.serialize_some(&*guard)
}

pub fn deserialize_arc_mutex_o_token<'de, D>(d: D) -> Result<Arc<Mutex<OToken>>, D::Error>
where
    D: Deserializer<'de>,
{
    let o_token = OToken::deserialize(d)?;
    Ok(Arc::new(Mutex::new(o_token)))
}
