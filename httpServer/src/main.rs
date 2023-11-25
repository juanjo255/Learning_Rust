fn main() {
    let server = server::Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

mod server{
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
}

mod http{

    mod request{
        struct Request {
            path:String,
            query_string:Option<String>,
            method: super::method::Method,
        }
    }
    
    mod method {

        pub enum Method {
            GET,
            POST,
            DELETE,
        }    
    }
}

