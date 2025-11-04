use crate::v2::model::score::structs::cursor::Cursor;
use crate::v2::model::score::structs::non_legacy_score::NonLegacyScore;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScoresResponse {
    pub scores: Vec<NonLegacyScore>,
    #[cfg_attr(feature = "export", tsify(type = "CursorInScore"))]
    pub cursor: Cursor,
    pub cursor_string: String,
}
