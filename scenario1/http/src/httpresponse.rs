use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        
        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        write!(write_stream, "{}", response_string).unwrap();
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }
    
    fn status_text(&self) -> &str {
        self.status_text
    }
    
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }

        header_string
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res1.version(),
            res1.status_code(),
            res1.status_text(),
            res1.headers(),
            res.body.unwrap().len(),
            res1.body(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_http_response() {
        let response = HttpResponse::default();
        assert_eq!(response.version, "HTTP/1.1");
        assert_eq!(response.status_code, "200");
        assert_eq!(response.status_text, "OK");
        assert!(response.headers.is_none());
        assert!(response.body.is_none());
    }

    #[test]
    fn test_custom_http_response() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        let body = Some("{\"key\":\"value\"}".to_string());

        let response = HttpResponse::new("404", Some(headers.clone()), body.clone());

        assert_eq!(response.version, "HTTP/1.1");
        assert_eq!(response.status_code, "404");
        assert_eq!(response.status_text, "Not Found");
        assert_eq!(response.headers, Some(headers));
        assert_eq!(response.body, body);
    }

    #[test]
    fn test_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        let response = HttpResponse::new("200", Some(headers.clone()), None);

        assert_eq!(headers, response.headers.unwrap());
    }

    #[test]
    fn test_body() {
        let body = Some("Hello, world!".to_string());
        let response = HttpResponse::new("200", None, body.clone());

        assert_eq!(response.body(), body.unwrap().as_str());
    }
}
