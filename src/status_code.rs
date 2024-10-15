use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    #[default]
    OK,
    Created,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StatusCode::OK => "200 OK",
            Self::Created => "201 Created",
            StatusCode::NotFound => "404 Not Found",
        };

        write!(f, "{s}")
    }
}
