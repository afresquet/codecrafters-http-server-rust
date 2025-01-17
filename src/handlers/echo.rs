use codecrafters_http_server::{Body, Request, Response};

pub fn handler(request: Request) -> Response {
    let body = request.target.split('/').last().expect("has body to echo");
    let mut respose_builder = Response::builder();

    let accept_encoding_header = request
        .headers
        .get("Accept-Encoding")
        .map(|header| header.split(", ").collect::<Vec<_>>());
    match accept_encoding_header {
        Some(compression_schemes) if compression_schemes.contains(&"gzip") => {
            respose_builder = respose_builder
                .header("Content-Encoding", "gzip")
                .body(Body::gzip(body).expect("to work"));
        }
        _ => {
            respose_builder = respose_builder.body(Body::TextPlain(body.to_string()));
        }
    }

    respose_builder.build()
}
