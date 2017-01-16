//! A CORS middleware for Iron.
//!
//! See https://www.html5rocks.com/static/images/cors_server_flowchart.png for
//! reference.
//!
//! The middleware will return `HTTP 400 Bad Request` if the Origin host is
//! missing or not allowed.
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
//! protocol aren't being checked by the middleware). The wrapped handler will only
//! be executed if the hostname in the `Origin` header matches one of the allowed
//! hosts. Requests without an `Origin` header will be rejected.
//!
//! Initialize the middleware with a vector of allowed host strings:
//!
//! ```rust
//! use iron_cors::CorsMiddleware;
//!
//! let allowed_hosts = vec!["example.com".to_string()];
//! let middleware = CorsMiddleware::with_whitelist(allowed_hosts);
//! ```
//!
//! See `examples/whitelist.rs` for a full usage example.
//!
//! ## Mode 2: Allow Any
//!
//! The user of the middleware can allow any origin header. The wrapped handler
//! will only be executed if the `Origin` header is set. The value doesn't matter.
//!
//! ```rust
//! use iron_cors::CorsMiddleware;
//!
//! let middleware = CorsMiddleware::with_allow_any(true);
//! ```
//!
//! The boolean flag specifies whether requests without an `Origin` header should
//! be rejected or not.
//!
//! See `examples/allow_any.rs` for a full usage example.

extern crate iron;
#[macro_use] extern crate log;

use iron::{Request, Response, IronResult, AroundMiddleware, Handler};
use iron::{headers, status};

/// The struct that holds the CORS configuration.
pub struct CorsMiddleware {
    allowed_hosts: Option<Vec<String>>,
    allow_invalid: bool,
}

impl CorsMiddleware {
    /// Specify which origin hosts are allowed to access the resource.
    ///
    /// Requests without an `Origin` header will be rejected.
    pub fn with_whitelist(allowed_hosts: Vec<String>) -> Self {
        CorsMiddleware {
            allowed_hosts: Some(allowed_hosts),
            allow_invalid: false,
        }
    }

    /// Allow all origins to access the resource. The
    /// `Access-Control-Allow-Origin` header of the response will be set to
    /// `*`.
    ///
    /// The `allow_invalid` parameter specifies whether requests without an
    /// `Origin` header should be accepted or not. When set to `false`,
    /// requests without that header will be answered with a HTTP 400
    /// response.
    pub fn with_allow_any(allow_invalid: bool) -> Self {
        CorsMiddleware {
            allowed_hosts: None,
            allow_invalid: allow_invalid,
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
                allow_invalid: self.allow_invalid,
            }),
        }
    }
}

/// Handler for whitelist based rules.
struct CorsHandlerWhitelist {
    handler: Box<Handler>,
    allowed_hosts: Vec<String>,
}

/// Handler if allowing any origin.
struct CorsHandlerAllowAny {
    handler: Box<Handler>,
    allow_invalid: bool,
}

/// The handler that acts as an AroundMiddleware.
///
/// It first checks an incoming request for appropriate CORS headers. If the
/// request is allowed, then process it as usual. If not, return a proper error
/// response.
impl Handler for CorsHandlerWhitelist {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // Extract origin header
        let origin = match req.headers.get::<headers::Origin>() {
            Some(origin) => origin.clone(),
            None => {
                warn!("Not a valid CORS request: Missing Origin header");
                return Ok(Response::with((status::BadRequest, "Invalid CORS request: Origin header missing")));
            }
        };

        // Verify origin header
        let may_process = self.allowed_hosts.contains(&origin.host.hostname);

        // Process request
        if may_process {
            // Everything OK, process request
            let mut res = try!(self.handler.handle(req));

            // Add Access-Control-Allow-Origin header to response
            let header = match origin.host.port {
                Some(port) => format!("{}://{}:{}", &origin.scheme, &origin.host.hostname, &port),
                None => format!("{}://{}", &origin.scheme, &origin.host.hostname),
            };
            res.headers.set(headers::AccessControlAllowOrigin::Value(header));

            Ok(res)
        } else {
            warn!("Got disallowed CORS request from {}", &origin.host.hostname);
            Ok(Response::with((status::BadRequest, "Invalid CORS request: Origin not allowed")))
        }
    }

}

/// The handler that acts as an AroundMiddleware.
///
/// It first checks an incoming request for appropriate CORS headers. If the
/// `Origin` header is present, or if invalid CORS requests are allowed, then
/// process it as usual. If not, return a proper error response.
impl Handler for CorsHandlerAllowAny {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // Extract origin header
        match req.headers.get::<headers::Origin>() {
            // If `Origin` wasn't set, abort if the user disallows invalid
            // CORS requests.
            None if !self.allow_invalid => {
                warn!("Not a valid CORS request: Missing Origin header");
                return Ok(Response::with((status::BadRequest, "Invalid CORS request: Origin header missing")));
            },
            _ => {},
        }

        // Everything OK, process request
        let mut res = try!(self.handler.handle(req));

        // Add Access-Control-Allow-Origin header to response
        res.headers.set(headers::AccessControlAllowOrigin::Value("*".into()));

        Ok(res)
    }

}
