use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerUpdateAction {
    action: String,
    date_of_birth: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl CtCustomerUpdateAction {
    pub fn set_date_of_birth(date_of_birth: Option<String>) -> Self {
        Self {
            action: String::from("setDateOfBirth"),
            date_of_birth,
            email: None,
            first_name: None,
            last_name: None,
        }
    }

    pub fn change_email(email: Option<String>) -> Self {
        Self {
            action: String::from("changeEmail"),
            date_of_birth: None,
            email,
            first_name: None,
            last_name: None,
        }
    }

    pub fn set_first_name(first_name: Option<String>) -> Self {
        Self {
            action: String::from("setFirstName"),
            date_of_birth: None,
            email: None,
            first_name,
            last_name: None,
        }
    }

    pub fn set_last_name(last_name: Option<String>) -> Self {
        Self {
            action: String::from("setLastName"),
            date_of_birth: None,
            email: None,
            first_name: None,
            last_name,
        }
    }
}
