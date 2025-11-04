// MultiplayerScores

// An object which contains scores and related data for fetching next page of the result.
// Field 	Type 	Description
// cursor_string 	CursorString 	To be used to fetch the next page.
// params 	object 	Parameters used for score listing.
// scores 	Score[]
// total 	integer? 	Index only. Total scores of the specified playlist item.
// user_score 	Score? 	Index only. Score of the accessing user if exists.

use serde::{Deserialize, Serialize};

use crate::v2::model::score::structs::multiplayer::multiplayer_score::MultiplayerScore;

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiplayerScores {
    pub cursor_string: Option<String>,
    pub params: Params,
    pub scores: Vec<MultiplayerScore>,
    pub total: Option<u32>,
    pub user_score: Option<MultiplayerScore>,
    #[cfg_attr(feature = "wasm", tsify(type = "CursorInMultiplayerScores | null"))]
    pub cursor: Option<Cursor>,
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InMultiplayerScores")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub total_score: Option<u32>,
    pub score_id: Option<u64>,
}

// wried docs...
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub limit: Option<u32>,
    pub sort: Option<String>,
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiplayerScoresAround {
    pub higher: MultiplayerScores,
    pub lower: MultiplayerScores,
}
