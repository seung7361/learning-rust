use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}


#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut parsed_method = 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }
    
    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
}