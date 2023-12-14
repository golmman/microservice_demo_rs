use serde_derive::Deserialize;
use serde_derive::Serialize;

use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}

pub struct CtClient {
    access_token: String,
}

impl CtClient {
    pub async fn new() -> Self {
        let client = reqwest::Client::new();

        let project_key = env::var("CT_PROJECT_KEY").unwrap();
        let client_id = env::var("CT_CLIENT_ID").unwrap();
        let client_secret = env::var("CT_SECRET").unwrap();
        let scope = env::var("CT_SCOPE").unwrap();
        let api_url = env::var("CT_API_URL").unwrap();
        let auth_url = env::var("CT_AUTH_URL").unwrap();

        let resp = client
            .post(format!("{auth_url}/oauth/token"))
            .header("content-length", 0)
            .query(&[("grant_type", "client_credentials")])
            .query(&[("scope", scope)])
            .basic_auth(client_id, Some(client_secret))
            .send()
            .await
            .unwrap()
            .json::<AccessToken>()
            .await
            .unwrap();

        println!("{:#?}", resp);

        Self {
            access_token: String::from(""),
        }
    }
}
