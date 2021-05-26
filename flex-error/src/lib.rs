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

// If `eyre_tracer` feature is active, it is the default error tracer
#[cfg(feature = "eyre_tracer")]
pub type DefaultTracer = tracer_impl::eyre::EyreTracer;

// Otherwise, if `eyre_tracer` feature is active, it is the default error tracer
#[cfg(all(feature = "anyhow_tracer", not(feature = "eyre_tracer")))]
pub type DefaultTracer = tracer_impl::anyhow::AnyhowTracer;

// Otherwise, if `string_tracer` feature is active, it is the default error tracer
#[cfg(all(
    feature = "string_tracer",
    not(feature = "eyre_tracer"),
    not(feature = "anyhow_tracer")
))]
pub type DefaultTracer = tracer_impl::string::StringTracer;
