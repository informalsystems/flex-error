# Changelog

## v4.0.0

Breaking changes:

- Define main error types in the form of `struct Error(ErrorDetail, Trace)` instead of
  the type alias `type Error = ErrorReport<ErrorDetail, Trace>`.

- Remove auto derive of common traits such as `Clone` and `Eq` for the main error type.
  Users can instead implement the traits explicitly since it is no longer a type alias.

- Change the error constructor convention from `error::constructor_error()` to
  `error::Error::constructor()`.

## v3.0.0

Breaking changes:

- Update structure of `ErrorReport` from `ErrorReport { detail, trace }` to `ErrorReport(detail, trace)`
  to allow simpler pattern matching.

- Auto derive common traits such as `Clone` and `Eq` for `ErrorReport`.

- Allow recursive error source using `Self`.

## v2.0.0

Breaking changes:

- Update define syntax from `define_error!{ Error; ... }` to `define_error!{ Error { ... } }`

- Allow custom derive attributes in the form of `define_error!{ #[derive(Debug, ...)] Error { ... } }`.

## v1.0.0

- Initial draft release for flex-error.
