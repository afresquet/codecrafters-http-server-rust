use std::fmt::Display;

pub enum StatusCode {
    OK,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StatusCode::OK => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        };

        write!(f, "{s}")
    }
}
