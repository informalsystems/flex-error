use core::fmt::Display;
use super::source::{ErrorSource, StdError};

pub trait ErrorMessageTracer {
  fn new_message<E: Display>(message: &E) -> Self;

  fn add_message<E: Display>(self, message: &E) -> Self;
}

pub trait ErrorTracer<E>: ErrorMessageTracer
{
  fn new_trace(err: &E) -> Self;

  fn add_trace(self, err: &E) -> Self;
}

impl <E, Tracer> ErrorSource<Tracer>
  for StdError<E>
where
  Tracer: ErrorTracer<E>,
{
  type Detail = E;
  type Source = E;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
    let trace = Tracer::new_trace(&source);
    (source, Some(trace))
  }
}
