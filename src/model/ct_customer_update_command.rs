use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::address::Address;
use super::ct_address::CtAddress;
use super::ct_address_draft::CtAddressDraft;
use super::ct_customer::CtCustomer;
use super::ct_customer_update_action::CtCustomerUpdateAction;
use super::customer::Customer;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerUpdateCommand {
    version: u32,
    actions: Vec<CtCustomerUpdateAction>,
}

impl From<(Customer, CtCustomer)> for CtCustomerUpdateCommand {
    fn from((customer, ct_customer): (Customer, CtCustomer)) -> Self {
        let Customer {
            email,
            first_name,
            last_name,
            date_of_birth,
            addresses,
        } = customer;

        let CtCustomer {
            version,
            addresses: ct_addresses,
            ..
        } = ct_customer;

        let mut actions = vec![
            CtCustomerUpdateAction::set_date_of_birth(date_of_birth),
            CtCustomerUpdateAction::change_email(Some(email)),
            CtCustomerUpdateAction::set_first_name(first_name),
            CtCustomerUpdateAction::set_last_name(last_name),
        ];

        actions.append(&mut remove_addresses(ct_addresses));
        actions.append(&mut add_addresses(addresses));

        Self { version, actions }
    }
}

fn remove_addresses(
    ct_addresses: Option<Vec<CtAddress>>,
) -> Vec<CtCustomerUpdateAction> {
    let Some(ct_addresses) = ct_addresses else {
        return Vec::new();
    };

    let mut actions = Vec::new();
    for ct_address in ct_addresses {
        actions.push(CtCustomerUpdateAction::remove_address(ct_address.id));
    }

    actions
}

fn add_addresses(
    addresses: Option<Vec<Address>>,
) -> Vec<CtCustomerUpdateAction> {
    let Some(addresses) = addresses else {
        return Vec::new();
    };

    let mut actions = Vec::new();
    for address in addresses {
        actions.push(CtCustomerUpdateAction::add_address(
            CtAddressDraft::from(address),
        ));
    }

    actions
}
