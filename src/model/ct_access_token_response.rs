use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct CtAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub scope: String,
    pub token_type: String,
}
