use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentNomination {
    beatmapset_id: u32,
    rulesets: Vec<String>,
    reset: bool,
    user_id: u32,
}
