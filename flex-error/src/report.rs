use alloc::boxed::Box;
use super::source::{ErrorSource, BoxDetail};
use super::tracer::ErrorMessageTracer;
use core::fmt::{Debug, Display, Formatter};

/// An [`ErrorSource`] that provides both the error
/// detail and error trace separately. The error trace in an error report
/// also already contains the trace of the current error detail already.
/// so new errors that arise from an `ErrorReport` only need to access
/// the `trace` object to add new traces to it.
///
/// `ErrorReport` should be used for all application code that uses `flex-error`.
/// When defining new error types using [`define_error!`], the error name is defined
/// as a type alias to `ErrorReport`.
pub struct ErrorReport<Detail, Trace> {
    pub detail: Detail,
    pub trace: Trace,
}

impl<Detail, Trace> ErrorSource<Trace> for ErrorReport<Detail, Trace> {
    type Source = Self;
    type Detail = Detail;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>) {
        (source.detail, Some(source.trace))
    }
}

impl <Detail, Trace> ErrorSource<Trace>
    for BoxDetail<Detail>
{
    type Source = ErrorReport<Detail, Trace>;
    type Detail = Box<Detail>;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>) {
        (Box::new(source.detail), Some(source.trace))
    }
}

impl<Detail, Trace> ErrorReport<Detail, Trace> {
    pub fn trace_from<E, Cont>(source: E::Source, cont: Cont) -> Self
    where
        Detail: Display,
        E: ErrorSource<Trace>,
        Trace: ErrorMessageTracer,
        Cont: FnOnce(E::Detail) -> Detail,
    {
        let (detail1, m_trace1) = E::error_details(source);
        let detail2 = cont(detail1);
        match m_trace1 {
            Some(trace1) => {
                let trace2 = trace1.add_message(&detail2);
                ErrorReport {
                    detail: detail2,
                    trace: trace2,
                }
            }
            None => {
                let trace2 = Trace::new_message(&detail2);
                ErrorReport {
                    detail: detail2,
                    trace: trace2,
                }
            }
        }
    }
}

impl<Detail, Trace> Debug for ErrorReport<Detail, Trace>
where
    Trace: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.trace.fmt(f)
    }
}

impl<Detail, Trace> Display for ErrorReport<Detail, Trace>
where
    Trace: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.trace.fmt(f)
    }
}

#[cfg(feature = "std")]
impl<Detail, Trace> std::error::Error for ErrorReport<Detail, Trace>
where
    Detail: Display,
    Trace: Debug + Display,
    Trace: ErrorMessageTracer,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.trace.as_error()
    }
}
