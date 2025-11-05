use crate::v2::model::user::structs::country::Country;
use crate::v2::model::user::structs::cover::Cover;
use crate::v2::model::user::structs::group::Group;
use crate::v2::model::user::structs::statistics::Statistics;
use crate::v2::model::user::structs::team::Team;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Friend {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: Option<String>,
    pub id: u32,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<String>,

    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,

    pub country: Option<Country>,
    pub cover: Option<Cover>,
    pub groups: Option<Vec<Group>>,
    pub statistics: Option<Statistics>,
    pub support_level: Option<u32>,
    pub team: Option<Team>,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendXApiVersion {
    pub target_id: u32,
    pub relation_type: String,
    pub mutual: bool,
    pub target: Friend,
}
