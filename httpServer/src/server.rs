pub struct Server {
        addr: String,
    }
    impl Server {
        pub fn new(addr: String) -> Self {
            return Self { addr: addr };
        }
        pub fn run(self) {
            // Self takes ownership
            println!("Listening on {}", self.addr);
        }
    }