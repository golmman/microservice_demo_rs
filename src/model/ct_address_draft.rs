use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::address::Address;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtAddressDraft {
    pub city: Option<String>,
    pub country: String,
    pub street_name: Option<String>,
    pub postal_code: Option<String>,
}

impl From<Address> for CtAddressDraft {
    fn from(address: Address) -> Self {
        let Address {
            city,
            street_name,
            postal_code,
        } = address;

        Self {
            city,
            country: String::from("DE"),
            street_name,
            postal_code,
        }
    }
}
