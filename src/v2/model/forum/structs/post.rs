// Forum Post
// Field 	Type 	Description
// created_at 	Timestamp
// deleted_at 	Timestamp?
// edited_at 	Timestamp?
// edited_by_id 	integer?
// forum_id 	integer
// id 	integer
// topic_id 	integer
// user_id 	integer

// Following fields are optional.
// Field 	Type 	Description
// body.html 	string 	Post content in HTML format.
// body.raw 	string 	Post content in BBCode format.

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForumPost {
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub edited_at: Option<String>,
    pub edited_by_id: Option<u64>,
    pub forum_id: u64,
    pub id: u64,
    pub topic_id: u64,
    pub user_id: u64,
    #[cfg_attr(feature = "export", tsify(type = "BodyInForumPost"))]
    pub body: Option<Body>,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InForumPost")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub html: String,
    pub raw: String,
}
