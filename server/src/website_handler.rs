use super::{
    http::{Request, Response, StatusCode},
    server::Handler,
};
pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}
