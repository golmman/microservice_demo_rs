use log::error;
use log::warn;
use warp::filters::body::BodyDeserializeError;

use std::convert::Infallible;
use std::error::Error;

use crate::model::error_code::ErrorCode;
use crate::model::reply::Reply;
use warp::reject::MethodNotAllowed;
use warp::reject::Rejection;

pub async fn handle_rejection(
    err: Rejection,
) -> Result<impl warp::reply::Reply, Infallible> {
    let Reply { response, status } = create_error_reply(err);

    Ok(warp::reply::with_status(response, status))
}

fn create_error_reply(err: Rejection) -> Reply {
    if err.is_not_found() {
        warn!("{err:?}");
        return Reply::error(
            ErrorCode::ResourceNotFound,
            "No resource was found for the given request.",
        );
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        warn!("{err:?}");
        return Reply::error(
            ErrorCode::DeserializationFailed,
            &e.source()
                .map(|s| s.to_string())
                .unwrap_or("reason undetermined".to_string()),
        );
    } else if err.find::<MethodNotAllowed>().is_some() {
        warn!("{err:?}");
        return Reply::error(
            ErrorCode::MethodNotAllowed,
            "The request method is not allowed to be used in this context.",
        );
    }

    error!("{err:?}");
    Reply::error(
        ErrorCode::InternalServerError,
        "An internal server error occured.",
    )
}
