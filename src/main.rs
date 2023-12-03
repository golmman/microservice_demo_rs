use std::convert::Infallible;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Address {
    street_name: String,
    city: String,
    zip_code: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Customer {
    email: String,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    addresses: Vec<Address>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Error {
    code: u16,
    message: String,
}

#[tokio::main]
async fn main() {
    let read_customer_route = warp::path!("hello" / String).map(read_customer);

    let upsert_customer_route = warp::post()
        .and(warp::path("customer"))
        .and(warp::body::json())
        .map(|employee: Customer| warp::reply::json(&employee));

    println!("Starting server on localhost:3030 ...");

    warp::serve(
        read_customer_route
            .or(upsert_customer_route)
            .recover(handle_rejection),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}

fn read_customer(name: String) -> String {
    format!("Hello, {}!", name)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code = StatusCode::METHOD_NOT_ALLOWED;

    let json = warp::reply::json(&Error {
        code: code.as_u16(),
        message: String::from("Method not allowed."),
    });

    Ok(warp::reply::with_status(json, code))
}
