use wasm_bindgen::prelude::*;

// Re-export main types for WASM
pub use crate::v1::interface::{
    beatmap::IBeatmap, multiplayer::IMultiplayer as IMultiplayerV1, replay::IReplay,
    scores::IScores as IScoresV1, user::IUser,
};

#[cfg(feature = "v2")]
pub use crate::v2::interface::{
    beatmaps::IBeatmaps, beatmapsets::IBeatmapsets, changelog::IChangelog, chat::IChat,
    comments::IComments, events::IEvents, forum::IForum, matches::IMatches,
    multiplayer::IMultiplayer, news::INews, notifications::INotifications, oauth::IOauth,
    ranking::IRanking, scores::IScores, search::ISearch, users::IUsers, wiki::IWiki, friends::IFriends,
};

pub use crate::prelude::OsynicOsuApiV1GlooClient as InnerOsynicOsuApiV1GlooClient;
pub use crate::prelude::OsynicOsuApiV2GlooClient as InnerOsynicOsuApiV2GlooClient;

// For V1 API
use crate::v1::model::beatmap::{Beatmap as BeatmapV1, GetBeatmapsParams};
use crate::v1::model::best::{BestScore, GetUserBestParams};
use crate::v1::model::multiplayer::{GetMatchParams, MultiplayerResponse};
use crate::v1::model::recent::{GetUserRecentParams, RecentPlay};
use crate::v1::model::replay::{GetReplayParams, Replay};
use crate::v1::model::scores::{GetScoresParams, Score as ScoreV1};
use crate::v1::model::user::{GetUserParams, User as UserV1};

// For V2 Oauth API
use crate::v2::model::oauth::enums::scope::Scope;
use crate::v2::model::oauth::structs::o_token::OToken;
// For V2 Beatmaps API
use crate::v2::model::beatmap::structs::beatmap::Beatmap;
use crate::v2::model::beatmap::structs::beatmaps::Beatmaps;
use crate::v2::model::beatmap::structs::difficulty_attributes::Attributes;
use crate::v2::model::beatmap::structs::pack::BeatmapPack;
use crate::v2::model::beatmap::structs::packs::BeatmapPacks;
use crate::v2::model::score::structs::beatmap_scores::BeatmapScores;
use crate::v2::model::score::structs::beatmap_user_score::BeatmapUserScore;
use crate::v2::model::score::structs::non_legacy_scores::NonLegacyScores;
use crate::v2::model::score::structs::scores::Scores;
// For V2 Beatmapsets API
use crate::v2::model::beatmapset::structs::beatmapset::Beatmapset;
use crate::v2::model::search::dtos::params::BeatmapsetsSearchParams;
use crate::v2::model::search::dtos::response::BeatmapsetsSearchResponse;
// For V2 Changelog API
use crate::v2::model::changelog::structs::build::ChanglogBuild;
use crate::v2::model::changelog::structs::changelog::ChangelogListing;
// For V2 Chat API
use crate::v2::model::chat::dtos::request::CreateChannelParams;
use crate::v2::model::chat::dtos::response::{
    CreateNewPMResponse, GetChannelResponse, GetUpdatesResponse,
};
use crate::v2::model::chat::structs::channel::ChatChannel;
use crate::v2::model::chat::structs::message::ChatMessage;
use crate::v2::model::chat::structs::silences::Silences;
// For V2 Comments API
use crate::v2::model::comment::structs::bundle::CommentBundle;
// For V2 Events API
use crate::v2::model::event::dtos::response::GetEventsResponse;
// For V2 Forum API
use crate::v2::model::forum::dtos::request::CreateTopicParams;
use crate::v2::model::forum::dtos::response::{
    CreateTopicResponse, GetForumAndTopicsResponse, GetTopicAndPostsResponse,
};
use crate::v2::model::forum::structs::forums::Forums;
use crate::v2::model::forum::structs::post::ForumPost;
use crate::v2::model::forum::structs::topic::{ForumTopic, TopicListing};
// For V2 Search API
use crate::v2::model::search::dtos::response::SearchResponse;
use crate::v2::model::search::enums::search_mode::SearchMode;
// For V2 Matches API
use crate::v2::model::matches::dtos::response::{GetMatchResponse, GetMatchesListingResponse};
// For V2 Multiplayer API
use crate::v2::model::room::structs::room::Room;
use crate::v2::model::score::structs::multiplayer::multiplayer_scores::MultiplayerScores;
// For V2 News API
use crate::v2::model::news::dtos::response::GetNewsListingResponse;
use crate::v2::model::news::structs::news::News;
// For V2 Notifications API
use crate::v2::model::notification::dtos::request::MarkNotificationsRequest;
use crate::v2::model::notification::dtos::response::GetNotificationsResponse;
// For V2 Ranking API
use crate::v2::model::ranking::enums::ranking_type::RankingType;
use crate::v2::model::ranking::structs::rankings::{KudosuRankings, Rankings};
use crate::v2::model::ranking::structs::spotlights::Spotlights;
// For V2 Scores API
use crate::v2::model::score::dtos::response::GetScoresResponse;
// For V2 Users API
use crate::v2::model::beatmap::structs::beatmap_playcount::BeatmapPlaycount;
use crate::v2::model::event::structs::event::Event;
use crate::v2::model::mode::enums::mode::Mode;
use crate::v2::model::score::enums::score_type::ScoreType;
use crate::v2::model::score::structs::score::Score;
use crate::v2::model::user::structs::kudosu_history::KudosuHistory;
use crate::v2::model::user::structs::user::User;
use crate::v2::model::user::structs::users::Users;
// For V2 Wiki API
use crate::v2::model::wiki::WikiPage;
// For V2 Friends API
use crate::v2::model::friend::{Friend, FriendXApiVersion};

