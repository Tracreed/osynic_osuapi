// src/enums/playstyle.rs

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Playstyle {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "keyboard")]
    Keyboard,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "touch")]
    Touch,
}
