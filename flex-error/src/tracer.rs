use core::fmt::Display;
use super::source::{ErrorSource, TraceOnly, DisplayError};

pub trait ErrorMessageTracer {
  fn new_message<E: Display>(message: &E) -> Self;

  fn add_message<E: Display>(self, message: &E) -> Self;

  #[cfg(feature = "std")]
  fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)>;
}

pub trait ErrorTracer<E>: ErrorMessageTracer
{
  fn new_trace(err: E) -> Self;

  fn add_trace(self, err: E) -> Self;
}

impl <E, Tracer> ErrorSource<Tracer>
  for DisplayError<E>
where
  E: Display,
  Tracer: ErrorMessageTracer,
{
  type Detail = E;
  type Source = E;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
    let trace = Tracer::new_message(&source);
    (source, Some(trace))
  }
}

impl <E, Tracer> ErrorSource<Tracer>
  for TraceOnly<E>
where
  Tracer: ErrorTracer<E>,
{
  type Detail = ();
  type Source = E;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
    let trace = Tracer::new_trace(source);
    ((), Some(trace))
  }
}
