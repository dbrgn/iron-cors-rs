extern crate iron;
extern crate iron_cors;

use std::collections::HashSet;

use iron::{Iron, Handler, Request, Response, IronResult, Chain, status};
use iron_cors::CorsMiddleware;

struct HelloWorldHandler;

impl Handler for HelloWorldHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }
}

fn main() {
    // Initialize handler
    let handler = HelloWorldHandler {};

    // Initialize middleware
    let allowed_hosts = ["example.com"].iter().map(ToString::to_string).collect::<HashSet<_>>();
    println!("Allowed origin hosts: {:?}", allowed_hosts);
    let cors_middleware = CorsMiddleware::with_whitelist(allowed_hosts);

    // Setup chain with middleware
    let mut chain = Chain::new(handler);
    chain.link_around(cors_middleware);

    // Start server
    println!("Starting new server on 127.0.0.1:3000...");
    Iron::new(chain).http(("127.0.0.1", 3000)).unwrap();
}
