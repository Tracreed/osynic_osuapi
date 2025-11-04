use serde::{Deserialize, Serialize};

use crate::v2::model::chat::structs::channel::ChatChannel;
use crate::v2::model::chat::structs::message::{ChatMessage, Sender};
use crate::v2::model::chat::structs::user_silence::UserSilence;

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateNewPMResponse {
    pub channel: ChatChannel,
    pub message: ChatMessage,
    pub new_channel_id: Option<u32>,
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUpdatesResponse {
    pub presence: Option<Vec<ChatMessage>>,
    // `messages` field is not used and will be removed.
    pub messages: (),
    pub silences: Option<Vec<UserSilence>>,
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChannelResponse {
    pub channel: ChatChannel,
    pub users: Vec<Sender>,
}
