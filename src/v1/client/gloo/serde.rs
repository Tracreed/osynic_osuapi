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
