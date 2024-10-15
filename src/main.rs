use std::{io::Write, net::TcpListener, path::PathBuf};

use clap::Parser;

use codecrafters_http_server::*;

mod handlers;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    let directory = args.directory.unwrap_or_else(|| PathBuf::from("/tmp"));

    for stream in listener.incoming() {
        let directory = directory.clone();
        std::thread::spawn(|| match stream {
            Ok(mut stream) => {
                let request = Request::from_stream(&mut stream).expect("request can be read");

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
                    Request {
                        method: Method::GET,
                        ref target,
                        ..
                    } if target.starts_with("/files") => {
                        handlers::files::handler_get(request, directory)
                    }
                    Request {
                        method: Method::POST,
                        ref target,
                        ..
                    } if target.starts_with("/files") => {
                        handlers::files::handler_post(request, directory)
                    }
                    _ => Response::builder()
                        .status_code(StatusCode::NotFound)
                        .build(),
                };

                write!(stream, "{response}").expect("response can be written");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }

    Ok(())
}
