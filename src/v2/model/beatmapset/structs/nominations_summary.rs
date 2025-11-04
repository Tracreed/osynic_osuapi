use serde::{Deserialize, Serialize};

use super::required_meta::RequiredMeta;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NominationsSummary {
    pub current: u32,
    pub eligible_main_rulesets: Vec<String>,
    pub required_meta: RequiredMeta,
}
