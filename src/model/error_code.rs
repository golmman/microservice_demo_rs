use warp::http::StatusCode;

#[derive(Clone, Debug)]
pub enum ErrorCode {
    CustomerCreationFailed,
    CustomerDeletionFailed,
    CustomerNotFound,
    CustomerUpdateFailed,
    InconsistentEmail,
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
            ErrorCode::InconsistentEmail => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<ErrorCode> for StatusCode {
    fn from(error_code: ErrorCode) -> Self {
        StatusCode::from(&error_code)
    }
}
