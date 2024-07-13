use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowedRequest {
    Get,
    Put,
    Delete,
    LIST,
}

impl fmt::Display for AllowedRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllowedRequest::Get => write!(f, "GET"),
            AllowedRequest::Put => write!(f, "PUT"),
            AllowedRequest::Delete => write!(f, "DELETE"),
            AllowedRequest::LIST => write!(f, "LIST"),
        }
    }
}

impl AllowedRequest {
    pub fn from_str(request: &str) -> Option<Self> {
        match request {
            "GET" => Some(AllowedRequest::Get),
            "PUT" => Some(AllowedRequest::Put),
            "DELETE" => Some(AllowedRequest::Delete),
            "LIST" => Some(AllowedRequest::LIST),
            _ => None,
        }
    }
}
