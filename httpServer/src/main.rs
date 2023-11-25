use http::Method;
use http::Request;

mod server;
mod http;
fn main() {
    let server = server::Server::new(String::from("127.0.0.1:8080"));
    server.run();
}


