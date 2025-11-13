// UserAccountHistory
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAccountHistory {
    pub description: Option<String>,
    pub id: u32,
    pub length: u32, // In seconds.
    pub permanent: bool,
    pub timestamp: String, // Timestamp
    #[serde(rename = "type")]
    pub typee: String, // note, restriction, or silence.
}

//     UserAccountHistory
// Field 	Type 	Description
// description 	string?
// id 	integer
// length 	integer 	In seconds.
// permanent 	boolean
// timestamp 	Timestamp
// type 	string 	note, restriction, or silence.
