use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
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
        let buf_reader = BufReader::new(stream);

        Self::from_buffer(buf_reader)
    }

    pub fn from_buffer<T: Read>(mut buf_reader: BufReader<T>) -> anyhow::Result<Self> {
        let (method, target, version) = {
            let mut request_line = String::new();
            buf_reader.read_line(&mut request_line)?;
            let mut request_line = request_line.split_whitespace();
            let method = request_line
                .next()
                .expect("has method")
                .parse()
                .expect("method is valid");
            let target = request_line.next().expect("has target").into();
            let version = request_line.next().expect("has version").into();
            (method, target, version)
        };

        let headers = {
            let mut headers = HashMap::new();
            loop {
                let mut header = String::new();
                buf_reader.read_line(&mut header)?;
                if header.trim().is_empty() {
                    break;
                }
                let (k, v) = header
                    .trim()
                    .split_once(": ")
                    .expect("headers are delimited by a colon and space");
                headers.insert(k.to_string(), v.to_string());
            }
            headers
        };

        let body = {
            let size = headers
                .get("Content-Length")
                .map(|n| n.parse::<usize>().expect("is number"))
                .unwrap_or_default();
            let mut body = vec![0; size];
            if !buf_reader.buffer().is_empty() {
                buf_reader.read_exact(&mut body)?;
            }
            body
        };

        Ok(Self {
            method,
            target,
            version,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = "GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\nfoobar";
        let buf_reader = BufReader::new(request.as_bytes());
        assert_eq!(
            Request::from_buffer(buf_reader).unwrap(),
            Request {
                method: Method::GET,
                target: "/index.html".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HashMap::from([
                    ("User-Agent".to_string(), "curl/7.64.1".to_string()),
                    ("Host".to_string(), "localhost:4221".to_string()),
                    ("Accept".to_string(), "*/*".to_string())
                ]),
                body: "foobar".as_bytes().to_vec()
            }
        )
    }
}
