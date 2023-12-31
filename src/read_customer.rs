use warp::http::StatusCode;
use warp::reply::Reply;

use std::convert::Infallible;

use crate::model::customer::Customer;

pub async fn read_customer(email: String) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&Customer {
        email,
        first_name: None,
        last_name: None,
        date_of_birth: None,
        addresses: Some(Vec::new()),
    });

    let code = StatusCode::OK;

    Ok(warp::reply::with_status(json, code))
}
