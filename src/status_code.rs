use std::fmt::Display;

pub enum StatusCode {
    OK,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StatusCode::OK => "200 OK",
        };

        write!(f, "{s}")
    }
}
