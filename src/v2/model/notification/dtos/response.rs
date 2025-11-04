use serde::{Deserialize, Serialize};

use crate::v2::model::notification::structs::notifications::Notification;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNotificationsResponse {
    pub has_more: bool,
    pub notifications: Vec<Notification>,
    pub unread_count: u32,
    pub notification_endpoint: String,
}
