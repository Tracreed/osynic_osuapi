use crate::v2::model::beatmap::structs::beatmap_playcount::BeatmapPlaycount;
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserBeatmapsResponse {
    pub beatmapsets: Option<Vec<Beatmapset>>,
    pub beatmap_playcounts: Option<Vec<BeatmapPlaycount>>,
    pub error: Option<String>,
}
