// {
// 			"acronym": "EZ",
// 			"settings": {}
// 		},

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistMod {
    pub acronym: String,
    #[cfg_attr(feature = "export", tsify(type = "any"))]
    pub settings: serde_json::Value,
}
