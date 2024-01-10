use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::address::Address;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtAddress {
    pub city: Option<String>,
    pub country: String,
    pub id: String,
    pub street_name: Option<String>,
    pub postal_code: Option<String>,
}
