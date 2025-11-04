// Get user recent
use osynic_osuapi::error::Result;
use osynic_osuapi::v1::client::request::client::OsynicOsuApiV1Client;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::recent::GetUserRecentParams;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY")
        .expect("Please set the API_KEY environment variable to a valid osu! API v1 API key");
    let client = OsynicOsuApiV1Client::new(api_key.clone());
    let params = GetUserRecentParams::default()
        .user("Islatri".to_string())
        .mode(3);

    let recents = client.user.get_user_recent(params).await?;
    println!("{:?}", recents);

    Ok(())
}

/*
ReqwestUserRecent get_user_recent
[RecentPlay { beatmap_id: "1070639", score: "622070", maxcombo: "253", count50: "1", count100: "1", count300: "62", countmiss: "0", countkatu: "19", countgeki: "79", perfect: "0", enabled_mods: "32768", user_id: "31175842", date: "2025-10-31 05:53:35", rank: "S" }]

*/
