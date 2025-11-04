// src/enums/profile_order.rs

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum ProfileOrder {
    #[serde(rename = "me")]
    Me,
    #[serde(rename = "recent_activity")]
    RecentActivity,
    #[serde(rename = "top_ranks")]
    TopRanks,
    #[serde(rename = "medals")]
    Medals,
    #[serde(rename = "historical")]
    Historical,
    #[serde(rename = "beatmaps")]
    Beatmaps,
    #[serde(rename = "kudosu")]
    Kudosu,
}
