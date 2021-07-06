use alloc::{boxed::Box, format, string::String};
use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::fmt::{Debug, Display, Formatter};

use super::source::{BoxDetail, ErrorSource};
use super::tracer::ErrorMessageTracer;

/// An [`ErrorSource`] that provides both the error
/// detail and error trace separately. The error trace in an error report
/// also already contains the trace of the current error detail already.
/// so new errors that arise from an `ErrorReport` only need to access
/// the `trace` object to add new traces to it.
///
/// `ErrorReport` should be used for all application code that uses `flex-error`.
/// When defining new error types using [`define_error!`], the error name is defined
/// as a type alias to `ErrorReport`.
pub struct ErrorReport<Detail, Trace>(pub Detail, pub Trace);

impl<Detail, Trace> ErrorSource<Trace> for ErrorReport<Detail, Trace> {
    type Source = Self;
    type Detail = Detail;

    fn error_details(ErrorReport(detail, trace): Self) -> (Detail, Option<Trace>) {
        (detail, Some(trace))
    }
}

impl<Detail, Trace> ErrorSource<Trace> for BoxDetail<Detail> {
    type Source = ErrorReport<Detail, Trace>;
    type Detail = Box<Detail>;

    fn error_details(ErrorReport(detail, trace): Self::Source) -> (Self::Detail, Option<Trace>) {
        (Box::new(detail), Some(trace))
    }
}

impl<Detail, Trace> ErrorReport<Detail, Trace> {
    pub fn new(detail: Detail, trace: Trace) -> Self {
        ErrorReport(detail, trace)
    }

    pub fn detail(&self) -> &Detail {
        &self.0
    }

    pub fn trace(&self) -> &Trace {
        &self.1
    }

    pub fn add_trace<E: Display>(self, message: &E) -> Self
    where
        Trace: ErrorMessageTracer,
    {
        let detail = self.0;
        let trace = self.1.add_message(message);
        ErrorReport(detail, trace)
    }

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
                ErrorReport(detail2, trace2)
            }
            None => {
                let trace2 = Trace::new_message(&detail2);
                ErrorReport(detail2, trace2)
            }
        }
    }
}

impl<Detail, Trace> Debug for ErrorReport<Detail, Trace>
where
    Trace: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.trace().fmt(f)
    }
}

impl<Detail, Trace> Display for ErrorReport<Detail, Trace>
where
    Trace: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.trace().fmt(f)
    }
}

impl<Detail, Trace> Clone for ErrorReport<Detail, Trace>
where
    Detail: Clone,
    Trace: Display + ErrorMessageTracer,
{
    fn clone(&self) -> Self {
        ErrorReport(self.0.clone(), Trace::new_message(&self.1))
    }
}

impl<Detail, Trace> PartialEq for ErrorReport<Detail, Trace>
where
    Detail: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<Detail, Trace> Eq for ErrorReport<Detail, Trace> where Detail: Eq {}

impl<Detail, Trace> PartialOrd for ErrorReport<Detail, Trace>
where
    Detail: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Detail, Trace> Ord for ErrorReport<Detail, Trace>
where
    Detail: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
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
        self.trace().as_error()
    }
}

#[cfg(feature = "serde")]
impl<Detail, Trace> serde::Serialize for ErrorReport<Detail, Trace>
where
    Detail: serde::Serialize,
    Trace: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.0, format!("{}", self.1)).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, Detail, Trace> serde::Deserialize<'de> for ErrorReport<Detail, Trace>
where
    Detail: serde::Deserialize<'de>,
    Trace: ErrorMessageTracer,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (detail, message) = <(Detail, String)>::deserialize(deserializer)?;
        Ok(ErrorReport::new(detail, Trace::new_message(&message)))
    }
}
