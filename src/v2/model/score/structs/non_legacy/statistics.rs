use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InScoreNonLegacy")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub ok: Option<u32>,
    pub meh: Option<u32>,
    pub great: Option<u32>,
    pub miss: Option<u32>,
    pub ignore_hit: Option<u32>,
    pub ignore_miss: Option<u32>,
    pub slider_tail_hit: Option<u32>,
    pub large_tick_hit: Option<u32>,
    pub large_tick_miss: Option<u32>,
    pub small_tick_hit: Option<u32>,
    pub small_tick_miss: Option<u32>,
}
