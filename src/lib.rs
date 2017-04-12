//! A CORS middleware for Iron.
//!
//! See https://www.html5rocks.com/static/images/cors_server_flowchart.png for
//! reference.
//!
//! Preflight requests are not yet supported.
//!
//! # Usage
//!
//! There are two modes available:
//!
//! ## Mode 1: Whitelist
//!
//! The user of the middleware must specify a list of allowed hosts (port or
//! protocol aren't being checked by the middleware). If the `Origin` header is
//! set on a request and if the value matches one of the allowed hosts, the
//! `Access-Control-Allow-Origin` header for that host is added to the response.
//!
//! Initialize the middleware with a `HashSet` of allowed host strings:
//!
//! ```rust
//! use std::collections::HashSet;
//! use iron_cors::CorsMiddleware;
//!
//! let allowed_hosts = ["example.com"].iter()
//!                                    .map(ToString::to_string)
//!                                    .collect::<HashSet<_>>();
//! let middleware = CorsMiddleware::with_whitelist(allowed_hosts);
//! ```
//!
//! See
//! [`examples/whitelist.rs`](https://github.com/dbrgn/iron-cors-rs/blob/master/examples/whitelist.rs)
//! for a full usage example.
//!
//! ## Mode 2: Allow Any
//!
//! Alternatively, the user of the middleware can choose to allow requests from
//! any origin. In that case, the `Access-Control-Allow-Origin` header is added
//! to any request with an `Origin` header.
//!
//! ```rust
//! use iron_cors::CorsMiddleware;
//!
//! let middleware = CorsMiddleware::with_allow_any();
//! ```
//!
//! See
//! [`examples/allow_any.rs`](https://github.com/dbrgn/iron-cors-rs/blob/master/examples/allow_any.rs)
//! for a full usage example.

extern crate iron;
#[macro_use] extern crate log;

use std::collections::HashSet;

use iron::{Request, Response, IronResult, AroundMiddleware, Handler};
use iron::headers;

/// The struct that holds the CORS configuration.
pub struct CorsMiddleware {
    allowed_hosts: Option<HashSet<String>>,
}

impl CorsMiddleware {
    /// Specify which origin hosts are allowed to access the resource.
    pub fn with_whitelist(allowed_hosts: HashSet<String>) -> Self {
        CorsMiddleware {
            allowed_hosts: Some(allowed_hosts),
        }
    }

    /// Allow all origins to access the resource. The
    /// `Access-Control-Allow-Origin` header of the response will be set to
    /// `*`.
    pub fn with_allow_any() -> Self {
        CorsMiddleware {
            allowed_hosts: None,
        }
    }
}

impl AroundMiddleware for CorsMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        match self.allowed_hosts {
            Some(allowed_hosts) => Box::new(CorsHandlerWhitelist {
                handler: handler,
                allowed_hosts: allowed_hosts,
            }),
            None => Box::new(CorsHandlerAllowAny {
                handler: handler,
            }),
        }
    }
}

/// Handler for whitelist based rules.
struct CorsHandlerWhitelist {
    handler: Box<Handler>,
    allowed_hosts: HashSet<String>,
}

/// Handler if allowing any origin.
struct CorsHandlerAllowAny {
    handler: Box<Handler>,
}

impl CorsHandlerWhitelist {
    fn add_cors_header(&self, headers: &mut headers::Headers, origin: &headers::Origin) {
        let header = match origin.host.port {
            Some(port) => format!("{}://{}:{}", &origin.scheme, &origin.host.hostname, &port),
            None => format!("{}://{}", &origin.scheme, &origin.host.hostname),
        };
        headers.set(headers::AccessControlAllowOrigin::Value(header));
    }
}

/// The handler that acts as an AroundMiddleware.
///
/// It first checks an incoming request for appropriate CORS headers. If the
/// `Origin` header is present and the header value is in the whitelist, add
/// the `Access-Control-Allow-Origin` header for that domain to the response.
/// Otherwise, the request is processed as usual.
impl Handler for CorsHandlerWhitelist {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // Extract origin header
        let origin = match req.headers.get::<headers::Origin>().cloned() {
            Some(o) => o,
            None => {
                return self.handler.handle(req);
            }
        };

        // Verify origin header
        let may_process = self.allowed_hosts.contains(&origin.host.hostname);

        // Process request
        if may_process {
            // Everything OK, process request and add CORS header to response
            self.handler.handle(req)
                .map(|mut res| { self.add_cors_header(&mut res.headers, &origin); res })
                .map_err(|mut err| { self.add_cors_header(&mut err.response.headers, &origin); err })
        } else {
            // Not adding headers
            warn!("Got disallowed CORS request from {}", &origin.host.hostname);
            self.handler.handle(req)
        }
    }
}

impl CorsHandlerAllowAny {
    fn add_cors_header(&self, headers: &mut headers::Headers) {
        headers.set(headers::AccessControlAllowOrigin::Any);
    }
}

/// The handler that acts as an AroundMiddleware.
///
/// It first checks an incoming request for appropriate CORS headers. If the
/// `Origin` header is present, then the `Access-Control-Allow-Origin: *`
/// header is added to the response. If not, the request is processed as usual.
impl Handler for CorsHandlerAllowAny {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match req.headers.get::<headers::Origin>() {
            None => {
                self.handler.handle(req)
            },
            Some(_) => {
                self.handler.handle(req)
                    .map(|mut res| { self.add_cors_header(&mut res.headers); res })
                    .map_err(|mut err| { self.add_cors_header(&mut err.response.headers); err })
            },
        }

    }
}
