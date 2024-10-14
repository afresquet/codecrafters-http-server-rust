use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::Method;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub target: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> anyhow::Result<Self> {
        let buf = BufReader::new(stream);
        let lines = buf.lines().map_while(Result::ok);

        Self::from_parts(lines)
    }

    pub fn from_parts(mut parts: impl Iterator<Item = String>) -> anyhow::Result<Self> {
        let request_line = parts.next().expect("has request line");
        let mut request_line = request_line.split_whitespace();
        let method = request_line
            .next()
            .expect("has method")
            .parse()
            .expect("method is valid");
        let target = request_line.next().expect("has target").into();
        let version = request_line.next().expect("has version").into();

        Ok(Self {
            method,
            target,
            version,
            headers: Default::default(),
            body: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = "GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";
        let parts = request.split_whitespace().map(ToString::to_string);
        assert_eq!(
            Request::from_parts(parts).unwrap(),
            Request {
                method: Method::GET,
                target: "/index.html".to_string(),
                version: "1.1".to_string(),
                headers: Default::default(),
                body: Default::default()
            }
        )
    }
}
