use log::error;
use log::info;
use log::warn;
use std::convert::Infallible;

use crate::ct_client::CtClient;
use crate::get_ct_customer::get_ct_customer;
use crate::model::ct_customer::CtCustomer;
use crate::model::customer::Customer;
use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;

pub async fn delete_customer(
    ct_client: CtClient,
    email: String,
) -> Result<impl warp::reply::Reply, Infallible> {
    let Reply { response, status } = execute_request(ct_client, email).await;

    Ok(warp::reply::with_status(response, status))
}

async fn execute_request(ct_client: CtClient, email: String) -> Reply {
    let Some(ct_customer) = get_ct_customer(&ct_client, &email).await else {
        warn!("customer {email} not found");
        return Reply::error(
            ErrorCode::CustomerNotFound,
            &format!(
                "Customer deletion failed: customer {email} does not exist"
            ),
        );
    };

    let path = format!("/customers/{}", ct_customer.id.unwrap());
    let query = [("version", ct_customer.version.unwrap())];

    info!("deleting ct customer {email}...");
    let ct_customer_raw = ct_client
        .delete(&path)
        .query(&query)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let Some(ct_customer) = CtCustomer::from_str(&ct_customer_raw) else {
        error!("{ct_customer_raw:?}");
        return Reply::error(
            ErrorCode::CustomerDeletionFailed,
            "Customer deletion failed: unexpected commercetools response",
        );
    };

    Reply::ok(&Customer::from(ct_customer))
}
