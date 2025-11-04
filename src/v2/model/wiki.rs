use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiPage {
    pub available_locales: Vec<String>,
    pub layout: String,
    pub locale: String,
    pub markdown: String,
    pub path: String,
    pub subtitle: Option<String>,
    pub tags: Vec<String>,
    pub title: String,
}
