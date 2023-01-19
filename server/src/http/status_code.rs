use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

#[allow(dead_code)]
impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "ok",
            Self::BadRequest => "bad request",
            Self::NotFound => "not found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
