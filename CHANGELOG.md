# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.


### v0.4.0 (2017-01-16)

- [changed] The whitelist is now initialized with a `HashSet` instead of a `Vec`
- [changed] Less allocations

### v0.3.0 (2017-01-15)

- [added] Make it possible to accept invalid CORS requests

### v0.2.0 (2017-01-15)

- [added] Add `CorsMiddleware::allow_any` constructor
- [changed] Renamed `CorsMiddleware::new` constructor to `CorsMiddleware::with_whitelist`

### v0.1.0 (2017-01-14)

- First crates.io release
