use std::convert::Infallible;

use crate::ct_client::CtClient;
use crate::model::ct_customer::CtCustomer;
use crate::model::ct_customer_created::CtCustomerCreated;
use crate::model::ct_customer_response::CtCustomerResponse;
use crate::model::customer::Customer;
use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;

pub async fn get_ct_customer(
    ct_client: &CtClient,
    email: &String,
) -> Option<CtCustomer> {
    println!("getting ct customer {email}...");

    let ct_customer_response_raw = ct_client
        .get("/customers")
        .query(&[("where", format!("email=\"{email}\""))])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str::<CtCustomerResponse>(&ct_customer_response_raw)
        .ok()
        .filter(|c| {
            !c.results
                .is_empty()
        })
        .map(|c| c.results[0].clone())
}
