use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowedRequest {
    Get,
    Post,
    Put,
    Delete,
}

impl fmt::Display for AllowedRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllowedRequest::Get => write!(f, "GET"),
            AllowedRequest::Post => write!(f, "POST"),
            AllowedRequest::Put => write!(f, "PUT"),
            AllowedRequest::Delete => write!(f, "DELETE"),
        }
    }
}

impl AllowedRequest {
    pub fn from_str(request: &str) -> Option<Self> {
        match request {
            "GET" => Some(AllowedRequest::Get),
            "POST" => Some(AllowedRequest::Post),
            "PUT" => Some(AllowedRequest::Put),
            "DELETE" => Some(AllowedRequest::Delete),
            _ => None,
        }
    }
}
