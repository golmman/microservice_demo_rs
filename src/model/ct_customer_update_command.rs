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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_update_actions_which_remove_addresses() {
        let ct_addresses = Some(vec![
            CtAddress {
                city: None,
                country: String::from("DE"),
                id: String::from("1"),
                street_name: None,
                postal_code: None,
            },
            CtAddress {
                city: None,
                country: String::from("DE"),
                id: String::from("2"),
                street_name: None,
                postal_code: None,
            },
        ]);

        let actions = remove_addresses(ct_addresses);

        assert_eq!(actions[0].action, "removeAddress");
        assert_eq!(actions[0].address_id, Some(String::from("1")));
        assert_eq!(actions[1].action, "removeAddress");
        assert_eq!(actions[1].address_id, Some(String::from("2")));
    }

    #[test]
    fn it_creates_update_actions_which_add_addresses() {
        let addresses = Some(vec![
            Address {
                city: Some(String::from("Berlin")),
                street_name: None,
                postal_code: None,
            },
            Address {
                city: None,
                street_name: None,
                postal_code: Some(String::from("1234")),
            },
        ]);

        let actions = add_addresses(addresses);

        assert_eq!(actions[0].action, "addAddress");
        assert_eq!(
            actions[0].address.as_ref().unwrap().city,
            Some(String::from("Berlin"))
        );
        assert_eq!(actions[1].action, "addAddress");
        assert_eq!(
            actions[1].address.as_ref().unwrap().postal_code,
            Some(String::from("1234"))
        );
    }
}
