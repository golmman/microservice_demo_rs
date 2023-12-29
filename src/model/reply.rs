use serde::Serialize;
use warp::http::StatusCode;

use super::error_code::ErrorCode;
use super::error_response::ErrorResponse;

pub struct Reply {
    pub response: String,
    pub status: StatusCode,
}

impl Reply {
    pub fn created<T>(value: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        let response = serde_json::to_string(value).unwrap();
        let status = StatusCode::CREATED;

        Self { response, status }
    }

    pub fn error(error_code: ErrorCode, message: &String) -> Self {
        let response = serde_json::to_string(&ErrorResponse {
            code: String::from(&error_code),
            message: message.clone(),
        })
        .unwrap();
        let status = StatusCode::from(&error_code);

        Self { response, status }
    }
}
