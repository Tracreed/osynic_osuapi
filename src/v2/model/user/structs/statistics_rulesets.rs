// src/structs/statistics_rulesets.rs

use crate::v2::model::user::structs::statistics::Statistics;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticsRulesets {
    pub osu: Statistics,
    pub taiko: Statistics,
    pub fruits: Statistics,
    pub mania: Statistics,
}
