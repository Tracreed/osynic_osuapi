use serde::{Deserialize, Serialize};

// 基本事件字段
#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseEvent {
    pub created_at: String,
    #[serde(rename = "createdAt")]
    pub created_at_alt: String,
    pub id: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    #[cfg_attr(feature = "export", tsify(type = "UserInEvent"))]
    pub user: User,
}

// 主事件枚举
#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "Type")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Rank {
        #[serde(flatten)]
        #[cfg_attr(feature = "export", tsify(type = "BaseEvent"))]
        base: BaseEvent,
        #[serde(rename = "scoreRank")]
        score_rank: String,
        rank: u32,
        mode: String,
        #[cfg_attr(feature = "export", tsify(type = "BeatmapInEvent"))]
        beatmap: Beatmap,
    },
    Achievement {
        #[serde(flatten)]
        #[cfg_attr(feature = "export", tsify(type = "BaseEvent"))]
        base: BaseEvent,
        #[cfg_attr(feature = "export", tsify(type = "Achievement"))]
        achievement: Achievement,
    },
    BeatmapsetUpdate {
        #[serde(flatten)]
        #[cfg_attr(feature = "export", tsify(type = "BaseEvent"))]
        base: BaseEvent,
        #[cfg_attr(feature = "export", tsify(type = "BeatmapsetInEvent"))]
        beatmapset: Beatmapset,
    },
    UserSupportAgain {
        #[serde(flatten)]
        #[cfg_attr(feature = "export", tsify(type = "BaseEvent"))]
        base: BaseEvent,
    },
    // 添加其他类型...

    // 捕获所有其他未知格式
    Unknown(#[cfg_attr(feature = "export", tsify(type = "BaseEvent"))] BaseEvent),
}

// 在代码中处理 Event 枚举时，可以根据 base.event_type 来区分类型
impl Event {
    pub fn get_type(&self) -> &str {
        match self {
            Event::Rank { base, .. } => &base.event_type,
            Event::Achievement { base, .. } => &base.event_type,
            Event::BeatmapsetUpdate { base, .. } => &base.event_type,
            Event::UserSupportAgain { base } => &base.event_type,
            Event::Unknown(base) => &base.event_type,
        }
    }
}

// 辅助结构体定义...
#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InEvent")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmap {
    pub title: String,
    pub url: String,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InEvent")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Achievement {
    pub icon_url: String,
    pub id: u64,
    pub name: String,
    pub grouping: String,
    pub ordering: u32,
    pub slug: String,
    pub description: String,
    pub mode: Option<String>,
    pub instructions: Option<String>,
}

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(
    feature = "export",
    tsify(into_wasm_abi, from_wasm_abi, type_suffix = "InEvent")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmapset {
    pub title: String,
    pub url: String,
}
