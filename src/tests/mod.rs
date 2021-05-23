
pub mod foo {
  use crate::*;

  use thiserror::Error;

  #[derive(Debug)]
  pub struct PrimitiveError;

  #[derive(Debug, Error, Clone)]
  pub enum SystemError {
    #[error("error1")]
    Error1,
    #[error("error2")]
    Error2,
  }

  define_error!{ FooError;
    Foo
      { foo: String }
      [ DetailOnly<PrimitiveError> ] =>
      | err | { format_args!("foo error: {}", err.foo) },
    System
      [ StdError<SystemError> ] =>
      | _ | { format_args!("system error") },
    Unknown[NoSource] =>
      | _ | { format_args!("unknown error") },
  }
}

pub mod bar {
  use crate::*;
  use super::foo;

  define_error!{ BarError;
    Bar
      { bar: String }
      [ NoSource ] =>
      | err | { format_args!("bar error {}", err.bar) },
    Foo
      { detail: String }
      [ foo::FooError ] =>
      | err | { format_args!("error caused by foo: {}", err.detail) },
  }
}

#[test]
fn test() {
  color_eyre::install().unwrap();
  {
    let err = foo::foo_error("No Foo".into(), foo::PrimitiveError);
    println!("Error: {:?}", err.trace);
  }
  {
    let err = foo::system_error(foo::SystemError::Error1);
    println!("Error: {:?}", err.trace);
  }
  {
    let err = foo::unknown_error();
    println!("Error: {:?}", err.trace);
  }
  {
    let err1 = foo::foo_error("Hello Foo".into(), foo::PrimitiveError);
    let err2 = bar::foo_error("Foo has failed".into(), err1);
    println!("Error: {:?}", err2.trace);
  }
}
