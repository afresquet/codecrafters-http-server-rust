use std::path::PathBuf;

use codecrafters_http_server::{Body, Request, Response, StatusCode};

pub fn handler(request: Request, mut directory: PathBuf) -> Response {
    let file_name = request.target.split('/').last().expect("has file name");
    directory.push(file_name);

    let Ok(file) = std::fs::read(directory) else {
        return Response::builder()
            .status_code(StatusCode::NotFound)
            .build();
    };

    Response::builder().body(Body::File(file)).build()
}
