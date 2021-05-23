use crate::*;

#[derive(Debug)]
pub struct PrimitiveError;

define_error!{ FooError;
  Foo(foo: String)
  [DetailOnly<PrimitiveError>] =>
    | err | { format_args!("foo error {}", err.foo) },
  Unknown[NoSource] =>
    | _e | { format_args!("unknown error") },
}
