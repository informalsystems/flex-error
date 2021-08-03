# Changelog

## v0.4.2

- Make the `Display` implementation of error types use `Debug` to display error traces
  from `eyre` and other error tracers.

## v0.4.1

Breaking changes:

- The `impl` definition for `std::error::Error` is now generated based on whether
  the `std` feature is enabled on the `flex-error` crate, instead of the user
  crate that calls `define_error`.

## v0.4.0

Breaking changes:

- Define main error types in the form of `struct Error(ErrorDetail, Trace)` instead of
  the type alias `type Error = ErrorReport<ErrorDetail, Trace>`.

- Remove auto derive of common traits such as `Clone` and `Eq` for the main error type.
  Users can instead implement the traits explicitly since it is no longer a type alias.

- Change the error constructor convention from `error::constructor_error()` to
  `error::Error::constructor()`.

- Allow multiple attributes to be given to main and sub errors in `define_error!`.

## v0.3.0

Breaking changes:

- Update structure of `ErrorReport` from `ErrorReport { detail, trace }` to `ErrorReport(detail, trace)`
  to allow simpler pattern matching.

- Auto derive common traits such as `Clone` and `Eq` for `ErrorReport`.

- Allow recursive error source using `Self`.

## v0.2.0

Breaking changes:

- Update define syntax from `define_error!{ Error; ... }` to `define_error!{ Error { ... } }`

- Allow custom derive attributes in the form of `define_error!{ #[derive(Debug, ...)] Error { ... } }`.

## v0.1.0

- Initial draft release for flex-error.
