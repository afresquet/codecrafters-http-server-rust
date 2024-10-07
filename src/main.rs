use std::{io::Write, net::TcpListener};

use codecrafters_http_server::*;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let response = HttpResponse {
                    status_code: StatusCode::OK,
                };

                stream.write_all(response.to_string().as_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
