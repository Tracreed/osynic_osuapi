use std::sync::{Arc, Mutex};

use super::api::beatmap::GlooBeatmap;
use super::api::multiplayer::GlooMultiplayer;
use super::api::replay::GlooReplay;
use super::api::scores::GlooScores;
use super::api::user::GlooUser;
use super::serde::{deserialize_arc_mutex_string, serialize_arc_mutex_string};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Serialize, Deserialize)]
pub struct OsynicOsuApiV1GlooClient {
    #[serde(skip)]
    pub beatmap: GlooBeatmap,
    #[serde(skip)]
    pub user: GlooUser,
    #[serde(skip)]
    pub multiplayer: GlooMultiplayer,
    #[serde(skip)]
    pub replay: GlooReplay,
    #[serde(skip)]
    pub score: GlooScores,
    #[serde(
        serialize_with = "serialize_arc_mutex_string",
        deserialize_with = "deserialize_arc_mutex_string"
    )]
    #[cfg_attr(feature = "export", tsify(type = "string"))]
    pub api_key: Arc<Mutex<String>>,
    #[serde(
        serialize_with = "serialize_arc_mutex_string",
        deserialize_with = "deserialize_arc_mutex_string"
    )]
    #[cfg_attr(feature = "export", tsify(type = "string"))]
    pub proxy_url: Arc<Mutex<String>>,
}

impl OsynicOsuApiV1GlooClient {
    pub fn new(api_key: String) -> Self {
        let api_key = Arc::new(Mutex::new(api_key));
        let proxy_url = Arc::new(Mutex::new(String::new()));
        OsynicOsuApiV1GlooClient {
            beatmap: GlooBeatmap {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            user: GlooUser {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            multiplayer: GlooMultiplayer {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            replay: GlooReplay {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            score: GlooScores {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            api_key,
            proxy_url,
        }
    }

    /// Set the proxy URL for all API calls
    pub fn set_proxy_url(&self, proxy_url: String) {
        let mut url = self.proxy_url.lock().unwrap();
        *url = proxy_url;
    }

    pub fn set_api_key(&self, api_key: String) {
        let mut token = self.api_key.lock().unwrap();
        *token = api_key;
    }
}

impl Default for OsynicOsuApiV1GlooClient {
    fn default() -> Self {
        let api_key = Arc::new(Mutex::new(String::new()));
        let proxy_url = Arc::new(Mutex::new(String::new()));
        OsynicOsuApiV1GlooClient {
            beatmap: GlooBeatmap {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            user: GlooUser {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            multiplayer: GlooMultiplayer {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            replay: GlooReplay {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            score: GlooScores {
                api_key: api_key.clone(),
                proxy_url: proxy_url.clone(),
            },
            api_key,
            proxy_url,
        }
    }
}
