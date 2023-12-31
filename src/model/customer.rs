use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::address::Address;
use super::ct_customer::CtCustomer;
use super::ct_customer_created::CtCustomerSignInResult;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub addresses: Option<Vec<Address>>,
    pub date_of_birth: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<CtCustomer> for Customer {
    fn from(ct_customer: CtCustomer) -> Self {
        let CtCustomer {
            addresses,
            date_of_birth,
            email,
            first_name,
            last_name,
            ..
        } = ct_customer;

        let addresses = addresses
            .map(|a| a.into_iter().map(|b| Address::from(b)).collect());

        Self {
            email,
            first_name,
            last_name,
            date_of_birth,
            addresses,
        }
    }
}

impl From<CtCustomerSignInResult> for Customer {
    fn from(ct_customer_created: CtCustomerSignInResult) -> Self {
        Customer::from(ct_customer_created.customer)
    }
}
