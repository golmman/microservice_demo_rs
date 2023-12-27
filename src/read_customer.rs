use warp::http::StatusCode;
use warp::reply::Reply;

use std::collections::HashMap;
use std::convert::Infallible;

use crate::Customer;

async fn fetch_data() -> Result<HashMap<String, String>, reqwest::Error> {
    let data_source_url = "https://httpbin.org/ip";
    let res = reqwest::get(data_source_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("Time elapsed doing a request {:?}ms", res);
    Ok(res)
}

pub async fn read_customer(email: String) -> Result<impl Reply, Infallible> {
    let resp = fetch_data().await;
    let r = match resp {
        Ok(data) => Ok(warp::reply::json(&data)),
        Err(_err) => Err(warp::reject()),
    };

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
