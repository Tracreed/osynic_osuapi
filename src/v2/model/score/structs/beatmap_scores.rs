use serde::{Deserialize, Serialize};

use super::beatmap_user_score::BeatmapUserScore;
use super::score::Score;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapScores {
    pub scores: Vec<Score>,
    pub position: Option<BeatmapUserScore>,
}
