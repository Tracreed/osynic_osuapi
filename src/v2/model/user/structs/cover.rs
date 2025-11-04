// src/structs/cover.rs
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}
