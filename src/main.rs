use env_logger::Target;
use log::info;
use log::LevelFilter;
use warp::Filter;

use crate::ct_client::CtClient;
use crate::delete_customer::delete_customer;
use crate::handle_rejection::handle_rejection;
use crate::read_customer::read_customer;
use crate::upsert_customer::upsert_customer;

mod ct_client;
mod delete_customer;
mod get_ct_customer;
mod handle_rejection;
mod model;
mod read_customer;
mod upsert_customer;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        //.filter_level(LevelFilter::max())
        .target(Target::Stdout)
        .init();

    info!("Initializing commercetools client...");
    let ct_client = CtClient::new().await;

    // reqwest client is an arc, so cloning is fine
    let c = ct_client.clone();
    let read_customer_route = warp::get()
        .map(move || c.clone())
        .and(warp::path!("customer" / String))
        .and_then(read_customer);

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

    let cors_route = warp::options().map(|| {
        warp::http::Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .status(204)
            .body("")
    });

    warp::serve(
        read_customer_route
            .or(upsert_customer_route)
            .or(delete_customer_route)
            .or(cors_route)
            .recover(handle_rejection),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}
