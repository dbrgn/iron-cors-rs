extern crate iron;
extern crate iron_cors;
extern crate iron_test;

use std::collections::HashSet;
use std::io::{Error, ErrorKind};

use iron::{Handler, Request, Response, IronResult, IronError, Chain, status};
use iron::headers::{Headers, Origin, AccessControlAllowOrigin};
use self::iron_test::{request, response};
use iron_cors::CorsMiddleware;

struct HelloWorldHandler;
impl Handler for HelloWorldHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }
}

struct ForbiddenHandler;
impl Handler for ForbiddenHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Forbidden, "You shall not pass!")))
    }
}

struct ErrorResultHandler;
impl Handler for ErrorResultHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Err(IronError::new(
            Error::new(ErrorKind::Other, "terrible things"),
            (status::InternalServerError, "Oh noes")
        ))
    }
}

macro_rules! setup_handler {
    ("whitelist": $allowed_hosts:expr) => {{
        let mut chain = Chain::new(HelloWorldHandler {});
        let whitelist = $allowed_hosts.iter().map(ToString::to_string).collect::<HashSet<_>>();
        chain.link_around(CorsMiddleware::with_whitelist(whitelist));
        chain
    }};
    ("any": $allow_invalid:expr) => {{
        let mut chain = Chain::new(HelloWorldHandler {});
        chain.link_around(CorsMiddleware::with_allow_any($allow_invalid));
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
    let handler = setup_handler!("whitelist": ["example.org"]);
    let headers = Headers::new();
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::BadRequest));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Invalid CORS request: Origin header missing");
}

#[test]
fn test_host_disallowed() {
    let handler = setup_handler!("whitelist": ["example.org"]);
    let headers = setup_origin_header!("forbidden.org");
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::BadRequest));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Invalid CORS request: Origin not allowed");
}

#[test]
fn test_host_allowed() {
    let handler = setup_handler!("whitelist": ["example.org"]);
    let headers = setup_origin_header!("example.org");
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Hello, world!");
}

#[test]
fn test_allow_any() {
    let handler = setup_handler!("any": false);
    let headers = setup_origin_header!("example.org");
    let response = request::get("http://example.org:3000/hello", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Hello, world!");
}

#[test]
fn test_allow_any_missing_header_allowed() {
    let handler = setup_handler!("any": true);
    let response = request::get("http://example.org:3000/hello", Headers::new(), &handler).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Hello, world!");
}

#[test]
fn test_allow_any_missing_header_denied() {
    let handler = setup_handler!("any": false);
    let response = request::get("http://example.org:3000/hello", Headers::new(), &handler).unwrap();
    assert_eq!(response.status, Some(status::BadRequest));
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Invalid CORS request: Origin header missing");
}

#[test]
fn test_intended_error_status() {
    //! A regular non-200 response should contain CORS headers.
    let mut handler = Chain::new(ForbiddenHandler {});
    handler.link_around(CorsMiddleware::with_allow_any(true));
    let headers = setup_origin_header!("example.org");
    let response = request::get("http://example.org:3000/forbidden", headers, &handler).unwrap();
    assert_eq!(response.status, Some(status::Forbidden));
    assert!(response.headers.has::<AccessControlAllowOrigin>());
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "You shall not pass!");
}

#[test]
fn test_unexpected_error_status_allow_any() {
    //! A response from an error inside a handler should contain CORS headers.
    let mut handler = Chain::new(ErrorResultHandler {});
    handler.link_around(CorsMiddleware::with_allow_any(true));
    let headers = setup_origin_header!("example.org");
    let error = request::get("http://example.org:3000/err", headers, &handler).unwrap_err();
    let response = error.response;
    assert_eq!(response.status, Some(status::InternalServerError));
    assert!(response.headers.has::<AccessControlAllowOrigin>());
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Oh noes");
}

#[test]
fn test_unexpected_error_status_whitelist() {
    //! A response from an error inside a handler should contain CORS headers.
    let mut handler = Chain::new(ErrorResultHandler {});
    let whitelist = ["example.org"].iter().map(ToString::to_string).collect::<HashSet<_>>();
    handler.link_around(CorsMiddleware::with_whitelist(whitelist));
    let headers = setup_origin_header!("example.org");
    let error = request::get("http://example.org:3000/err", headers, &handler).unwrap_err();
    let response = error.response;
    assert_eq!(response.status, Some(status::InternalServerError));
    assert!(response.headers.has::<AccessControlAllowOrigin>());
    let result_body = response::extract_body_to_string(response);
    assert_eq!(&result_body, "Oh noes");
}
