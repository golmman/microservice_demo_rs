use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::ct_address_draft::CtAddressDraft;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerUpdateAction {
    action: String,
    address: Option<CtAddressDraft>,
    address_id: Option<String>,
    date_of_birth: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl CtCustomerUpdateAction {
    fn new(action: &str) -> Self {
        Self {
            action: String::from(action),
            address: None,
            address_id: None,
            date_of_birth: None,
            email: None,
            first_name: None,
            last_name: None,
        }
    }

    pub fn set_date_of_birth(date_of_birth: Option<String>) -> Self {
        let mut action = Self::new("setDateOfBirth");
        action.date_of_birth = date_of_birth;
        action
    }

    pub fn change_email(email: Option<String>) -> Self {
        let mut action = Self::new("changeEmail");
        action.email = email;
        action
    }

    pub fn set_first_name(first_name: Option<String>) -> Self {
        let mut action = Self::new("setFirstName");
        action.first_name = first_name;
        action
    }

    pub fn set_last_name(last_name: Option<String>) -> Self {
        let mut action = Self::new("setLastName");
        action.last_name = last_name;
        action
    }

    pub fn remove_address(address_id: String) -> Self {
        let mut action = Self::new("removeAddress");
        action.address_id = Some(address_id);
        action
    }

    pub fn add_address(address: CtAddressDraft) -> Self {
        let mut action = Self::new("addAddress");
        action.address = Some(address);
        action
    }
}
