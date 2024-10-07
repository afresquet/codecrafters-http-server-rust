use std::fmt::Display;

use crate::StatusCode;

pub struct HttpResponse {
    pub status_code: StatusCode,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {}\r\n\r\n", self.status_code)
    }
}
