use std::{io::Write, net::TcpListener};

use codecrafters_http_server::*;

mod handlers;

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
                    } if &target[..] == "/" => Response::default(),
                    Request {
                        method: Method::GET,
                        ref target,
                        ..
                    } if target.starts_with("/echo") => handlers::echo::handler(request),
                    Request {
                        method: Method::GET,
                        ref target,
                        ..
                    } if target.starts_with("/user-agent") => {
                        handlers::user_agent::handler(request)
                    }
                    _ => Response::builder()
                        .status_code(StatusCode::NotFound)
                        .build(),
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
