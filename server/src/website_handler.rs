use super::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};
pub struct WebSiteHandler {
    public_path: String,
}

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>WELCOME!</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>HELLO!</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
