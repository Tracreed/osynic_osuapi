use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Status {
    #[serde(rename = "ranked")]
    Ranked,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "qualified")]
    Qualified,
    #[serde(rename = "loved")]
    Loved,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "wip")]
    WIP,
    #[serde(rename = "graveyard")]
    Graveyard,
}
