use crate::tracer::{ErrorMessageTracer, ErrorTracer};
use core::fmt::Display;
use eyre;

pub type EyreTracer = eyre::Report;

impl ErrorMessageTracer for EyreTracer {
    fn new_message<E: Display>(err: &E) -> Self {
        let message = alloc::format!("{}", err);
        EyreTracer::msg(message)
    }

    fn add_message<E: Display>(self, err: &E) -> Self {
        let message = alloc::format!("{}", err);
        self.wrap_err(message)
    }

    #[cfg(feature = "std")]
    fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use core::ops::Deref;
        Some(self.deref())
    }
}

impl<E> ErrorTracer<E> for EyreTracer
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn new_trace(err: E) -> Self {
        EyreTracer::new(err)
    }

    fn add_trace(self, err: E) -> Self {
        let message = alloc::format!("{}", err);
        self.wrap_err(message)
    }
}
