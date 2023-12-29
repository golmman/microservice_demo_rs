use std::convert::Infallible;

use warp::http::StatusCode;
use warp::reply::Reply;

use crate::ct_client::CtClient;
use crate::model::ct_customer::CtCustomer;
use crate::model::ct_customer_created::CtCustomerCreated;
use crate::model::customer::Customer;
use crate::model::error_response::ErrorResponse;

pub async fn upsert_customer(
    ct_client: CtClient,
    email: String,
    customer: Customer,
) -> Result<impl Reply, Infallible> {
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

    let (response, status) = if let Ok(ct_customer_created) =
        serde_json::from_str::<CtCustomerCreated>(&ct_customer_response_raw)
    {
        (
            serde_json::to_string(&Customer::from(ct_customer_created))
                .unwrap(),
            StatusCode::CREATED,
        )
    } else {
        (
            serde_json::to_string(&ErrorResponse {
                code: String::from("CUSTOMER_CREATION_FAILED"),
                message: String::from(ct_customer_response_raw),
            })
            .unwrap(),
            StatusCode::BAD_GATEWAY,
        )
    };

    Ok(warp::reply::with_status(response, status))
}
