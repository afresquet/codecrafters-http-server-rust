use codecrafters_http_server::{Body, Request, Response};

pub fn handler(request: Request) -> Response {
    let user_agent = request
        .headers
        .get("User-Agent")
        .expect("has User-Agent header")
        .to_owned();
    Response::builder()
        .body(Body::TextPlain(user_agent))
        .build()
}
