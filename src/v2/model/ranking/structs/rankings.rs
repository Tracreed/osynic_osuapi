use serde::{Deserialize, Serialize};

use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::ranking::structs::spotlight::Spotlight;
use crate::v2::model::user::structs::statistics::Statistics;
use crate::v2::model::user::structs::user::User;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KudosuRankings {
    pub ranking: Vec<User>,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rankings {
    pub beatmapsets: Option<Vec<Beatmapset>>,
    #[cfg_attr(feature = "export", tsify(type = "CursorInRankings"))]
    pub cursor: Cursor,
    pub ranking: Vec<Statistics>,
    pub spotlight: Option<Spotlight>,
    pub total: u64,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InRankings")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub page: u64,
}
