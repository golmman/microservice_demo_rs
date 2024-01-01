use log::warn;

use std::convert::Infallible;

use crate::ct_client::CtClient;
use crate::get_ct_customer::get_ct_customer;
use crate::model::customer::Customer;
use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;

pub async fn read_customer(
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
            &format!("Customer read failed: customer {email} does not exist"),
        );
    };

    Reply::ok(&Customer::from(ct_customer))
}
