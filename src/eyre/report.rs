use eyre;
use std::ops::Deref;
use std::fmt::{Display, Formatter, Debug};

pub use eyre::Report as EyreReport;

pub trait TraceError<E> {
  fn trace_error(self, err: E, msg: String) -> Self;
}

pub struct SimpleTrace(pub String);

pub type ErrorTrace = eyre::Report;

pub trait ErrorSource {
  type Source;
  type Detail;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<EyreReport>);
}

pub type AsErrorDetail<Error> = < Error as ErrorSource >::Detail;
pub type AsErrorSource<Error> = < Error as ErrorSource >::Source;

#[derive(Debug)]
pub struct Report<Detail> {
  pub detail: Detail,
  pub report: EyreReport,
}

#[derive(Debug)]
pub struct SimpleError<Detail>(Detail);

#[derive(Debug)]
pub struct StdError<Detail>(Detail);

#[derive(Debug)]
pub struct NoError;

impl <Detail> ErrorSource for Report<Detail> {
  type Detail = Detail;
  type Source = Self;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<EyreReport>) {
    (source.detail, Some(source.report))
  }
}

pub fn error_details<E: ErrorSource>
  (source: E::Source)
  -> (E::Detail, Option<EyreReport>)
{
  E::error_details(source)
}

impl <Detail> ErrorSource for SimpleError<Detail> {
  type Detail = Detail;
  type Source = Detail;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<EyreReport>) {
    (source, None)
  }
}

impl <Detail> ErrorSource for StdError<Detail>
where
  Detail: std::error::Error + Clone + Send + Sync + 'static,
{
  type Detail = Detail;
  type Source = Detail;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<EyreReport>) {
    (source.clone(), Some(EyreReport::new(source)))
  }
}

impl ErrorSource for NoError {
  type Detail = ();
  type Source = ();

  fn error_details(_: Self::Source) -> (Self::Detail, Option<EyreReport>) {
    ((), None)
  }
}

impl <Detail> Display for Report<Detail>
where
  Detail: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{0}", self.report)
  }
}

impl <Detail> std::error::Error for Report<Detail>
where
  Detail: Debug + Display
{
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(self.report.deref())
  }
}
