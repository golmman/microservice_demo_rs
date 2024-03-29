use std::convert::Infallible;

use log::error;
use log::info;
use log::warn;

use crate::ct_client::CtClient;
use crate::decode_url::decode_url;
use crate::get_ct_customer::get_ct_customer;
use crate::model::ct_customer::CtCustomer;
use crate::model::ct_customer_draft::CtCustomerDraft;
use crate::model::ct_customer_sign_in_result::CtCustomerSignInResult;
use crate::model::ct_customer_update_command::CtCustomerUpdateCommand;
use crate::model::customer::Customer;
use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;

pub async fn upsert_customer(
    ct_client: CtClient,
    email: String,
    customer: Customer,
) -> Result<impl warp::reply::Reply, Infallible> {
    let email = decode_url(email);

    let Reply { response, status } =
        execute_request(ct_client, email, customer).await;

    Ok(warp::http::Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", "application/json")
        .status(status)
        .body(response))
}

async fn execute_request(
    ct_client: CtClient,
    email: String,
    customer: Customer,
) -> Reply {
    if let Some(ct_customer) = get_ct_customer(&ct_client, &email).await {
        update_customer(ct_client, email, customer, ct_customer).await
    } else {
        create_customer(ct_client, email, customer).await
    }
}

async fn create_customer(
    ct_client: CtClient,
    email: String,
    customer: Customer,
) -> Reply {
    if email != customer.email {
        warn!(
            "email in path: {}, email in body: {}",
            email, customer.email
        );
        return Reply::error(
            ErrorCode::InconsistentEmail,
            "Customer creation failed: email in path and body don't match",
        );
    }

    let ct_customer_draft = CtCustomerDraft::from(customer);

    info!("creating ct customer {email}...");
    let ct_customer_raw = ct_client
        .post("/customers")
        .json(&ct_customer_draft)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let Some(ct_customer_created) =
        CtCustomerSignInResult::from_str(&ct_customer_raw)
    else {
        error!("{ct_customer_raw:?}");
        return Reply::error(
            ErrorCode::CustomerCreationFailed,
            "Customer creation failed: unexpected commercetools response",
        );
    };

    Reply::created(&Customer::from(ct_customer_created))
}

async fn update_customer(
    ct_client: CtClient,
    email: String,
    customer: Customer,
    ct_customer: CtCustomer,
) -> Reply {
    let path = format!("/customers/{}", ct_customer.id);
    let ct_customer_update_command =
        CtCustomerUpdateCommand::from((customer, ct_customer));

    info!("updating ct customer {email}...");
    let ct_customer_raw = ct_client
        .post(&path)
        .json(&ct_customer_update_command)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let Some(ct_customer) = CtCustomer::from_str(&ct_customer_raw) else {
        error!("{ct_customer_raw:?}");
        return Reply::error(
            ErrorCode::CustomerUpdateFailed,
            "Customer update failed: unexpected commercetools response",
        );
    };

    Reply::ok(&Customer::from(ct_customer))
}
