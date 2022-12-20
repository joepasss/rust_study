struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Server running in {}", self.addr);
    }
}

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
