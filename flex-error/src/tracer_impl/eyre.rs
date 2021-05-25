use eyre;
use core::fmt::Display;
use crate::tracer::{ErrorTracer, ErrorMessageTracer};

pub type EyreTracer = eyre::Report;

impl ErrorMessageTracer for EyreTracer
{
  fn new_message<E: Display>(err: &E) -> Self {
    let message = alloc::format!("{}", err);
    EyreTracer::msg(message)
  }

  fn add_message<E: Display>(self, err: &E) -> Self {
    let message = alloc::format!("{}", err);
    self.wrap_err(message)
  }
}

impl <E> ErrorTracer<E> for EyreTracer
where
  E: Clone + std::error::Error + Send + Sync + 'static,
{
  fn new_trace(err: &E) -> Self {
    EyreTracer::new(err.clone())
  }

  fn add_trace(self, err: &E) -> Self {
    let message = alloc::format!("{}", err);
    self.wrap_err(message)
  }
}
