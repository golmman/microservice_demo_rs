use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_customer::CtCustomer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerResponse {
    pub limit: u32,
    pub total: u32,
    pub results: Vec<CtCustomer>,
}
