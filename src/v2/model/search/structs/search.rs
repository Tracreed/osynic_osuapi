use crate::v2::model::search::enums::sort::Sort;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    pub sort: String,
}

impl From<Sort> for Search {
    fn from(sort: Sort) -> Search {
        Search {
            limit: None,
            sort: sort.to_beatmapset_search(),
        }
    }
}

impl Default for Search {
    fn default() -> Search {
        Search {
            limit: None,
            sort: "relevance_dsec".to_string(),
        }
    }
}
