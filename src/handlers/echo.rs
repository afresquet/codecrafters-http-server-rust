use codecrafters_http_server::{Body, Request, Response};

pub fn handler(request: Request) -> Response {
    let body = request.target.split('/').last().expect("has body to echo");
    Response::builder()
        .body(Body::TextPlain(body.to_string()))
        .build()
}
