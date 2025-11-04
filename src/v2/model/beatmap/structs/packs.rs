use serde::{Deserialize, Serialize};

use super::pack::BeatmapPack;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatmapPacks {
    /// The beatmap packs of the beatmap packs.
    pub beatmap_packs: Vec<BeatmapPack>,
}
