extern crate iron;
extern crate iron_cors;
extern crate iron_test;

use iron::{Handler, Request, Response, IronResult, Chain, status};
use iron::headers::{Headers, Origin};
use self::iron_test::{request, response};
use iron_cors::CorsMiddleware;

struct HelloWorldHandler;

impl Handler for HelloWorldHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }
}

macro_rules! setup_handler {
    ( $allowed_hosts:expr ) => {{
        let mut chain = Chain::new(HelloWorldHandler {});
        chain.link_around(CorsMiddleware::new($allowed_hosts));
        chain
    }};
}

macro_rules! setup_origin_header {
    ( $origin_host:expr ) => {{
        let mut headers = Headers::new();
        headers.set(Origin::new("http", $origin_host, None));
        headers
    }};
}

#[test]
fn test_no_middleware() {
    let response = request::get("http://localhost:3000/hello",
        Headers::new(),
        &HelloWorldHandler).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Hello, world!");
}

#[test]
fn test_missing_origin_header() {
    let handler = setup_handler!(vec!["example.org".to_string()]);
    let headers = Headers::new();
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::BadRequest));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Invalid CORS request: Origin header missing");
}

#[test]
fn test_host_disallowed() {
    let handler = setup_handler!(vec!["example.org".to_string()]);
    let headers = setup_origin_header!("forbidden.org");
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::BadRequest));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Invalid CORS request: Origin not allowed");
}

#[test]
fn test_host_allowed() {
    let handler = setup_handler!(vec!["example.org".to_string()]);
    let headers = setup_origin_header!("example.org");
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Hello, world!");
}
