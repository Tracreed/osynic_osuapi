use super::api::beatmaps::GlooBeatmaps;
use super::api::beatmapsets::GlooBeatmapsets;
use super::api::changelog::GlooChangelog;
use super::api::chat::GlooChat;
use super::api::comments::GlooComments;
use super::api::events::GlooEvents;
use super::api::forum::GlooForum;
use super::api::friends::GlooFriends;
use super::api::matches::GlooMatches;
use super::api::multiplayer::GlooMultiplayer;
use super::api::news::GlooNews;
use super::api::notifications::GlooNotifications;
use super::api::oauth::GlooOauth;
use super::api::ranking::GlooRanking;
use super::api::scores::GlooScores;
use super::api::search::GlooSearch;
use super::api::users::GlooUsers;
use super::api::wiki::GlooWiki;
use crate::v2::model::oauth::structs::o_token::OToken;
use std::sync::{Arc, Mutex};

use super::serde::{
    deserialize_arc_mutex_o_token, deserialize_arc_mutex_string, serialize_arc_mutex_o_token,
    serialize_arc_mutex_string,
};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Serialize, Deserialize)]
pub struct OsynicOsuApiV2GlooClient {
    #[serde(skip)]
    pub oauth: GlooOauth,
    #[serde(skip)]
    pub beatmapsets: GlooBeatmapsets,
    #[serde(skip)]
    pub beatmaps: GlooBeatmaps,
    #[serde(skip)]
    pub changelog: GlooChangelog,
    #[serde(skip)]
    pub chat: GlooChat,
    #[serde(skip)]
    pub comments: GlooComments,
    #[serde(skip)]
    pub events: GlooEvents,
    #[serde(skip)]
    pub forum: GlooForum,
    #[serde(skip)]
    pub search: GlooSearch,
    #[serde(skip)]
    pub matches: GlooMatches,
    #[serde(skip)]
    pub multiplayer: GlooMultiplayer,
    #[serde(skip)]
    pub news: GlooNews,
    #[serde(skip)]
    pub notifications: GlooNotifications,
    #[serde(skip)]
    pub ranking: GlooRanking,
    #[serde(skip)]
    pub scores: GlooScores,
    #[serde(skip)]
    pub users: GlooUsers,
    #[serde(skip)]
    pub wiki: GlooWiki,
    #[serde(skip)]
    pub friends: GlooFriends,
    #[serde(
        serialize_with = "serialize_arc_mutex_o_token",
        deserialize_with = "deserialize_arc_mutex_o_token"
    )]
    #[cfg_attr(feature = "export", tsify(type = "OToken"))]
    pub o_token: Arc<Mutex<OToken>>,
    #[serde(
        serialize_with = "serialize_arc_mutex_string",
        deserialize_with = "deserialize_arc_mutex_string"
    )]
    #[cfg_attr(feature = "export", tsify(type = "string"))]
    pub proxy_url: Arc<Mutex<String>>,
}

impl OsynicOsuApiV2GlooClient {
    pub fn new(o_token: OToken) -> Self {
        let o_token = Arc::new(Mutex::new(o_token));
        let proxy_url = Arc::new(Mutex::new(String::new()));

        OsynicOsuApiV2GlooClient {
            oauth: GlooOauth {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            beatmapsets: GlooBeatmapsets {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            beatmaps: GlooBeatmaps {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            changelog: GlooChangelog {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            chat: GlooChat {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            comments: GlooComments {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            events: GlooEvents {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            forum: GlooForum {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            search: GlooSearch {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            matches: GlooMatches {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            multiplayer: GlooMultiplayer {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            news: GlooNews {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            notifications: GlooNotifications {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            ranking: GlooRanking {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            scores: GlooScores {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            users: GlooUsers {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            wiki: GlooWiki {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            friends: GlooFriends {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            o_token,
            proxy_url,
        }
    }

    /// Set the proxy URL for all API calls
    pub fn set_proxy_url(&self, proxy_url: String) {
        let mut url = self.proxy_url.lock().unwrap();
        *url = proxy_url;
    }

    pub fn set_o_token(&self, o_token: OToken) {
        let mut token = self.o_token.lock().unwrap();
        *token = o_token;
    }
}

impl Default for OsynicOsuApiV2GlooClient {
    fn default() -> Self {
        let o_token = Arc::new(Mutex::new(OToken::default()));
        let proxy_url = Arc::new(Mutex::new(String::new()));

        OsynicOsuApiV2GlooClient {
            oauth: GlooOauth {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            beatmapsets: GlooBeatmapsets {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            beatmaps: GlooBeatmaps {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            changelog: GlooChangelog {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            chat: GlooChat {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            comments: GlooComments {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            events: GlooEvents {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            forum: GlooForum {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            search: GlooSearch {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            matches: GlooMatches {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            multiplayer: GlooMultiplayer {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            news: GlooNews {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            notifications: GlooNotifications {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            ranking: GlooRanking {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            scores: GlooScores {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            users: GlooUsers {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            wiki: GlooWiki {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            friends: GlooFriends {
                o_token: o_token.clone(),
                proxy_url: proxy_url.clone(),
            },
            o_token,
            proxy_url,
        }
    }
}
