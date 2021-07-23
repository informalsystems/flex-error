use core::fmt::Display;

/// An `ErrorMessageTracer` can be used to generically trace
/// any error detail that implements [`Display`](std::fmt::Display).
///
/// The error tracer may add backtrace information when the tracing
/// methods are called. However since the error detail is required
/// to only implement `Display`, any existing error trace may be
/// lost even if the error detail implements `Error` and contains
/// backtrace, unless the backtrace is serialized in `Display`.
pub trait ErrorMessageTracer {
    /// Creates a new error trace, starting from a source error
    /// detail that implements [`Display`](std::fmt::Display).
    fn new_message<E: Display>(message: &E) -> Self;

    /// Adds new error detail to an existing trace.
    fn add_message<E: Display>(self, message: &E) -> Self;

    /// If the `std` feature is enabled, the error tracer
    /// also provides method to optionally converts itself
    /// to a `dyn` [`Error`](std::error::Error).
    #[cfg(feature = "std")]
    fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)>;
}

/// An error tracer implements `ErrorTracer<E>` if it supports
/// more sophisticated error tracing for an error type `E`.
/// The contraint for `E` depends on the specific error tracer
/// implementation.
///
/// For example, [`EyreTracer`](crate::tracer_impl::eyre::EyreTracer)
/// and [`AnyhowTracer`](crate::tracer_impl::anyhow::AnyhowTracer) requires
/// an error type to satisfy `E: Error + Send + Sync + 'static`.
///
/// The error tracer also requires ownership of the source error to be
/// transferred to the error tracer. Because of this, it may not be possible
/// to extract a source error type to be used as both error detail and
/// error trace. We also should not expect `E` to implement `Clone`, as
/// error types such as [`eyre::Report`] do not implement `Clone`.
pub trait ErrorTracer<E>: ErrorMessageTracer {
    /// Create a new error trace from `E`, also taking ownership of it.
    ///
    /// This calls the underlying methods such as [`eyre::Report::new`]
    /// and [`anyhow::Error::new`].
    fn new_trace(err: E) -> Self;

    /// Add a new error trace from `E`. In the current underlying implementation,
    /// this is effectively still has the same behavior as
    /// [`ErrorMessageTracer::add_message`]. This is because [`eyre`] and
    /// [`anyhow`] do not support adding new set of backtraces to an existing
    /// trace. So effectively, currently the error tracers can track at most
    /// one backtrace coming from the original error source.
    fn add_trace(self, err: E) -> Self;
}
