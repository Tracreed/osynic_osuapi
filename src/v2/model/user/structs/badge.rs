use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    pub awarded_at: String,
    pub description: String,
    pub image_url: String,
    #[serde(rename = "image@2x_url")]
    pub image_2x_url: String,
    pub url: String,
}

// {
//     "awarded_at": "2023-11-26T22:57:45+00:00",
//     "description": "Longstanding commitment to World Cup Pooling (2 years)",
//     "image@2x_url": "https://assets.ppy.sh/profile-badges/tournament-helpers/pooling-2@2x.png",
//     "image_url": "https://assets.ppy.sh/profile-badges/tournament-helpers/pooling-2.png",
//     "url": "https://osu.ppy.sh/wiki/en/Tournaments#official-world-cups"
// },
