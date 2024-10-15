use std::{collections::HashMap, fmt::Display};

use crate::StatusCode;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Response {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<Body>,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}\r\n"))
            .collect::<Vec<_>>()
            .join("");
        let body = self
            .body
            .as_ref()
            .map(|body| body.to_string())
            .unwrap_or_default();
        write!(
            f,
            "HTTP/1.1 {}\r\n{}\r\n{}",
            self.status_code, headers, body
        )
    }
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ResponseBuilder {
    status_code: Option<StatusCode>,
    headers: HashMap<String, String>,
    body: Option<Body>,
}

impl ResponseBuilder {
    pub fn build(self) -> Response {
        Response {
            status_code: self.status_code.unwrap_or_default(),
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        let _ = self.status_code.insert(status_code);
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn body(mut self, body: Body) -> Self {
        self.headers
            .insert("Content-Type".to_string(), body.content_type().to_string());
        self.headers
            .insert("Content-Length".to_string(), body.len().to_string());
        let _ = self.body.insert(body);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    TextPlain(String),
    File(Vec<u8>),
}

impl Body {
    pub fn len(&self) -> usize {
        match self {
            Body::TextPlain(body) => body.len(),
            Body::File(bytes) => bytes.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            Body::TextPlain(_) => "text/plain",
            Body::File(_) => "application/octet-stream",
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::TextPlain("".to_string())
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Body::TextPlain(body) => write!(f, "{body}"),
            Body::File(bytes) => {
                write!(f, "{}", String::from_utf8(bytes.to_vec()).expect("is utf8"))
            }
        }
    }
}
