use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub id: u32,
    pub username: String,
}
