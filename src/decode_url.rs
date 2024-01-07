// TODO: switch to axum
pub fn decode_url(url: String) -> String {
    url.replace("%40", "@")
}
