use warp::http::StatusCode;

pub struct Reply {
    pub response: String,
    pub status: StatusCode,
}
