use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Banner {
    pub id: u32,
    pub tournament_id: u32,
    pub image: Option<String>,
    #[serde(rename = "image@2x")]
    pub image_2x: Option<String>,
}
