use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_address::CtAddress;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomer {
    pub addresses: Option<Vec<CtAddress>>,
    pub date_of_birth: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: String,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub version: u32,
}

impl CtCustomer {
    pub fn from_str(json: &str) -> Option<Self> {
        serde_json::from_str::<Self>(&json).ok()
    }
}
