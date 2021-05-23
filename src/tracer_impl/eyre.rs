use eyre;
use std::fmt::Display;
use crate::tracer::{ErrorTracer};
use crate::source::{ErrorSource, StdError};

pub type EyreTracer = eyre::Report;

impl <E> ErrorSource<EyreTracer>
  for StdError<E>
where
  E: std::error::Error + Clone + Send + Sync + 'static,
{
  type Detail = E;
  type Source = E;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<EyreTracer>) {
    let trace = EyreTracer::new(source.clone());
    (source, Some(trace))
  }
}

impl <E> ErrorTracer<E> for EyreTracer
where
  E: Display
{
  fn new_trace(err: &E) -> Self {
    let message = format!("{}", err);
    EyreTracer::msg(message)
  }

  fn add_trace(self, err: &E) -> Self {
    let message = format!("{}", err);
    self.wrap_err(message)
  }
}
