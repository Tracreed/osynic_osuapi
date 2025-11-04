// src/structs/monthly_playcounts.rs

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonthlyPlaycounts {
    pub start_date: String,
    pub count: u32,
}
