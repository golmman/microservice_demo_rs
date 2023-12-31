use std::env;

use reqwest::RequestBuilder;

use crate::model::ct_access_token_response::CtAccessTokenResponse;

#[allow(dead_code)]
#[derive(Clone)]
pub struct CtClient {
    access_token: String,
    api_url: String,
    auth_url: String,
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
    project_key: String,
    scope: String,
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

        let ct_access_token_response = client
            .post(format!("{auth_url}/oauth/token"))
            .header("content-length", 0)
            .query(&[("grant_type", "client_credentials")])
            .query(&[("scope", &scope)])
            .basic_auth(&client_id, Some(&client_secret))
            .send()
            .await
            .unwrap()
            .json::<CtAccessTokenResponse>()
            .await
            .unwrap();

        let access_token = ct_access_token_response.access_token;

        Self {
            access_token,
            api_url,
            auth_url,
            client,
            client_id,
            client_secret,
            project_key,
            scope,
        }
    }

    pub fn delete(&self, ct_path: &str) -> RequestBuilder {
        self.client
            .delete(format!("{}/{}{}", self.api_url, self.project_key, ct_path))
            .bearer_auth(&self.access_token)
    }

    pub fn get(&self, ct_path: &str) -> RequestBuilder {
        self.client
            .get(format!("{}/{}{}", self.api_url, self.project_key, ct_path))
            .bearer_auth(&self.access_token)
    }

    pub fn post(&self, ct_path: &str) -> RequestBuilder {
        self.client
            .post(format!("{}/{}{}", self.api_url, self.project_key, ct_path))
            .bearer_auth(&self.access_token)
    }
}
