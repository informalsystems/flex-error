use anyhow;
use std::fmt::Display;
use crate::tracer::{ErrorTracer};
use crate::source::{ErrorSource, StdError};

pub type AnyhowTracer = anyhow::Error;

impl <E> ErrorSource<AnyhowTracer>
  for StdError<E>
where
  E: std::error::Error + Clone + Send + Sync + 'static,
{
  type Detail = E;
  type Source = E;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<AnyhowTracer>) {
    let trace = AnyhowTracer::new(source.clone());
    (source, Some(trace))
  }
}

impl ErrorTracer for AnyhowTracer
{
  fn new_trace<E: Display>(err: &E) -> Self {
    let message = format!("{}", err);
    AnyhowTracer::msg(message)
  }

  fn add_trace<E: Display>(self, err: &E) -> Self {
    let message = format!("{}", err);
    self.context(message)
  }
}
