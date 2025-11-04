// "difficulty_range": {
// 	"max": 6.46,
// 	"min": 4.23
// },

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultyRange {
    pub max: f64,
    pub min: f64,
}
