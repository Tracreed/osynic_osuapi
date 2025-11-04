// src/enums/grade_counts.rs

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Grade {
    #[serde(rename = "ss")]
    SS,
    #[serde(rename = "ssh")]
    SSH,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "sh")]
    SH,
    #[serde(rename = "a")]
    A,
}
