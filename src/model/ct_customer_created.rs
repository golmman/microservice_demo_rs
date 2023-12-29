use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_customer::CtCustomer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerCreated {
    pub customer: CtCustomer,
}
