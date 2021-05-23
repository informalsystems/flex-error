use super::source::ErrorSource;
use super::tracer::ErrorTracer;

pub struct ErrorReport<Detail, Trace> {
  pub detail: Detail,
  pub trace: Trace,
}

impl <Detail, Trace> ErrorSource<Trace> for ErrorReport<Detail, Trace> {
  type Source = Self;
  type Detail = Detail;

  fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>) {
    (source.detail, Some(source.trace))
  }
}

pub fn trace_error
  <Error, Source, Detail1, Detail2, Trace>
  ( source: Source,
    cont: impl FnOnce(Detail1) -> Detail2
  ) ->
    ErrorReport<Detail2, Trace>
where
  Error: ErrorSource<Trace, Source=Source, Detail=Detail1>,
  Detail2: Clone,
  Trace: ErrorTracer<Detail2>,
{
  let (detail1, m_trace1) = Error::error_details(source);
  let detail2 = cont(detail1);
  match m_trace1 {
    Some(trace1) => {
      let trace2 = trace1.add_trace(&detail2);
      ErrorReport { detail: detail2, trace: trace2 }
    }
    None => {
      let trace2 = Trace::new_trace(&detail2);
      ErrorReport { detail: detail2, trace: trace2 }
    }
  }

}
