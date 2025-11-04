use serde::{Deserialize, Serialize};

use crate::v2::model::event::structs::event::Event;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventsResponse {
    pub events: Vec<Event>,
    #[cfg_attr(feature = "export", tsify(type = "CursorInEvent"))]
    pub cursor: Cursor,
    pub cursor_string: String,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InEvent")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub event_id: u64,
}
