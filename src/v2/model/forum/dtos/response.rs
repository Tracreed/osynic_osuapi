use serde::{Deserialize, Serialize};

use crate::v2::model::forum::structs::forum::Forum;
use crate::v2::model::forum::structs::post::ForumPost;
use crate::v2::model::forum::structs::topic::ForumTopic;
use crate::v2::model::search::structs::search::Search;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTopicResponse {
    pub topic: ForumTopic,
    pub post: ForumPost,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTopicAndPostsResponse {
    pub cursor_string: String,
    pub posts: Vec<ForumPost>,
    pub search: Search,
    pub topic: ForumTopic,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetForumAndTopicsResponse {
    pub forum: Forum,
    pub topics: Vec<ForumTopic>,
    pub pinned_topics: Vec<ForumTopic>,
}
