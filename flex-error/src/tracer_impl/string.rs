use crate::tracer::{ErrorMessageTracer, ErrorTracer};
use alloc::string::String;
use core::fmt::{Debug, Display, Formatter};

/// A naive string tracer serializes error messages into
/// string and simply concatenate them together.
/// This can be used for example in `no_std` environment,
/// which may not support more complex error tracers.
pub struct StringTracer(pub String);

impl ErrorMessageTracer for StringTracer {
    fn new_message<E: Display>(err: &E) -> Self {
        StringTracer(alloc::format!("{}", err))
    }

    fn add_message<E: Display>(self, err: &E) -> Self {
        StringTracer(alloc::format!("{0}: {1}", err, self.0))
    }

    #[cfg(feature = "std")]
    fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<E: Display> ErrorTracer<E> for StringTracer {
    fn new_trace(err: E) -> Self {
        StringTracer(alloc::format!("{}", err))
    }

    fn add_trace(self, err: E) -> Self {
        StringTracer(alloc::format!("{0}: {1}", err, self.0))
    }
}

impl Debug for StringTracer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "StringTracer: {0}", self.0)
    }
}

impl Display for StringTracer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{0}", self.0)
    }
}
