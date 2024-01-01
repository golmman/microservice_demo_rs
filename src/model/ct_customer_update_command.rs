use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_customer_update_action::CtCustomerUpdateAction;
use super::customer::Customer;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerUpdateCommand {
    version: u32,
    actions: Vec<CtCustomerUpdateAction>,
}

impl From<(Customer, u32)> for CtCustomerUpdateCommand {
    fn from((customer, version): (Customer, u32)) -> Self {
        let Customer {
            email,
            first_name,
            last_name,
            date_of_birth,
            ..
        } = customer;

        let actions = vec![
            CtCustomerUpdateAction::set_date_of_birth(date_of_birth),
            CtCustomerUpdateAction::change_email(Some(email)),
            CtCustomerUpdateAction::set_first_name(first_name),
            CtCustomerUpdateAction::set_last_name(last_name),
        ];

        Self { version, actions }
    }
}
