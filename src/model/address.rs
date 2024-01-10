use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_address::CtAddress;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: Option<String>,
    pub street_name: Option<String>,
    pub postal_code: Option<String>,
}

impl From<CtAddress> for Address {
    fn from(ct_address: CtAddress) -> Self {
        let CtAddress {
            city,
            street_name,
            postal_code,
            ..
        } = ct_address;

        Self {
            city,
            street_name,
            postal_code,
        }
    }
}
