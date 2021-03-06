pub mod test {
    use flex_error::*;
    use thiserror::Error;
    #[derive(Debug, Error, Eq, PartialEq, Clone)]
    #[error("external")]
    pub struct ExternalError;

    define_error! {
      /// This is documentation for foo error

      #[derive(Debug, Clone)]
      #[derive(Eq, PartialEq)]
      FooError {
        /// This is documentation for bar error
        Bar
          { code: u32 }
          [ DisplayError<ExternalError> ]
          | e | { format_args!("Bar error with code {}", e.code) },

        /// This is documentation for baz error
        #[derive(Ord, PartialOrd)]
        Baz
          { extra: String }
          | e | { format_args!("General Baz error with extra detail: {}", e.extra) },
      }
    }
}

pub mod foo {
    use flex_error::*;

    use thiserror::Error;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct PrimitiveError;

    #[derive(Debug, Clone, Error, PartialEq)]
    pub enum SystemError {
        #[error("error1")]
        Error1,
        #[error("error2")]
        Error2,
    }

    define_error! {
      #[derive(Debug, Clone, PartialEq, Eq)]
      FooError {
        Foo
          { foo_val: String }
          [ DetailOnly<PrimitiveError> ]
          | err | { format_args!("foo error: {}", err.foo_val) },
        System
          [ TraceError<SystemError> ]
          | _ | { "system error" },
        Unknown
          | _ | { "unknown error" },

        Nested
          [ Self ]
          | _ | { format_args!("nested foo error") }
      }
    }
}

pub mod bar {
    use super::foo;
    use flex_error::*;

    define_error! {
      #[derive(Debug, Clone, PartialEq, Eq)]
      BarError {
        Bar
          { bar: String }
          | err | { format_args!("bar error {}", err.bar) },
        Foo
          { detail: String }
          [ foo::FooError ]
          | err | { format_args!("error caused by foo: {}", err.detail) },
      }
    }
}

fn main() -> Result<(), bar::BarError> {
    color_eyre::install().unwrap();

    let err1 = foo::FooError::system(foo::SystemError::Error1);
    let err2 = foo::FooError::nested(err1);
    let err3 = bar::BarError::foo("Foo has failed".into(), err2);

    println!("error: {:?}", err3);

    // Err(err3)
    Ok(())
}
