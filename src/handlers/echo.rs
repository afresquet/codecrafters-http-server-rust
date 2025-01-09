use codecrafters_http_server::{Body, Request, Response};

pub fn handler(request: Request) -> Response {
    let body = request.target.split('/').last().expect("has body to echo");
    let mut respose_builder = Response::builder().body(Body::TextPlain(body.to_string()));
    match request.headers.get("Accept-Encoding") {
        Some(header) if header == "gzip" => {
            respose_builder = respose_builder.header("Content-Encoding", "gzip");
        }
        _ => (),
    }
    respose_builder.build()
}
