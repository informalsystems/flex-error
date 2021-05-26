#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub mod macros;
mod report;
mod source;
mod tracer;
pub mod tracer_impl;

pub use report::*;
pub use source::*;
pub use tracer::*;

/// The `DefaultTracer` type alias is used when defining error types
/// using [`define_error`]. With the default Cargo features, or when
/// the `eyre_tracer` feature is set, this is configured to use the
/// [EyreTracer](tracer_impl::eyre::EyreTracer). Otherwise, it will
/// be set to [AnyhowTracer](tracer_impl::anyhow::AnyhowTracer) if
/// the `anyhow_tracer` feature is set. If neither `eyre_tracer`
/// nor `anyhow_tracer` is set, then `DefaultTracer` is set to
/// [StringTracer](tracer_impl::string::StringTracer).
///
/// We hard code globally the default error tracer to be used in
/// [`define_error`], to avoid making the error types overly generic.

// If `eyre_tracer` feature is active, it is the default error tracer
#[cfg(feature = "eyre_tracer")]
pub type DefaultTracer = tracer_impl::eyre::EyreTracer;

// Otherwise, if `eyre_tracer` feature is active, it is the default error tracer
#[cfg(all(feature = "anyhow_tracer", not(feature = "eyre_tracer")))]
pub type DefaultTracer = tracer_impl::anyhow::AnyhowTracer;

// Otherwise, if `string_tracer` feature is active, it is the default error tracer
#[cfg(all(
    not(feature = "eyre_tracer"),
    not(feature = "anyhow_tracer")
))]
pub type DefaultTracer = tracer_impl::string::StringTracer;
