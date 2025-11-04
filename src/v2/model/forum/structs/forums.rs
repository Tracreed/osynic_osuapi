// Forum
// Field 	Type 	Notes
// id 	integer
// name 	string
// description 	string
// subforums 	Forum[]? 	Maximum 2 layers of subforums from the top-level Forum

use serde::{Deserialize, Serialize};

use super::forum::Forum;

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forums {
    pub forums: Vec<Forum>,
}
