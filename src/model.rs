use serde_derive::Deserialize;
use serde_derive::Serialize;
use warp::http::StatusCode;

pub mod address;
pub mod ct_access_token_response;
pub mod ct_address;
pub mod ct_customer;
pub mod ct_customer_created;
pub mod ct_customer_response;
pub mod customer;
pub mod error_code;
pub mod error_response;
pub mod reply;
