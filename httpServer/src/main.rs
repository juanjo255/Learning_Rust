use http::Method;
use http::Request;
use server::Server;
use website_handler::WebsiteHandler;
mod http;
mod server;
mod website_handler;

#[allow(dead_code)]
fn main() {
    let server = server::Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler);
}
