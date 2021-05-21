use crate::define_error;
use crate::eyre::*;

#[derive(Debug)]
pub struct PrimitiveError;

define_error!{ FooError;
  Foo(foo: String)
  [SimpleError<PrimitiveError>] =>
    | err | { format_args!("foo error {}", err.foo) },
  Unknown[NoError] =>
    | _e | { format_args!("unknown error") },
}
