//Taken from https://github.com/dfinity/ic/blob/1c7f774ae0c096d98f873fa66f1d200b85a64ecf/rs/rust_canisters/http_types/src/lib.rs

use candid::{CandidType, Deserialize};
use serde_bytes::ByteBuf;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
}

impl HttpRequest {
    pub fn path(&self) -> &str {
        match self.url.find('?') {
            None => &self.url[..],
            Some(index) => &self.url[..index],
        }
    }

    /// Searches for the first appearance of a parameter in the request URL.
    /// Returns `None` if the given parameter does not appear in the query.
    #[allow(dead_code)]
    pub fn raw_query_param(&self, param: &str) -> Option<&str> {
        const QUERY_SEPARATOR: &str = "?";
        let query_string = self.url.split(QUERY_SEPARATOR).nth(1)?;
        if query_string.is_empty() {
            return None;
        }
        const PARAMETER_SEPARATOR: &str = "&";
        for chunk in query_string.split(PARAMETER_SEPARATOR) {
            const KEY_VALUE_SEPARATOR: &str = "=";
            let mut split = chunk.splitn(2, KEY_VALUE_SEPARATOR);
            let name = split.next()?;
            if name == param {
                return Some(split.next().unwrap_or_default());
            }
        }
        None
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
}

pub struct HttpResponseBuilder(HttpResponse);

impl HttpResponseBuilder {
    pub fn ok() -> Self {
        Self(HttpResponse {
            status_code: 200,
            headers: vec![],
            body: ByteBuf::default(),
        })
    }

    #[allow(dead_code)]
    pub fn bad_request() -> Self {
        Self(HttpResponse {
            status_code: 400,
            headers: vec![],
            body: ByteBuf::from("bad request"),
        })
    }

    pub fn not_found() -> Self {
        Self(HttpResponse {
            status_code: 404,
            headers: vec![],
            body: ByteBuf::from("not found"),
        })
    }

    #[allow(dead_code)]
    pub fn server_error(reason: impl ToString) -> Self {
        Self(HttpResponse {
            status_code: 500,
            headers: vec![],
            body: ByteBuf::from(reason.to_string()),
        })
    }

    pub fn header(&mut self, name: impl ToString, value: impl ToString) {
        self.0.headers.push((name.to_string(), value.to_string()));
    }

    #[allow(dead_code)]
    pub fn body(&mut self, bytes: impl Into<Vec<u8>>) {
        self.0.body = ByteBuf::from(bytes.into());
    }

    #[allow(dead_code)]
    pub fn with_body_and_content_length(&mut self, bytes: impl Into<Vec<u8>>) {
        let bytes = bytes.into();
        self.header("Content-Length", bytes.len());
        self.body(bytes);
    }

    pub fn build(self) -> HttpResponse {
        self.0
    }
}

#[test]
fn test_raw_query_param() {
    fn request_with_url(url: String) -> HttpRequest {
        HttpRequest {
            method: "".to_string(),
            url,
            headers: vec![],
            body: Default::default(),
        }
    }
    let http_request = request_with_url("/endpoint?time=1000".to_string());
    assert_eq!(http_request.raw_query_param("time"), Some("1000"));
    let http_request = request_with_url("/endpoint".to_string());
    assert_eq!(http_request.raw_query_param("time"), None);
    let http_request =
        request_with_url("/endpoint?time=1000&time=1001&other=abcde&time=1002".to_string());
    assert_eq!(http_request.raw_query_param("time"), Some("1000"));
}
