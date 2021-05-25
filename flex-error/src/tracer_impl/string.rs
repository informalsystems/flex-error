use alloc::string::String;
use core::fmt::{Display, Debug, Formatter};
use crate::tracer::{ErrorTracer, ErrorMessageTracer};

pub struct StringTracer(pub String);

impl ErrorMessageTracer for StringTracer
{
  fn new_message<E: Display>(err: &E) -> Self {
    StringTracer(alloc::format!("{}", err))
  }

  fn add_message<E: Display>(self, err: &E) -> Self {
    StringTracer(
      alloc::format!("{0}: {1}", err, self.0)
    )
  }
}

impl <E: Display> ErrorTracer<E> for StringTracer
{
  fn new_trace(err: &E) -> Self {
    StringTracer(alloc::format!("{}", err))
  }

  fn add_trace(self, err: &E) -> Self {
    StringTracer(
      alloc::format!("{0}: {1}", err, self.0)
    )
  }
}

impl Debug for StringTracer
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "StringTracer: {0}", self.0)
  }
}

impl Display for StringTracer
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{0}", self.0)
  }
}
