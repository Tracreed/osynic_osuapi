use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InScore")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub count_100: u32,
    pub count_300: u32,
    pub count_50: u32,
    pub count_geki: Option<u32>,
    pub count_katu: u32,
    pub count_miss: u32,
}
