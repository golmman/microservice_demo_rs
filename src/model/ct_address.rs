use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::address::Address;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtAddress {
    pub city: Option<String>,
    pub street_name: Option<String>,
    pub zip_code: Option<String>,
}

impl From<Address> for CtAddress {
    fn from(address: Address) -> Self {
        let Address {
            city,
            street_name,
            zip_code,
        } = address;

        Self {
            city,
            street_name,
            zip_code,
        }
    }
}
