use std::{fs, path::PathBuf};

use codecrafters_http_server::{Body, Request, Response, StatusCode};

pub fn handler_get(request: Request, mut directory: PathBuf) -> Response {
    let file_name = request.target.split('/').last().expect("has file name");
    directory.push(file_name);

    let Ok(file) = fs::read(directory) else {
        return Response::builder()
            .status_code(StatusCode::NotFound)
            .build();
    };

    Response::builder().body(Body::File(file)).build()
}

pub fn handler_post(request: Request, mut directory: PathBuf) -> Response {
    let file_name = request.target.split('/').last().expect("has file name");
    directory.push(file_name);

    fs::write(directory, request.body).expect("file is written");

    Response::builder().status_code(StatusCode::Created).build()
}
