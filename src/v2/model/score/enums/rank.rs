use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "Type")
)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Rank {
    #[serde(rename = "XH")]
    XH,
    #[serde(rename = "X")]
    X,
    #[serde(rename = "SH")]
    SH,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "None")]
    None,
}
