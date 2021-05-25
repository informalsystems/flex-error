use anyhow;
use core::fmt::Display;
use crate::tracer::{ErrorTracer, ErrorMessageTracer};

pub type AnyhowTracer = anyhow::Error;

impl ErrorMessageTracer for AnyhowTracer
{
  fn new_message<E: Display>(err: &E) -> Self {
    let message = alloc::format!("{}", err);
    AnyhowTracer::msg(message)
  }

  fn add_message<E: Display>(self, err: &E) -> Self {
    let message = alloc::format!("{}", err);
    self.context(message)
  }
}

impl <E> ErrorTracer<E> for AnyhowTracer
where
  E: Clone + std::error::Error + Send + Sync + 'static,
{
  fn new_trace(err: &E) -> Self {
    AnyhowTracer::new(err.clone())
  }

  fn add_trace(self, err: &E) -> Self {
    let message = alloc::format!("{}", err);
    self.context(message)
  }
}
