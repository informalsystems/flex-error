mod tracer;
mod source;
mod report;
mod macros;
pub mod tracer_impl;

pub use tracer::*;
pub use source::*;
pub use report::*;

// pub type DefaultTracer = tracer_impl::anyhow::AnyhowTracer;
pub type DefaultTracer = tracer_impl::eyre::EyreTracer;
// pub type DefaultTracer = tracer_impl::string::StringTracer;

pub mod tests;
