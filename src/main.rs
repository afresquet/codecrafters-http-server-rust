use std::{io::Write, net::TcpListener};

use codecrafters_http_server::*;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let request = Request::from_stream(&mut stream)?;

                let response = match request {
                    Request {
                        method: Method::GET,
                        target,
                        ..
                    } if &target[..] == "/" => HttpResponse {
                        status_code: StatusCode::OK,
                    },
                    _ => HttpResponse {
                        status_code: StatusCode::NotFound,
                    },
                };

                write!(stream, "{response}")?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
