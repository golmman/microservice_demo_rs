use std::convert::Infallible;

use crate::ct_client::CtClient;
use crate::get_ct_customer::get_ct_customer;
use crate::model::ct_customer::CtCustomer;
use crate::model::ct_customer_created::CtCustomerCreated;
use crate::model::customer::Customer;
use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;

pub async fn delete_customer(
    ct_client: CtClient,
    email: String,
) -> Result<impl warp::reply::Reply, Infallible> {
    println!("deleting customer {email}...");

    let ct_customer = get_ct_customer(&ct_client, &email).await;

    println!("{ct_customer:?}");
    panic!();

    let ct_customer_response_raw = ct_client
        .delete("/customers")
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
