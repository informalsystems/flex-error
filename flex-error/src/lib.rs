#![no_std]

/*!
`flex-error` is a lightweight Rust library that uses macros and traits
to switch between different error tracing implementations and no_std. The library currently supports 3 modes via Cargo feature flags: `eyre_tracer` (default), `anyhow_tracer`, and `string_tracer` (no_std).

The library separates out several concepts as traits:
`ErrorDetail`, [`ErrorTracer`], and [`ErrorSource`].

  - `ErrorDetail` is responsible to structured metadata information about a specific error.

  - `ErrorTracer` is responsible for tracing error chains and backtraces.

  - `ErrorSource` allows generic conversion of external error types into an ErrorDetail with optional ErrorTrace.

With the separation of concerns, `flex-error` allows applications to easily
switch between different error reporting implementations,
such as [`eyre`] and [`anyhow`], by implementing
[`ErrorTracer`] for the respective reporters.

`flex-error` defines a [`define_error!`] macro that define custom `Detail`
types and error types implementing `ErrorSource<DefaultTracer>`.
The [`DefaultTracer`] type is set globally by the feature flag, so that
application error types do not have to be over-generalized.
The trade off is that it is not possible to use multiple
[`ErrorTracer`] implementations at the same time across different crates that
use `flex-error`.

!*/

#[cfg(feature = "std")]
extern crate std;

pub extern crate alloc;

#[cfg(feature = "std")]
pub use std::error::Error as StdError;

pub mod macros;
mod source;
mod tracer;
pub mod tracer_impl;

pub use source::*;
pub use tracer::*;

/// The `DefaultTracer` type alias is used when defining error types
/// using [`define_error!`]. With the default Cargo features, or when
/// the `eyre_tracer` feature is set, this is configured to use the
/// [EyreTracer](tracer_impl::eyre::EyreTracer). Otherwise, it will
/// be set to [AnyhowTracer](tracer_impl::anyhow::AnyhowTracer) if
/// the `anyhow_tracer` feature is set. If neither `eyre_tracer`
/// nor `anyhow_tracer` is set, then `DefaultTracer` is set to
/// [StringTracer](tracer_impl::string::StringTracer).
///
/// We hard code globally the default error tracer to be used in
/// [`define_error!`], to avoid making the error types overly generic.

// If `eyre_tracer` feature is active, it is the default error tracer
#[cfg(feature = "eyre_tracer")]
pub type DefaultTracer = tracer_impl::eyre::EyreTracer;

// Otherwise, if `eyre_tracer` feature is active, it is the default error tracer
#[cfg(all(feature = "anyhow_tracer", not(feature = "eyre_tracer")))]
pub type DefaultTracer = tracer_impl::anyhow::AnyhowTracer;

// Otherwise, if `string_tracer` feature is active, it is the default error tracer
#[cfg(all(not(feature = "eyre_tracer"), not(feature = "anyhow_tracer")))]
pub type DefaultTracer = tracer_impl::string::StringTracer;
