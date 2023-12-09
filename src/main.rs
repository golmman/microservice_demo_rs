use std::convert::Infallible;
use std::error::Error;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Address {
    street_name: String,
    city: String,
    zip_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Customer {
    email: String,
    first_name: String,
    last_name: String,
    date_of_birth: String,
    addresses: Vec<Address>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    code: String,
    message: String,
}

#[tokio::main]
async fn main() {
    println!("Starting server on localhost:3030 ...");

    let read_customer_route = warp::get()
        .and(warp::path!("customer" / String))
        .and_then(read_customer);

    let upsert_customer_route = warp::post()
        .and(warp::path!("customer" / String))
        .and(warp::body::json())
        .and_then(upsert_customer);

    warp::serve(
        read_customer_route
            .or(upsert_customer_route)
            .recover(handle_rejection),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}

async fn read_customer(email: String) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&Customer {
        email,
        first_name: String::from("Helge"),
        last_name: String::from("Schneider"),
        date_of_birth: String::from(""),
        addresses: Vec::new(),
    });

    let code = StatusCode::OK;

    Ok(warp::reply::with_status(json, code))
}

async fn upsert_customer(
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

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) =
        err.find::<warp::filters::body::BodyDeserializeError>()
    {
        message = match e.source() {
            Some(cause) => {
                if cause
                    .to_string()
                    .contains("denom")
                {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
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
