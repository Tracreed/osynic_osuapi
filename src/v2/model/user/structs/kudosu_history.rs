use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KudosuHistory {
    pub id: u64,
    pub action: String,
    pub amount: i32,
    pub model: String,
    pub created_at: String,
    pub giver: Option<String>,
    pub post: Post,
    pub details: Details,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub url: String,
    pub title: String,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub event: String,
}
