
pub mod foo {
  use flex_error::*;

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
      [ DetailOnly<PrimitiveError> ]
      | err | { format_args!("foo error: {}", err.foo) },
    System
      [ StdError<SystemError> ]
      | _ | { format_args!("system error") },
    Unknown
      | _ | { format_args!("unknown error") },
  }
}

pub mod bar {
  use flex_error::*;
  use super::foo;

  define_error!{ BarError;
    Bar
      { bar: String }
      | err | { format_args!("bar error {}", err.bar) },
    Foo
      { detail: String }
      [ foo::FooError ]
      | err | { format_args!("error caused by foo: {}", err.detail) },
  }
}

fn main() -> Result<(), bar::BarError> {
  color_eyre::install().unwrap();

  let err1 = foo::system_error(foo::SystemError::Error1);
  let err2 = bar::foo_error("Foo has failed".into(), err1);
  Err(err2)
}
