use std::convert::Infallible;

use crate::ct_client::CtClient;
use crate::model::ct_customer::CtCustomer;
use crate::model::ct_customer_created::CtCustomerCreated;
use crate::model::customer::Customer;
use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;

pub async fn upsert_customer(
    ct_client: CtClient,
    email: String,
    customer: Customer,
) -> Result<impl warp::reply::Reply, Infallible> {
    println!("{:?}", customer);

    let ct_customer_request = CtCustomer::from(customer);

    let ct_customer_response_raw = ct_client
        .post("/customers")
        .json(&ct_customer_request)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let reply = if let Ok(ct_customer_created) =
        serde_json::from_str::<CtCustomerCreated>(&ct_customer_response_raw)
    {
        Reply::created(&Customer::from(ct_customer_created))
    } else {
        Reply::error(
            ErrorCode::CustomerCreationFailed,
            &ct_customer_response_raw,
        )
    };

    Ok(warp::reply::with_status(reply.response, reply.status))
}
