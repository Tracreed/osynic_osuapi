// src/structs/user.rs

use crate::v2::model::user::enums::playstyle::Playstyle;
use crate::v2::model::user::enums::profile_order::ProfileOrder;
use crate::v2::model::user::structs::badge::Badge;
use crate::v2::model::user::structs::banner::Banner;
use crate::v2::model::user::structs::country::Country;
use crate::v2::model::user::structs::cover::Cover;
use crate::v2::model::user::structs::daily_challenge_user_stats::DailyChallengeUserStats;
use crate::v2::model::user::structs::group::Group;
use crate::v2::model::user::structs::kudosu::Kudosu;
use crate::v2::model::user::structs::monthly_playcounts::MonthlyPlaycounts;
use crate::v2::model::user::structs::rank_highest::RankHighest;
use crate::v2::model::user::structs::rank_history::RankHistory;
use crate::v2::model::user::structs::replays_watched_count::ReplaysWatchedCount;
use crate::v2::model::user::structs::statistics::Statistics;
use crate::v2::model::user::structs::statistics_rulesets::StatisticsRulesets;
use crate::v2::model::user::structs::team::Team;
use crate::v2::model::user::structs::user_account_history::UserAccountHistory;
use crate::v2::model::user::structs::user_achievements::UserAchievements;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
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
    pub cover_url: Option<String>,
    pub discord: Option<String>,
    pub has_supported: Option<bool>,
    pub interests: Option<String>,

    pub join_date: Option<String>,
    pub location: Option<String>,
    pub max_blocks: Option<u32>,
    pub max_friends: Option<u32>,
    pub occupation: Option<String>,
    pub playmode: Option<String>,
    pub playstyle: Option<Vec<Playstyle>>,
    pub post_count: Option<u32>,
    pub profile_hue: Option<u32>,
    pub profile_order: Option<Vec<ProfileOrder>>,
    pub title: Option<String>,
    pub title_url: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub country: Option<Country>,
    pub cover: Option<Cover>,
    pub is_restricted: Option<bool>,
    pub kudosu: Option<Kudosu>,
    pub account_history: Option<Vec<UserAccountHistory>>,
    pub active_tournament_banner: Option<Banner>,
    pub active_tournament_banners: Option<Vec<Banner>>,
    pub badges: Option<Vec<Badge>>,
    pub beatmap_playcounts_count: Option<u32>,
    pub comments_count: Option<u32>,
    pub daily_challenge_user_stats: Option<DailyChallengeUserStats>,
    pub favourite_beatmapset_count: Option<u32>,
    pub follower_count: Option<u32>,
    pub graveyard_beatmapset_count: Option<u32>,
    pub groups: Option<Vec<Group>>,
    pub guest_beatmapset_count: Option<u32>,
    pub loved_beatmapset_count: Option<u32>,
    pub mapping_follower_count: Option<u32>,
    pub monthly_playcounts: Option<Vec<MonthlyPlaycounts>>,
    pub nominated_beatmapset_count: Option<u32>,
    pub page: Option<Page>,
    pub pending_beatmapset_count: Option<u32>,
    pub previous_usernames: Option<Vec<String>>,
    pub rank_highest: Option<RankHighest>,
    pub ranked_beatmapset_count: Option<u32>,
    pub replays_watched_counts: Option<Vec<ReplaysWatchedCount>>,
    pub scores_best_count: Option<u32>,
    pub scores_first_count: Option<u32>,
    pub scores_pinned_count: Option<u32>,
    pub scores_recent_count: Option<u32>,
    pub session_verified: Option<bool>,
    pub statistics: Option<Statistics>,
    pub statistics_rulesets: Option<StatisticsRulesets>,
    pub support_level: Option<u32>,
    pub team: Option<Team>,
    pub user_achievements: Option<Vec<UserAchievements>>,
    pub rank_history: Option<RankHistory>,
    #[serde(rename = "rankHistory")] // 好没素质
    pub rank_istoriya: Option<RankHistory>,
    pub ranked_and_approved_beatmapset_count: Option<u32>,
    pub unranked_beatmapset_count: Option<u32>,
    // Following fields are commented cause they are not always present
    // pub is_admin: Option<bool>,
    // pub is_bng: Option<bool>,
    // pub is_full_bn: Option<bool>,
    // pub is_gmt: Option<bool>,
    // pub is_limited_bn: Option<bool>,
    // pub is_moderator: Option<bool>,
    // pub is_nat: Option<bool>,
    // pub is_silenced: Option<bool>,
    // pub blocks: Option<Vec<String>>,
    // pub follow_user_mapping: Option<Vec<u32>>,
    // pub friends: Option<Vec<Friend>>,
    // pub unread_pm_count: Option<u32>,
    // pub user_preferences: Option<UserPreferences>,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub html: String,
    pub raw: String,
}

// #[cfg_attr(feature = "export", derive(tsify::Tsify))]
// #[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Friend {
//     pub target_id: u32,
//     pub relation_type: String,
//     pub mutual: bool,
// }

// #[cfg_attr(feature = "export", derive(tsify::Tsify))]
// #[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct UserPreferences {
//     pub audio_autoplay: Option<bool>,
//     pub audio_muted: Option<bool>,
//     pub audio_volume: Option<f32>,
//     pub beatmapset_card_size: Option<String>,
//     pub beatmapset_download: Option<String>,
//     pub beatmapset_show_anime_cover: Option<bool>,
//     pub beatmapset_show_nsfw: Option<bool>,
//     pub beatmapset_title_show_original: Option<bool>,
//     pub comments_show_deleted: Option<bool>,
//     pub forum_posts_show_deleted: Option<bool>,
//     pub legacy_score_only: Option<bool>,
//     pub profile_cover_expanded: Option<bool>,
//     pub scoring_mode: Option<String>,
//     pub user_list_filter: Option<String>,
//     pub user_list_sort: Option<String>,
//     pub user_list_view: Option<String>,
// }
