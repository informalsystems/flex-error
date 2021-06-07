pub mod test {
    use flex_error::*;
    use thiserror::Error;
    #[derive(Debug, Error, Clone)]
    #[error("external")]
    pub struct ExternalError;

    define_error! { FooError;
      Bar
        { code: u32 }
        [ DisplayError<ExternalError> ]
        | e | { format_args!("Bar error with code {}", e.code) },
      Baz
        { extra: String }
        | e | { format_args!("General Baz error with extra detail: {}", e.extra) }
    }
}

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

    define_error! { FooError;
      Foo
        { foo_val: String }
        [ DetailOnly<PrimitiveError> ]
        | err | { format_args!("foo error: {}", err.foo_val) },
      System
        [ StdError<SystemError> ]
        | _ | { format_args!("system error") },
      Unknown
        | _ | { format_args!("unknown error") },
    }
}

pub mod bar {
    use super::foo;
    use flex_error::*;

    define_error! { BarError;
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
