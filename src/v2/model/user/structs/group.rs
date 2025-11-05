use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    colour: String,
    has_listing: bool,
    has_playmodes: bool,
    id: u32,
    identifier: String,
    is_probationary: bool,
    name: String,
    short_name: String,
    playmodes: Option<Vec<String>>,
}
