mod tracer;
mod source;
mod report;
pub mod macros;
pub mod tracer_impl;

pub use tracer::*;
pub use source::*;
pub use report::*;

#[cfg(feature = "anyhow_tracer")]
pub type DefaultTracer = tracer_impl::anyhow::AnyhowTracer;

#[cfg(feature = "eyre_tracer")]
pub type DefaultTracer = tracer_impl::eyre::EyreTracer;

#[cfg(feature = "string_tracer")]
pub type DefaultTracer = tracer_impl::string::StringTracer;
