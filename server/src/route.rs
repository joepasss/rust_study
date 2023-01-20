use super::{
    http::{response::Response, Method, Request, StatusCode},
    server::Handler,
};
use std::fs;

pub struct Route {
    public_path: String,
}

impl Route {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }

            Err(_) => None,
        }
    }
}

impl Handler for Route {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("200.json")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, self.read_file("404.json")),
                },
            },
            _ => Response::new(StatusCode::NotFound, self.read_file("404.json")),
        }
    }
}
