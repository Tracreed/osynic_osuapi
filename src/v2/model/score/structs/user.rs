use serde::{Deserialize, Serialize};

use crate::v2::model::score::structs::team::Team;
use crate::v2::model::user::structs::country::Country;
use crate::v2::model::user::structs::cover::Cover;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InScore")
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
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
    #[cfg_attr(feature = "export", tsify(type = "Country | null"))]
    pub country: Option<Country>,
    #[cfg_attr(feature = "export", tsify(type = "Cover | null"))]
    pub cover: Option<Cover>,
    #[cfg_attr(feature = "export", tsify(type = "Team | null"))]
    pub team: Option<Team>,
}
