use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_address_draft::CtAddressDraft;
use super::customer::Customer;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerDraft {
    pub addresses: Option<Vec<CtAddressDraft>>,
    pub date_of_birth: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub version: Option<u32>,
}

impl From<Customer> for CtCustomerDraft {
    fn from(customer: Customer) -> Self {
        let Customer {
            email,
            first_name,
            last_name,
            date_of_birth,
            addresses,
        } = customer;

        let addresses = addresses
            .map(|a| a.into_iter().map(|b| CtAddressDraft::from(b)).collect());

        Self {
            addresses,
            date_of_birth,
            email,
            first_name,
            id: None,
            last_name,
            password: Some(String::from("admin_admin")),
            version: None,
        }
    }
}
