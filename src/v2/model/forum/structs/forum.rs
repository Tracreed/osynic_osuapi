// Forum
// Field 	Type 	Notes
// id 	integer
// name 	string
// description 	string
// subforums 	Forum[]? 	Maximum 2 layers of subforums from the top-level Forum

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forum {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub subforums: Option<Vec<Forum>>,
}
