extern crate hyper;
extern crate serde_json;
extern crate url;

use hyper::Client as HyperClient;
use hyper::Url;
use hyper::header::{Headers, Authorization, Basic, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use url::form_urlencoded;

use spotify::responses::{AccessToken, User};
use spotify::error::Error;
use spotify::error::Error::{Hyper, Json};

pub struct Client {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Client {
        Client {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
        }
    }

    pub fn authorize_url(&self) -> String {
        let mut url = Url::parse("https://accounts.spotify.com/authorize").unwrap();

        url.query_pairs_mut()
            .clear()
            .append_pair("client_id", &self.client_id)
            .append_pair("response_type", "code")
            .append_pair("scope", "user-library-modify")
            .append_pair("redirect_uri", &self.redirect_uri);

        url.to_string()
    }

    pub fn access_token_for_user(&self, code: &str) -> Result<AccessToken, Error> {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: self.client_id.clone(),
            password: Some(self.client_secret.clone()),
        }));
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])));

        let body = form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", code)
            .append_pair("redirect_uri", &self.redirect_uri)
            .finish();

        let response = HyperClient::new().post("https://accounts.spotify.com/api/token")
            .headers(headers)
            .body(&body)
            .send()?;

        let access_token = try!(serde_json::from_reader(response));
        Ok(access_token)
    }

    pub fn user_profile(&self, user: &str, access_token: &str) -> Result<User, Error> {
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: access_token.to_string() }));

        let response = HyperClient::new().get(&format!("https://api.spotify.com/v1/users/{}", user))
            .headers(headers)
            .send()?;

        let user = try!(serde_json::from_reader(response));
        Ok(user)
    }
}
