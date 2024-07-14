use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowedRequest {
    Get,
    UPLOAD,
    Delete,
    LIST,
}

impl fmt::Display for AllowedRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllowedRequest::Get => write!(f, "GET"),
            AllowedRequest::UPLOAD => write!(f, "PUT"),
            AllowedRequest::Delete => write!(f, "DELETE"),
            AllowedRequest::LIST => write!(f, "LIST"),
        }
    }
}

impl AllowedRequest {
    pub fn from_str(request: &str) -> Option<Self> {
        match request {
            req if req.starts_with("PUT") => Some(AllowedRequest::UPLOAD),
            req if req.starts_with("LIST") => Some(AllowedRequest::LIST),
            _ => None,
        }
    }
}
