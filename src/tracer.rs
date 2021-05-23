pub trait ErrorTracer<E>
{
  fn new_trace(err: &E) -> Self;

  fn add_trace(self, err: &E) -> Self;
}
