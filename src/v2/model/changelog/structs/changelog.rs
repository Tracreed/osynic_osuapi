// ChangelogListing

use serde::{Deserialize, Serialize};

use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::search::Search;
use crate::v2::model::changelog::structs::stream::Stream;

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangelogListing {
    pub streams: Vec<Stream>,
    pub builds: Vec<ChanglogBuild>,
    #[cfg_attr(feature = "wasm", tsify(type = "SearchInChangelog"))]
    pub search: Search,
}
