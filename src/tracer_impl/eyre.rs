use eyre;
use crate::source::{ErrorSource, StdError};

type EyreTracer = eyre::Report;

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
