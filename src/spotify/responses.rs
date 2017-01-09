use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    display_name: String,
    external_urls: HashMap<String, String>,
    followers: Followers,
    href: String,
    id: String,
    images: Vec<Image>,
    #[serde(rename = "type")]
    account_type: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Followers {
    href: Option<String>,
    total: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    height: Option<i32>,
    url: String,
    width: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    token_type: String,
    scope: String,
    expires_in: i32,
    refresh_token: String,
}
