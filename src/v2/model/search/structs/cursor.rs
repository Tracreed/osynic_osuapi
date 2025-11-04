use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InSearch")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub approved_date: u64,
    pub id: u64,
}
