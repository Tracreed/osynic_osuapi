use serde::{Deserialize, Serialize};

use super::user_silence::UserSilence;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Silences {
    pub silences: Vec<UserSilence>,
}
