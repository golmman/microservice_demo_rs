use warp::http::StatusCode;

#[derive(Clone, Debug)]
pub enum ErrorCode {
    CustomerCreationFailed,
    CustomerDeletionFailed,
    CustomerNotFound,
    CustomerUpdateFailed,
    DeserializationFailed,
    InconsistentEmail,
    InternalServerError,
    MethodNotAllowed,
    ResourceNotFound,
}

impl From<&ErrorCode> for String {
    fn from(error_code: &ErrorCode) -> Self {
        format!("{error_code:?}")
    }
}

impl From<ErrorCode> for String {
    fn from(error_code: ErrorCode) -> Self {
        String::from(&error_code)
    }
}

impl From<&ErrorCode> for StatusCode {
    fn from(error_code: &ErrorCode) -> Self {
        match error_code {
            ErrorCode::CustomerCreationFailed => StatusCode::BAD_GATEWAY,
            ErrorCode::CustomerDeletionFailed => StatusCode::BAD_GATEWAY,
            ErrorCode::CustomerNotFound => StatusCode::NOT_FOUND,
            ErrorCode::CustomerUpdateFailed => StatusCode::BAD_GATEWAY,
            ErrorCode::DeserializationFailed => StatusCode::BAD_REQUEST,
            ErrorCode::InconsistentEmail => StatusCode::BAD_REQUEST,
            ErrorCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ErrorCode::ResourceNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<ErrorCode> for StatusCode {
    fn from(error_code: ErrorCode) -> Self {
        StatusCode::from(&error_code)
    }
}
