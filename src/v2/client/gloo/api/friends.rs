use crate::error::Result;
use crate::v2::client::gloo::check::check_res;
use crate::v2::interface::friends::IFriends;
use crate::v2::model::friend::{Friend, FriendXApiVersion};
use crate::v2::model::oauth::structs::o_token::OToken;
use gloo_net::http::Request;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Clone, Default)]
pub struct GlooFriends {
    pub o_token: Arc<Mutex<OToken>>,
    pub proxy_url: Arc<Mutex<String>>,
}

impl IFriends for GlooFriends {
    async fn get_friends(&self) -> Result<Vec<Friend>> {
        console::log_1(&JsValue::from_str("GlooFriends get_friends"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/friends");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let friends: Vec<Friend> = response.json().await?;
        Ok(friends)
    }

    async fn get_friends_x_api_version(
        &self,
        x_api_version: Option<String>,
    ) -> Result<Vec<FriendXApiVersion>> {
        console::log_1(&JsValue::from_str("GlooFriends get_friends_x_api_version"));

        let access_token = {
            let token = self.o_token.lock().unwrap();
            token.access_token.clone()
        };

        let x_api_version = x_api_version.unwrap_or("20251105".to_string());

        let proxy_url = {
            let url = self.proxy_url.lock().unwrap();
            url.clone()
        };

        let url = format!("{proxy_url}https://osu.ppy.sh/api/v2/friends");

        let res = Request::get(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("x-api-version", &x_api_version)
            .header("Authorization", &format!("Bearer {access_token}"))
            .send()
            .await?;

        let response = check_res(res)?;
        let friends: Vec<FriendXApiVersion> = response.json().await?;
        Ok(friends)
    }
}
