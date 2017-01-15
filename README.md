# CORS Middleware for Iron

[![Travis CI][travis-ci-badge]][travis-ci]
[![Crates.io][crates-io-badge]][crates-io]

A CORS Middleware for [Iron](http://ironframework.io/).

See https://www.html5rocks.com/static/images/cors_server_flowchart.png for
reference.

The middleware will return `HTTP 400 Bad Request` if the origin host is missing
or not allowed.

Preflight requests are not yet supported.


## Usage

There are two modes available:

### Whitelist

The user of the middleware must specify a list of allowed hosts (port or
protocol aren't being checked by the middleware). The wrapped handler will only
be executed if the hostname in the `Origin` header matches one of the allowed
hosts.

Initialize the middleware with a vector of allowed host strings:

```rust
extern crate iron_cors;

use iron_cors::CorsMiddleware;

let allowed_hosts = vec!["example.com".to_string()];
let middleware = CorsMiddleware::with_whitelist(allowed_hosts);
```

See `examples/whitelist.rs` for a full usage example.

### Allow Any

The user of the middleware can allow any origin header. The wrapped handler
will only be executed if the `Origin` header is set. The value doesn't matter.

```rust
extern crate iron_cors;

use iron_cors::CorsMiddleware;

let middleware = CorsMiddleware::with_allow_any();
```

See `examples/allow_any.rs` for a full usage example.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


<!-- Badges -->
[travis-ci]: https://travis-ci.org/dbrgn/iron-cors-rs
[travis-ci-badge]: https://img.shields.io/travis/dbrgn/iron-cors-rs.svg
[crates-io]: https://crates.io/crates/iron-cors
[crates-io-badge]: https://img.shields.io/crates/v/iron-cors.svg
