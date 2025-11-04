use serde::{Deserialize, Serialize};

use super::non_legacy_score::NonLegacyScore;

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonLegacyScores {
    pub scores: Vec<NonLegacyScore>,
}