// Configure console error panic hook for better debugging
#[cfg(feature = "export")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

//////////////////////////////////////////////////////////////////////////
/// OsynicOsuApiV1GlooClient
/// - get_beatmaps
/// - get_user
/// - get_user_best
/// - get_user_recent
/// - get_match
/// - get_scores
/// - get_replay
//////////////////////////////////////////////////////////////////////////

/// WASM wrapper for OsynicOsuApiV1GlooClient
#[wasm_bindgen]
pub struct OsynicOsuApiV1GlooClient {
    inner: InnerOsynicOsuApiV1GlooClient,
}

#[wasm_bindgen]
impl OsynicOsuApiV1GlooClient {
    /// Create a new OsynicOsuApiV1GlooClient
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Self {
        let inner = InnerOsynicOsuApiV1GlooClient::new(api_key);
        OsynicOsuApiV1GlooClient { inner }
    }

    /// Set the proxy URL for all API calls
    #[wasm_bindgen(js_name = setProxyUrl)]
    pub fn set_proxy_url(&self, proxy_url: String) {
        self.inner.set_proxy_url(proxy_url);
    }

    /// Get Beatmaps API
    #[wasm_bindgen(js_name = getBeatmaps)]
    pub async fn get_beatmaps(&self, params: GetBeatmapsParams) -> Result<Vec<BeatmapV1>, JsValue> {
        match self.inner.beatmap.get_beatmaps(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User API
    #[wasm_bindgen(js_name = getUser)]
    pub async fn get_user(&self, params: GetUserParams) -> Result<Vec<UserV1>, JsValue> {
        match self.inner.user.get_user(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Best API
    #[wasm_bindgen(js_name = getUserBest)]
    pub async fn get_user_best(
        &self,
        params: GetUserBestParams,
    ) -> Result<Vec<BestScore>, JsValue> {
        match self.inner.user.get_user_best(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Recent API
    #[wasm_bindgen(js_name = getUserRecent)]
    pub async fn get_user_recent(
        &self,
        params: GetUserRecentParams,
    ) -> Result<Vec<RecentPlay>, JsValue> {
        match self.inner.user.get_user_recent(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Match API
    #[wasm_bindgen(js_name = getMatch)]
    pub async fn get_match(&self, params: GetMatchParams) -> Result<MultiplayerResponse, JsValue> {
        match self.inner.multiplayer.get_match(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Scores API
    #[wasm_bindgen(js_name = getScores)]
    pub async fn get_scores(&self, params: GetScoresParams) -> Result<Vec<ScoreV1>, JsValue> {
        match self.inner.score.get_scores(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Replay API
    #[wasm_bindgen(js_name = getReplay)]
    pub async fn get_replay(&self, params: GetReplayParams) -> Result<Replay, JsValue> {
        match self.inner.replay.get_replay(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

//////////////////////////////////////////////////////////////////////////
/// OsynicOsuApiV2GlooClient
//////////////////////////////////////////////////////////////////////////

/// WASM wrapper for OsynicOsuApiV2GlooClient
#[wasm_bindgen]
pub struct OsynicOsuApiV2GlooClient {
    inner: InnerOsynicOsuApiV2GlooClient,
}

#[wasm_bindgen]
impl OsynicOsuApiV2GlooClient {
    /// Create a new OsynicOsuApiV2GlooClient
    #[wasm_bindgen(constructor)]
    pub fn new(o_token: OToken) -> Self {
        let inner = InnerOsynicOsuApiV2GlooClient::new(o_token);
        OsynicOsuApiV2GlooClient { inner }
    }

    /// Set the proxy URL for all API calls
    #[wasm_bindgen(js_name = setProxyUrl)]
    pub fn set_proxy_url(&self, proxy_url: String) {
        self.inner.set_proxy_url(proxy_url);
    }

    //////////////////////////////////////////////////////////////////////////
    /// Oauth API
    /// - get_token_with_code
    /// - get_token_without_code
    /// - refresh_token
    /// - revoke_current_token
    //////////////////////////////////////////////////////////////////////////

    /// Get Token With Code API
    #[wasm_bindgen(js_name = getTokenWithCode)]
    pub async fn get_token_with_code(
        &self,
        client_id: u64,
        client_secret: String,
        code: String,
        redirect_uri: String,
    ) -> Result<OToken, JsValue> {
        match self
            .inner
            .oauth
            .get_token_with_code(client_id, &client_secret, &code, &redirect_uri)
            .await
        {
            Ok(token) => Ok(token),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Token Without Code API
    #[wasm_bindgen(js_name = getTokenWithoutCode)]
    pub async fn get_token_without_code(
        &self,
        client_id: u64,
        client_secret: String,
    ) -> Result<OToken, JsValue> {
        match self
            .inner
            .oauth
            .get_token_without_code(client_id, &client_secret)
            .await
        {
            Ok(token) => Ok(token),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Refresh Token API
    #[wasm_bindgen(js_name = refreshToken)]
    pub async fn refresh_token(
        &self,
        client_id: u64,
        client_secret: String,
        scope: Option<Vec<Scope>>,
    ) -> Result<OToken, JsValue> {
        match self
            .inner
            .oauth
            .refresh_token(client_id, &client_secret, scope)
            .await
        {
            Ok(token) => Ok(token),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Revoke Current Token API
    #[wasm_bindgen(js_name = revokeCurrentToken)]
    pub async fn revoke_current_token(&self) -> Result<(), JsValue> {
        match self.inner.oauth.revoke_current_token().await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Beatmaps API
    /// - get_beatmap_packs
    /// - get_beatmap_pack
    /// - lookup_beatmap
    /// - get_beatmap
    /// - get_beatmap_attributes
    /// - get_beatmaps
    /// - get_scores_in_beatmap
    /// - get_solo_scores
    /// - get_user_score
    /// - get_user_scoress_in_beatmap
    //////////////////////////////////////////////////////////////////////////

    /// Get Beatmap Packs API
    #[wasm_bindgen(js_name = getBeatmapPacks)]
    pub async fn get_beatmap_packs(
        &self,
        pack_type: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<BeatmapPacks, JsValue> {
        match self
            .inner
            .beatmaps
            .get_beatmap_packs(pack_type, cursor_string)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Beatmap Pack API
    #[wasm_bindgen(js_name = getBeatmapPack)]
    pub async fn get_beatmap_pack(
        &self,
        pack: String,
        legacy_only: Option<u32>,
    ) -> Result<BeatmapPack, JsValue> {
        match self
            .inner
            .beatmaps
            .get_beatmap_pack(pack, legacy_only)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Lookup Beatmap API
    #[wasm_bindgen(js_name = lookupBeatmap)]
    pub async fn lookup_beatmap(
        &self,
        checksum: Option<String>,
        filename: Option<String>,
        id: Option<String>,
    ) -> Result<Beatmap, JsValue> {
        match self.inner.beatmaps.lookup(checksum, filename, id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Score API
    #[wasm_bindgen(js_name = getUserScore)]
    pub async fn get_user_score(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
    ) -> Result<BeatmapUserScore, JsValue> {
        match self
            .inner
            .beatmaps
            .get_user_score(beatmap_id, user_id, legacy_only, mode, mods)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Scores API
    #[wasm_bindgen(js_name = getUserScoresInBeatmap)]
    pub async fn get_user_scores_in_beatmap(
        &self,
        beatmap_id: u32,
        user_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        ruleset: Option<Mode>,
    ) -> Result<Scores, JsValue> {
        match self
            .inner
            .beatmaps
            .get_user_scores(beatmap_id, user_id, legacy_only, mode, ruleset)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Scores In Beatmap API
    #[wasm_bindgen(js_name = getScoresInBeatmap)]
    pub async fn get_scores_in_beatmap(
        &self,
        beatmap_id: u32,
        legacy_only: Option<u32>,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> Result<BeatmapScores, JsValue> {
        match self
            .inner
            .beatmaps
            .get_scores(beatmap_id, legacy_only, mode, mods, ranking_type)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Solo Scores API
    #[wasm_bindgen(js_name = getSoloScores)]
    pub async fn get_solo_scores(
        &self,
        beatmap_id: u32,
        mode: Option<Mode>,
        mods: Option<String>,
        ranking_type: Option<String>,
    ) -> Result<NonLegacyScores, JsValue> {
        match self
            .inner
            .beatmaps
            .get_solo_scores(beatmap_id, mode, mods, ranking_type)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Beatmaps API
    #[wasm_bindgen(js_name = getBeatmaps)]
    pub async fn get_beatmaps(&self, beatmap_ids: Vec<u32>) -> Result<Beatmaps, JsValue> {
        match self.inner.beatmaps.get_beatmaps(beatmap_ids).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Beatmap API
    #[wasm_bindgen(js_name = getBeatmap)]
    pub async fn get_beatmap(&self, beatmap_id: u32) -> Result<Beatmap, JsValue> {
        match self.inner.beatmaps.get_beatmap(beatmap_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Beatmap Attributes API
    #[wasm_bindgen(js_name = getBeatmapAttributes)]
    pub async fn get_beatmap_attributes(
        &self,
        beatmap_id: u32,
        mods: Option<Vec<String>>,
        ruleset: Option<Mode>,
        ruleset_id: Option<i32>,
    ) -> Result<Attributes, JsValue> {
        match self
            .inner
            .beatmaps
            .get_beatmap_attributes(beatmap_id, mods, ruleset, ruleset_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Beatmapsets API
    /// - search_beatmapsets
    /// - get_beatmapset
    /// - download_beatmapset
    //////////////////////////////////////////////////////////////////////////

    /// Search Beatmapsets API
    #[wasm_bindgen(js_name = searchBeatmapsets)]
    pub async fn search_beatmapsets(
        &self,
        params: BeatmapsetsSearchParams,
    ) -> Result<BeatmapsetsSearchResponse, JsValue> {
        match self.inner.beatmapsets.search(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Beatmapset API
    #[wasm_bindgen(js_name = getBeatmapset)]
    pub async fn get_beatmapset(&self, beatmapset_id: u32) -> Result<Beatmapset, JsValue> {
        match self.inner.beatmapsets.get_beatmapset(beatmapset_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Download Beatmapset API
    #[wasm_bindgen(js_name = downloadBeatmapset)]
    pub async fn download_beatmapset(&self, beatmapset_id: u32) -> Result<(), JsValue> {
        match self.inner.beatmapsets.download(beatmapset_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Changelog API
    /// - get_changelog_build
    /// - get_changelog_listing
    /// - lookup_changelog_build
    //////////////////////////////////////////////////////////////////////////

    /// Get Changelog Build API
    #[wasm_bindgen(js_name = getChangelogBuild)]
    pub async fn get_changelog_build(
        &self,
        stream: String,
        build: String,
    ) -> Result<ChanglogBuild, JsValue> {
        match self
            .inner
            .changelog
            .get_changelog_build(stream, build)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Changelog Listing API
    #[wasm_bindgen(js_name = getChangelogListing)]
    pub async fn get_changelog_listing(
        &self,
        from: Option<String>,
        max_id: Option<u32>,
        stream: Option<String>,
        to: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> Result<ChangelogListing, JsValue> {
        match self
            .inner
            .changelog
            .get_changelog_listing(from, max_id, stream, to, message_formats)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Lookup Changelog Build API
    #[wasm_bindgen(js_name = lookupChangelogBuild)]
    pub async fn lookup_changelog_build(
        &self,
        changelog: String,
        key: Option<String>,
        message_formats: Option<Vec<String>>,
    ) -> Result<ChanglogBuild, JsValue> {
        match self
            .inner
            .changelog
            .lookup_changelog_build(changelog, key, message_formats)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Chat API
    /// - chat_keepalive
    /// - create_new_pm
    /// - get_updates
    /// - get_channel_messages
    /// - send_message_to_channel
    /// - join_channel
    /// - leave_channel
    /// - mark_channel_as_read
    /// - get_channel_list
    /// - create_channel
    /// - get_channel
    //////////////////////////////////////////////////////////////////////////

    /// Chat Keepalive API
    #[wasm_bindgen(js_name = chatKeepalive)]
    pub async fn chat_keepalive(
        &self,
        history_since: Option<u64>,
        since: Option<u64>,
    ) -> Result<Silences, JsValue> {
        match self.inner.chat.chat_keepalive(history_since, since).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Create New PM API
    #[wasm_bindgen(js_name = createNewPM)]
    pub async fn create_new_pm(
        &self,
        target_id: u64,
        message: String,
        is_action: bool,
        uuid: Option<String>,
    ) -> Result<CreateNewPMResponse, JsValue> {
        match self
            .inner
            .chat
            .create_new_pm(target_id, message, is_action, uuid)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }
    /// Get Updates API
    #[wasm_bindgen(js_name = getUpdates)]
    pub async fn get_updates(
        &self,
        history_since: Option<u64>,
        includes: Option<Vec<String>>,
        since: Option<u64>,
    ) -> Result<GetUpdatesResponse, JsValue> {
        match self
            .inner
            .chat
            .get_updates(history_since, includes, since)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Channel Messages API
    #[wasm_bindgen(js_name = getChannelMessages)]
    pub async fn get_channel_messages(
        &self,
        channel: String,
        limit: Option<u64>,
        since: Option<u64>,
        until: Option<u64>,
    ) -> Result<Vec<ChatMessage>, JsValue> {
        match self
            .inner
            .chat
            .get_channel_messages(channel, limit, since, until)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Send Message To Channel API
    #[wasm_bindgen(js_name = sendMessageToChannel)]
    pub async fn send_message_to_channel(
        &self,
        channel: u64,
        message: String,
        is_action: bool,
    ) -> Result<ChatMessage, JsValue> {
        match self
            .inner
            .chat
            .send_message_to_channel(channel, message, is_action)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Join Channel API
    #[wasm_bindgen(js_name = joinChannel)]
    pub async fn join_channel(
        &self,
        channel: String,
        user: String,
    ) -> Result<ChatChannel, JsValue> {
        match self.inner.chat.join_channel(channel, user).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Leave Channel API
    #[wasm_bindgen(js_name = leaveChannel)]
    pub async fn leave_channel(&self, channel: String, user: String) -> Result<(), JsValue> {
        match self.inner.chat.leave_channel(channel, user).await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Mark Channel As Read API
    #[wasm_bindgen(js_name = markChannelAsRead)]
    pub async fn mark_channel_as_read(
        &self,
        channel: String,
        message: String,
        channel_id: String,
        message_id: String,
    ) -> Result<(), JsValue> {
        match self
            .inner
            .chat
            .mark_channel_as_read(channel, message, channel_id, message_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Channel List API
    #[wasm_bindgen(js_name = getChannelList)]
    pub async fn get_channel_list(&self) -> Result<Vec<ChatChannel>, JsValue> {
        match self.inner.chat.get_channel_list().await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Create Channel API
    #[wasm_bindgen(js_name = createChannel)]
    pub async fn create_channel(
        &self,
        params: CreateChannelParams,
    ) -> Result<ChatChannel, JsValue> {
        match self.inner.chat.create_channel(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Channel API
    #[wasm_bindgen(js_name = getChannel)]
    pub async fn get_channel(&self, channel: String) -> Result<GetChannelResponse, JsValue> {
        match self.inner.chat.get_channel(channel).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Comments API
    /// - get_comments
    /// - post_comment
    /// - get_comment
    /// - edit_comment
    /// - delete_comment
    /// - add_comment_vote
    /// - remove_comment_vote
    //////////////////////////////////////////////////////////////////////////

    /// Get Comments API
    #[wasm_bindgen(js_name = getComments)]
    pub async fn get_comments(
        &self,
        after: Option<String>,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        cursor: Option<String>,
        parent_id: Option<String>,
        sort: Option<String>,
    ) -> Result<CommentBundle, JsValue> {
        match self
            .inner
            .comments
            .get_comments(
                after,
                commentable_type,
                commentable_id,
                cursor,
                parent_id,
                sort,
            )
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Post Comment API
    #[wasm_bindgen(js_name = postComment)]
    pub async fn post_comment(
        &self,
        commentable_type: Option<String>,
        commentable_id: Option<String>,
        parent_id: Option<String>,
        message: Option<String>,
    ) -> Result<CommentBundle, JsValue> {
        match self
            .inner
            .comments
            .post_comment(commentable_type, commentable_id, parent_id, message)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Comment API
    #[wasm_bindgen(js_name = getComment)]
    pub async fn get_comment(&self, comment: String) -> Result<CommentBundle, JsValue> {
        match self.inner.comments.get_comment(comment).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Edit Comment API
    #[wasm_bindgen(js_name = editComment)]
    pub async fn edit_comment(
        &self,
        comment: String,
        message: Option<String>,
    ) -> Result<CommentBundle, JsValue> {
        match self.inner.comments.edit_comment(comment, message).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Delete Comment API
    #[wasm_bindgen(js_name = deleteComment)]
    pub async fn delete_comment(&self, comment: String) -> Result<CommentBundle, JsValue> {
        match self.inner.comments.delete_comment(comment).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Add Comment Vote API
    #[wasm_bindgen(js_name = addCommentVote)]
    pub async fn add_comment_vote(&self, comment: String) -> Result<CommentBundle, JsValue> {
        match self.inner.comments.add_comment_vote(comment).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Remove Comment Vote API
    #[wasm_bindgen(js_name = removeCommentVote)]
    pub async fn remove_comment_vote(&self, comment: String) -> Result<CommentBundle, JsValue> {
        match self.inner.comments.remove_comment_vote(comment).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Events API
    /// - get_events
    //////////////////////////////////////////////////////////////////////////

    /// Get Events API
    #[wasm_bindgen(js_name = getEvents)]
    pub async fn get_events(
        &self,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetEventsResponse, JsValue> {
        match self.inner.events.get_events(sort, cursor_string).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Forum API
    /// - reply_topic
    /// - get_topics_listing
    /// - create_topic
    /// - get_topic_and_posts
    /// - edit_topic
    /// - edit_post
    /// - get_forum_listing
    /// - get_forum_and_topic
    //////////////////////////////////////////////////////////////////////////

    /// Reply Topic API
    #[wasm_bindgen(js_name = replyTopic)]
    pub async fn reply_topic(&self, topic: String, body: String) -> Result<ForumPost, JsValue> {
        match self.inner.forum.reply_topic(topic, body).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Topics Listing API
    #[wasm_bindgen(js_name = getTopicsListing)]
    pub async fn get_topics_listing(
        &self,
        forum_id: Option<String>,
        sort: Option<String>,
        limit: Option<u32>,
        cursor_string: Option<String>,
    ) -> Result<TopicListing, JsValue> {
        match self
            .inner
            .forum
            .get_topics_listing(forum_id, sort, limit, cursor_string)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Create Topic API
    #[wasm_bindgen(js_name = createTopic)]
    pub async fn create_topic(
        &self,
        params: CreateTopicParams,
    ) -> Result<CreateTopicResponse, JsValue> {
        match self.inner.forum.create_topic(params).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Topic And Posts API
    #[wasm_bindgen(js_name = getTopicAndPosts)]
    pub async fn get_topic_and_posts(
        &self,
        topic: u32,
        sort: Option<String>,
        limit: Option<u32>,
        start: Option<String>,
        end: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetTopicAndPostsResponse, JsValue> {
        match self
            .inner
            .forum
            .get_topic_and_posts(topic, sort, limit, start, end, cursor_string)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Edit Topic API
    #[wasm_bindgen(js_name = editTopic)]
    pub async fn edit_topic(
        &self,
        topic: String,
        topic_title: String,
    ) -> Result<ForumTopic, JsValue> {
        match self.inner.forum.edit_topic(topic, topic_title).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Edit Post API
    #[wasm_bindgen(js_name = editPost)]
    pub async fn edit_post(&self, post: String, body: String) -> Result<ForumPost, JsValue> {
        match self.inner.forum.edit_post(post, body).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Forum Listing API
    #[wasm_bindgen(js_name = getForumListing)]
    pub async fn get_forum_listing(&self) -> Result<Forums, JsValue> {
        match self.inner.forum.get_forum_listing().await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Forum And Topic API
    #[wasm_bindgen(js_name = getForumAndTopic)]
    pub async fn get_forum_and_topic(
        &self,
        forum: u64,
    ) -> Result<GetForumAndTopicsResponse, JsValue> {
        match self.inner.forum.get_forum_and_topic(forum).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Home API
    /// - search
    //////////////////////////////////////////////////////////////////////////

    /// Search API
    #[wasm_bindgen(js_name = search)]
    pub async fn search(
        &self,
        mode: Option<SearchMode>,
        query: Option<String>,
        page: Option<u32>,
    ) -> Result<SearchResponse, JsValue> {
        match self.inner.search.search(mode, query, page).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Matches API
    /// - get_matches_listing
    /// - get_match
    //////////////////////////////////////////////////////////////////////////

    /// Get Matches Listing API
    #[wasm_bindgen(js_name = getMatchesListing)]
    pub async fn get_matches_listing(
        &self,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<GetMatchesListingResponse, JsValue> {
        match self
            .inner
            .matches
            .get_matches_listing(limit, sort, cursor_string)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Match API
    #[wasm_bindgen(js_name = getMatch)]
    pub async fn get_match(
        &self,
        match_id: u64,
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    ) -> Result<GetMatchResponse, JsValue> {
        match self
            .inner
            .matches
            .get_match(match_id, before, after, limit)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Multiplayer API
    /// - get_user_high_score
    /// - get_scores_in_multiplayer
    /// - get_score_in_multiplayer
    /// - get_multiplayer_rooms
    //////////////////////////////////////////////////////////////////////////

    /// Get User High Score API
    #[wasm_bindgen(js_name = getUserHighScore)]
    pub async fn get_user_high_score(
        &self,
        room: String,
        playlist: u64,
        user: u64,
    ) -> Result<Score, JsValue> {
        match self
            .inner
            .multiplayer
            .get_user_high_score(room, playlist, user)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Scores In Multiplayer API
    #[wasm_bindgen(js_name = getScoresInMultiplayer)]
    pub async fn get_scores_in_multiplayer(
        &self,
        room: String,
        playlist: u64,
        limit: Option<u32>,
        sort: Option<String>,
        cursor_string: Option<String>,
    ) -> Result<MultiplayerScores, JsValue> {
        match self
            .inner
            .multiplayer
            .get_scores(room, playlist, limit, sort, cursor_string)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Score In Multiplayer API
    #[wasm_bindgen(js_name = getScoreInMultiplayer)]
    pub async fn get_score_in_multiplayer(
        &self,
        room: String,
        playlist: u64,
        score: u64,
    ) -> Result<Score, JsValue> {
        match self
            .inner
            .multiplayer
            .get_score(room, playlist, score)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Multiplayer Rooms API
    #[wasm_bindgen(js_name = getMultiplayerRooms)]
    pub async fn get_multiplayer_rooms(
        &self,
        limit: Option<u32>,
        mode: Option<String>,
        season_id: Option<u32>,
        sort: Option<String>,
        type_group: Option<String>,
    ) -> Result<Vec<Room>, JsValue> {
        match self
            .inner
            .multiplayer
            .get_multiplayer_rooms(limit, mode, season_id, sort, type_group)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// News API
    /// - get_news_listing
    /// - get_news_post
    //////////////////////////////////////////////////////////////////////////

    /// Get News Listing API
    #[wasm_bindgen(js_name = getNewsListing)]
    pub async fn get_news_listing(
        &self,
        limit: Option<u32>,
        year: Option<u32>,
        cursor_string: Option<String>,
    ) -> Result<GetNewsListingResponse, JsValue> {
        match self
            .inner
            .news
            .get_news_listing(limit, year, cursor_string)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get News Post API
    #[wasm_bindgen(js_name = getNewsPost)]
    pub async fn get_news_post(&self, news: String, key: Option<String>) -> Result<News, JsValue> {
        match self.inner.news.get_news_post(news, key).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Notifications API
    /// - get_notifications
    /// - mark_notification_as_read
    //////////////////////////////////////////////////////////////////////////

    // Get Notifications API
    #[wasm_bindgen(js_name = getNotifications)]
    pub async fn get_notifications(
        &self,
        max_id: Option<String>,
    ) -> Result<GetNotificationsResponse, JsValue> {
        match self.inner.notifications.get_notifications(max_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Mark Notifications As Read API
    #[wasm_bindgen(js_name = markNotificationsAsRead)]
    pub async fn mark_notifications_as_read(
        &self,
        params: Option<MarkNotificationsRequest>,
    ) -> Result<(), JsValue> {
        match self
            .inner
            .notifications
            .mark_notifications_as_read(params)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Ranking API
    /// - get_ranking
    /// - get_kudosu_ranking
    /// - get_spotlights
    //////////////////////////////////////////////////////////////////////////

    /// Get Kudosu Ranking API
    #[wasm_bindgen(js_name = getKudosuRanking)]
    pub async fn get_kudosu_ranking(&self, page: Option<u32>) -> Result<KudosuRankings, JsValue> {
        match self.inner.ranking.get_kudosu_ranking(page).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Ranking API
    #[wasm_bindgen(js_name = getRanking)]
    pub async fn get_ranking(
        &self,
        mode: Mode,
        ranking_type: RankingType,
        country: Option<String>,
        cursor_string: Option<String>,
        filter: Option<String>,
        spotlight: Option<String>,
        variant: Option<String>,
    ) -> Result<Rankings, JsValue> {
        match self
            .inner
            .ranking
            .get_ranking(
                mode,
                ranking_type,
                country,
                cursor_string,
                filter,
                spotlight,
                variant,
            )
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Spotlights API
    #[wasm_bindgen(js_name = getSpotlights)]
    pub async fn get_spotlights(&self) -> Result<Spotlights, JsValue> {
        match self.inner.ranking.get_spotlights().await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Scores API
    /// - get_scores
    //////////////////////////////////////////////////////////////////////////

    /// Get Scores API
    #[wasm_bindgen(js_name = getScores)]
    pub async fn get_scores(
        &self,
        ruleset: Option<Mode>,
        cursor_string: Option<String>,
    ) -> Result<GetScoresResponse, JsValue> {
        match self.inner.scores.get_scores(ruleset, cursor_string).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Users API
    /// - get_own_data
    /// - get_user_kudosu
    /// - get_user_beatmaps
    /// - get_user_beatmaps_most_played
    /// - get_user_recent_activity
    /// - get_user
    /// - get_user_by_username
    /// - get_users
    //////////////////////////////////////////////////////////////////////////

    /// Get Own Data API
    #[wasm_bindgen(js_name = getOwnData)]
    pub async fn get_own_data(
        &self,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> Result<User, JsValue> {
        match self.inner.users.get_own_data(mode, key).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Kudosu API
    #[wasm_bindgen(js_name = getUserKudosu)]
    pub async fn get_user_kudosu(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<KudosuHistory>, JsValue> {
        match self.inner.users.get_user_kudosu(id, limit, offset).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Scores API
    #[wasm_bindgen(js_name = getUserScores)]
    pub async fn get_user_scores(
        &self,
        id: u32,
        score_type: ScoreType,
        legacy_only: Option<u32>,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Score>, JsValue> {
        match self
            .inner
            .users
            .get_user_scores(id, score_type, legacy_only, limit, offset)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Beatmaps API
    #[wasm_bindgen(js_name = getUserBeatmaps)]
    pub async fn get_user_beatmaps(
        &self,
        id: u32,
        beatmap_type: String,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Beatmapset>, JsValue> {
        match self
            .inner
            .users
            .get_user_beatmaps(id, beatmap_type, limit, offset)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Beatmaps Most Played API
    #[wasm_bindgen(js_name = getUserBeatmapsMostPlayed)]
    pub async fn get_user_beatmaps_most_played(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<BeatmapPlaycount>, JsValue> {
        match self
            .inner
            .users
            .get_user_beatmaps_most_played(id, limit, offset)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User Recent Activity API
    #[wasm_bindgen(js_name = getUserRecentActivity)]
    pub async fn get_user_recent_activity(
        &self,
        id: u32,
        limit: Option<i32>,
        offset: Option<String>,
    ) -> Result<Vec<Event>, JsValue> {
        match self
            .inner
            .users
            .get_user_recent_activity(id, limit, offset)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User API
    #[wasm_bindgen(js_name = getUser)]
    pub async fn get_user(
        &self,
        id: u32,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> Result<User, JsValue> {
        match self.inner.users.get_user(id, mode, key).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get User By Username API
    #[wasm_bindgen(js_name = getUserByUsername)]
    pub async fn get_user_by_username(
        &self,
        username: String,
        mode: Option<Mode>,
        key: Option<String>,
    ) -> Result<User, JsValue> {
        match self
            .inner
            .users
            .get_user_by_username(&username, mode, key)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Users API
    #[wasm_bindgen(js_name = getUsers)]
    pub async fn get_users(
        &self,
        ids: Vec<u32>,
        include_variant_statistics: Option<bool>,
    ) -> Result<Users, JsValue> {
        match self
            .inner
            .users
            .get_users(ids, include_variant_statistics)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Wiki API
    /// - get_wiki_page
    //////////////////////////////////////////////////////////////////////////

    /// Get Wiki Page API
    #[wasm_bindgen(js_name = getWikiPage)]
    pub async fn get_wiki_page(&self, locale: String, path: String) -> Result<WikiPage, JsValue> {
        match self.inner.wiki.get_wiki_page(locale, path).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    /// Friends API
    /// - get_friends
    /// - get_friends_x_api_version
    //////////////////////////////////////////////////////////////////////////

    /// Get Friends API
    #[wasm_bindgen(js_name = getFriends)]
    pub async fn get_friends(&self) -> Result<Vec<Friend>, JsValue> {
        match self.inner.friends.get_friends().await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get Friends X API Version API
    #[wasm_bindgen(js_name = getFriendsXApiVersion)]
    pub async fn get_friends_x_api_version(&self, x_api_version: Option<String>) -> Result<Vec<FriendXApiVersion>, JsValue> {
        match self.inner.friends.get_friends_x_api_version(x_api_version).await {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("API error: {}", e))),
        }
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}
