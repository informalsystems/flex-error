use error_macros::tests::{foo, bar};

fn main() -> Result<(), bar::BarError> {
  color_eyre::install().unwrap();

  let err1 = foo::system_error(foo::SystemError::Error1);
  let err2 = bar::foo_error("Foo has failed".into(), err1);
  Err(err2)
}
