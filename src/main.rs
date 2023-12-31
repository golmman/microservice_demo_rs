use std::convert::Infallible;
use std::error::Error;

use model::error_response::ErrorResponse;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

use crate::ct_client::CtClient;
use crate::delete_customer::delete_customer;
use crate::read_customer::read_customer;
use crate::upsert_customer::upsert_customer;

mod ct_client;
mod delete_customer;
mod get_ct_customer;
mod model;
mod read_customer;
mod upsert_customer;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        //.filter_level(log::LevelFilter::max())
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    println!("Initializing commercetools client...");
    let ct_client = CtClient::new().await;

    println!("Starting server on localhost:3030 ...");

    let read_customer_route = warp::get()
        .and(warp::path!("customer" / String))
        .and_then(read_customer);

    // reqwest client is an arc, so cloning is fine
    let c = ct_client.clone();
    let upsert_customer_route = warp::post()
        .map(move || c.clone())
        .and(warp::path!("customer" / String))
        .and(warp::body::json())
        .and_then(upsert_customer);

    let c = ct_client.clone();
    let delete_customer_route = warp::delete()
        .map(move || c.clone())
        .and(warp::path!("customer" / String))
        .and_then(delete_customer);

    warp::serve(
        read_customer_route
            .or(upsert_customer_route)
            .or(delete_customer_route)
            .recover(handle_rejection),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
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
