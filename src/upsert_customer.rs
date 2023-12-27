use std::convert::Infallible;

use warp::http::StatusCode;
use warp::reply::Reply;

use crate::ct_client::CtClient;
use crate::model::CtCustomer;
use crate::model::Customer;

pub async fn upsert_customer(
    ct_client: CtClient,
    email: String,
    customer: Customer,
) -> Result<impl Reply, Infallible> {
    println!("{:?}", customer);

    let ct_customer_response = ct_client
        .post("/customers")
        .json(&customer)
        .send()
        .await
        .unwrap()
        .json::<CtCustomer>()
        //.text()
        .await
        .unwrap();

    let json = warp::reply::json(&Customer {
        email,
        first_name: None,
        last_name: None,
        date_of_birth: None,
        addresses: Some(Vec::new()),
    });

    let code = StatusCode::CREATED;

    Ok(warp::reply::with_status(json, code))
}
