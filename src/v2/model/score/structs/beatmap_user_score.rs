use serde::{Deserialize, Serialize};

use super::score::Score;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapUserScore {
    pub position: Option<u32>,
    pub score: Option<Score>,
    pub error: Option<String>,
}
