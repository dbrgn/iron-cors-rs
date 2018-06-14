# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.


### v0.8.0 (2018-06-14)

This release raises the min supported Rust version to 1.21, due to a patch
release of a transitive dependency that does not work on 1.17 anymore.

No functional changes.

### v0.7.1 (2018-06-12)

This is a documentation-only update.

### v0.7.0 (2018-01-05)

- [changed] Update log dependency to 0.4
- [changed] Update iron dependency to 0.6

Note that the version 0.6.0 has been skipped due to the potentially breaking
iron version upgrade.

### v0.6.0-rc.1 (2017-08-29)

- [changed] Require at least Rust 1.17
- [added] Implement support for preflight requests (#9, thanks @DavidBM!)

### v0.5.1 (2017-03-22)

- [fixed] Headers are now added to response even if handler returns an error (#4)

### v0.5.0 (2017-03-21)

- [changed] Upgrade iron to 0.5, minimal version of Rust required is now 0.11

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
