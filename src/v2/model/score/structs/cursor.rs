use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InScore")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub id: u64,
}
