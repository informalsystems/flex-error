use std::fmt::{Display, Debug, Formatter};
use crate::tracer::ErrorTracer;
use crate::source::{ErrorSource, StdError};

#[derive(Debug)]
pub struct StringTracer(pub String);

impl <E> ErrorSource<StringTracer> for StdError<E>
where
  E: Display,
{
  type Detail = E;
  type Source = E;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<StringTracer>) {
    let trace = StringTracer(format!("{}", source));
    (source, Some(trace))
  }
}

impl ErrorTracer for StringTracer
{
  fn new_trace<E: Display>(err: &E) -> Self {
    StringTracer(format!("{}", err))
  }

  fn add_trace<E: Display>(self, err: &E) -> Self {
    StringTracer(
      format!("{0}: {1}", err, self.0)
    )
  }
}

impl Display for StringTracer
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{0}", self.0)
  }
}
