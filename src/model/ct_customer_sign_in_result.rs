use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_customer::CtCustomer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerSignInResult {
    pub customer: CtCustomer,
}

impl CtCustomerSignInResult {
    pub fn from_str(json: &str) -> Option<Self> {
        serde_json::from_str::<Self>(&json).ok()
    }
}
