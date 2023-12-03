use std::convert::Infallible;
use std::error::Error;

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
struct ErrorResponse {
    code: String,
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
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorResponse {
        code: String::from("todo"),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
