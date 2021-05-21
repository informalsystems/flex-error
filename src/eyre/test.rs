use crate::define_error;

define_error!{ FooError;
  Foo(foo: String)[basic[()]] =>
    | err | { format_args!("foo error {}", err.foo) }
}
