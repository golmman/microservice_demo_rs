use std::convert::Infallible;

use warp::http::StatusCode;
use warp::reply::Reply;

use crate::model::Customer;

pub async fn upsert_customer(
    email: String,
    customer: Customer,
) -> Result<impl Reply, Infallible> {
    println!("{:?}", customer);

    let json = warp::reply::json(&Customer {
        email,
        first_name: String::from("Helge"),
        last_name: String::from("Schneider"),
        date_of_birth: String::from(""),
        addresses: Vec::new(),
    });

    let code = StatusCode::CREATED;

    Ok(warp::reply::with_status(json, code))
}
