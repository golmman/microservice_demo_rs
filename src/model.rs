use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct CtAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub scope: String,
    pub token_type: String,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerResponse {
    pub limit: u32,
    pub total: u32,
}
