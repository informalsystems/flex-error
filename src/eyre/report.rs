use eyre;
use std::ops::Deref;
use std::fmt::{Display, Formatter, Debug};

pub use eyre::Report as EyreReport;

pub trait IsErrorReport {
  type Detail;

  fn to_error_report(self) -> (Self::Detail, EyreReport);
}

#[derive(Debug)]
pub struct Report<Detail> {
  pub detail: Detail,
  pub report: EyreReport,
}

pub type ReportDetail<Report> = < Report as IsErrorReport >::Detail;

impl <Detail> IsErrorReport for Report<Detail> {
  type Detail = Detail;

  fn to_error_report(self) -> (Self::Detail, EyreReport) {
    (self.detail, self.report)
  }
}

impl <Detail> Report<Detail> {
  pub fn new(detail: Detail, message: &str) -> Self
  {
    Report {
      detail,
      report: eyre::eyre!("{}", message)
    }
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
