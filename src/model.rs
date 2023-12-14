use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct CtAccessToken {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street_name: String,
    pub city: String,
    pub zip_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}
