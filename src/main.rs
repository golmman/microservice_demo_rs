use serde_derive::Deserialize;
use serde_derive::Serialize;
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
    code: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let read_customer_route = warp::path!("hello" / String).map(read_customer);

    let upsert_customer_route = warp::post()
        .and(warp::path("customer"))
        .and(warp::body::json())
        .map(|employee: Customer| warp::reply::json(&employee));

    warp::serve(read_customer_route.or(upsert_customer_route))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn read_customer(name: String) -> String {
    format!("Hello, {}!", name)
}
