use std::fmt::Display;

pub trait ErrorTracer
{
  fn new_trace<E: Display>(err: &E) -> Self;

  fn add_trace<E: Display>(self, err: &E) -> Self;
}
