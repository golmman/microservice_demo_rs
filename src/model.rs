use warp::http::StatusCode;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub enum ErrorCode {
    CustomerCreationFailed,
}

pub struct Reply {
    pub response: String,
    pub status: StatusCode,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: Option<String>,
    pub street_name: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub addresses: Option<Vec<Address>>,
    pub date_of_birth: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtAddress {
    pub city: Option<String>,
    pub street_name: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerResponse {
    pub limit: u32,
    pub total: u32,
    pub results: Vec<CtCustomer>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomer {
    pub addresses: Option<Vec<CtAddress>>,
    pub date_of_birth: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CtCustomerCreated {
    pub customer: CtCustomer,
}

impl From<CtAddress> for Address {
    fn from(ct_address: CtAddress) -> Self {
        let CtAddress {
            city,
            street_name,
            zip_code,
        } = ct_address;

        Self {
            city,
            street_name,
            zip_code,
        }
    }
}

impl From<CtCustomer> for Customer {
    fn from(ct_customer: CtCustomer) -> Self {
        let CtCustomer {
            addresses,
            date_of_birth,
            email,
            first_name,
            last_name,
            password,
        } = ct_customer;

        let addresses = addresses.map(|a| {
            a.into_iter()
                .map(|b| Address::from(b))
                .collect()
        });

        Self {
            email,
            first_name,
            last_name,
            date_of_birth,
            addresses,
        }
    }
}

impl From<CtCustomerCreated> for Customer {
    fn from(ct_customer_created: CtCustomerCreated) -> Self {
        Customer::from(ct_customer_created.customer)
    }
}

impl From<Address> for CtAddress {
    fn from(address: Address) -> Self {
        let Address {
            city,
            street_name,
            zip_code,
        } = address;

        Self {
            city,
            street_name,
            zip_code,
        }
    }
}

impl From<Customer> for CtCustomer {
    fn from(customer: Customer) -> Self {
        let Customer {
            email,
            first_name,
            last_name,
            date_of_birth,
            addresses,
        } = customer;

        let addresses = addresses.map(|a| {
            a.into_iter()
                .map(|b| CtAddress::from(b))
                .collect()
        });

        Self {
            addresses,
            date_of_birth,
            email,
            first_name,
            last_name,
            password: Some(String::from("admin_admin")),
        }
    }
}
