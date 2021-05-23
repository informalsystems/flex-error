pub trait ErrorSource<Trace> {
  type Source;
  type Detail;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>);
}

pub type AsErrorDetail<Error, Trace> = < Error as ErrorSource<Trace> >::Detail;
pub type AsErrorSource<Error, Trace> = < Error as ErrorSource<Trace> >::Source;

#[derive(Debug)]
pub struct NoSource;

pub struct StdError<E>(E);

#[derive(Debug)]
pub struct DetailOnly<Detail>(Detail);

impl <Detail, Trace> ErrorSource<Trace> for DetailOnly<Detail> {
  type Detail = Detail;
  type Source = Detail;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>) {
    (source, None)
  }
}

impl <Trace> ErrorSource<Trace> for NoSource {
  type Detail = ();
  type Source = ();

  fn error_details(_: Self::Source) -> (Self::Detail, Option<Trace>) {
    ((), None)
  }
}
