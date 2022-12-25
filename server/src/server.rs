use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Server running in {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // == while true {}
            continue;

            break;
        }
    }
}
