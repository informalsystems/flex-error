use crate::tracer::{ErrorMessageTracer, ErrorTracer};
use core::fmt::{Debug, Display};

/// Type alias to [`anyhow::Error`]
pub type AnyhowTracer = anyhow::Error;

impl ErrorMessageTracer for AnyhowTracer {
    fn new_message<E: Display>(err: &E) -> Self {
        let message = alloc::format!("{}", err);
        AnyhowTracer::msg(message)
    }

    fn add_message<E: Display>(self, err: &E) -> Self {
        let message = alloc::format!("{}", err);
        self.context(message)
    }

    #[cfg(feature = "std")]
    fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use core::ops::Deref;
        Some(self.deref())
    }
}

impl<E> ErrorTracer<E> for AnyhowTracer
where
    E: Display + Debug + Send + Sync + 'static,
{
    fn new_trace(err: E) -> Self {
        AnyhowTracer::msg(err)
    }

    fn add_trace(self, err: E) -> Self {
        let message = alloc::format!("{}", err);
        self.context(message)
    }
}
