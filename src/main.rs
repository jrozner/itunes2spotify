#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate plist;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate hyper;
extern crate url;

use plist::Plist;

use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::Read;

use rocket_contrib::JSON;

use hyper::Url;
use hyper::header::{Headers, Authorization, Basic, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use url::form_urlencoded;

mod spotify;

use spotify::client::Client;

#[derive(Serialize, Deserialize, Debug)]
struct WebUser {
    username: String,
}

static CLIENT_ID: &'static str = "d33a252362b3454ba9c83d18443f92e6";
static CLIENT_SECRET: &'static str = "6417e038dfe9446294761e8ed785c955";
static REDIRECT_URI: &'static str = "http://2bf15b08.ngrok.io/oauth/callback";

#[post("/users", data = "<user>")]
fn create_user(user: JSON<WebUser>) -> JSON<HashMap<String, String>> {
    let client = Client::new(CLIENT_ID, CLIENT_SECRET, REDIRECT_URI);

    JSON(map!{
        "url".to_string() => client.authorize_url()
    })
}

#[derive(FromForm)]
struct OAuthResponse {
    code: Option<String>,
    error: Option<String>,
}

#[get("/oauth/callback?<response>")]
fn oauth_callback(response: OAuthResponse) -> JSON<HashMap<String, String>> {
    if response.error.is_some() {
        return JSON(map!{
            "error".to_string() => response.error.unwrap(),
        });
    }

    let client = Client::new(CLIENT_ID, CLIENT_SECRET, REDIRECT_URI);

    let access_token = match client.access_token_for_user(&response.code.unwrap()) {
        Ok(access_token) => access_token,
        Err(_) => {
            return JSON(map!{"error".to_string() => "unable to get access token".to_string()})
        }
    };

    match client.user_profile("jrozner", &access_token.access_token) {
        Ok(user) => user,
        Err(_) => {
            return JSON(map!{"error".to_string() => "Unable to retrieve user profile".to_string()})
        }
    };

    JSON(map!{"nothing".to_string() => "nothing".to_string()})
}

fn main() {
    rocket::ignite().mount("/", routes![create_user, oauth_callback]).launch();
}

fn read_library(filename: String) -> HashSet<(String, String)> {
    let file = File::open(filename).unwrap();
    let plist = Plist::read(file).unwrap();

    let doc = plist.as_dictionary().expect("Malformed plist");

    let tracks =
        doc.get("Tracks").expect("No tracked").as_dictionary().expect("Malformed track listing");

    let mut pairs = HashSet::new();

    for track in tracks.iter() {
        let track_data = track.1.as_dictionary().expect("Malformed track data");
        let artist = match track_data.get("Artist") {
            Some(a) => a.as_string().expect("Malformed artist").to_string(),
            None => String::new(),
        };
        let album = match track_data.get("Album") {
            Some(a) => a.as_string().expect("Malformed album").to_string(),
            None => String::new(),
        };

        if album == "" || (artist == "" && album == "") {
            continue;
        }

        pairs.insert((artist, album));
    }

    pairs
}

struct Album {
    artist: String,
    name: String,
    art_url: String,
}
