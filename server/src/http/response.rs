use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[allow(dead_code)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

#[allow(dead_code)]
impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nAccept:application/json\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body,
        )
    }
}
