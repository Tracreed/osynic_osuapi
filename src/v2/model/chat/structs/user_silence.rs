use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSilence {
    pub id: u32,
    pub user_id: u32,
}
