use core::fmt::Display;
use core::marker::PhantomData;

use crate::tracer::{ErrorMessageTracer, ErrorTracer};

/**
 A type implementing `ErrorSource<Trace>` is a proxy type that provides the
 capability of extracting from an error source of type `Self::Source`,
 returning error detail of type `Self::Detail`, and an optional error
 tracer of type `Tracer`.

 The proxy type `Self` is not used anywhere. We separate out `Self`
 and `Self::Source` so that there can be different generic implementations
 of error sources, such as for all `E: Display` or for all `E: Error`.

 There are currently 4 types of error sources:
   - [`NoSource`] - Indicating the lack of any error source
   - [`DisplayError`] - An error source that implements [`Display`](std::fmt::Display)
     to be used for tracing, and also be stored as detail.
   - [`DisplayOnly`] - An error source that implements [`Display`](std::fmt::Display)
     to be used for tracing, and discarded instead of being stored as detail.
   - [`DetailOnly`] - An error source that is used as detail and do not contain any error trace.
   - [`TraceError`] - An error source that implements [`Error`](std::error::Error)
     and used only for tracing.
   - [`TraceClone`] - An error source that implements [`Error`](std::error::Error) and
     have a cloned copy as detail.
**/

pub trait ErrorSource<Trace> {
    /// The type of the error source.
    type Source;

    /// The type of the error detail that can be extracted from the error source
    type Detail;

    /// Extracts the error details out from the error source, together with
    /// an optional error trace.
    fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>);
}

/// Type alias to `<Error as ErrorSource<Trace>>::Detail`
pub type AsErrorDetail<Error, Trace> = <Error as ErrorSource<Trace>>::Detail;

/// Type alias to `<Error as ErrorSource<Trace>>::Source`
pub type AsErrorSource<Error, Trace> = <Error as ErrorSource<Trace>>::Source;

/// An [`ErrorSource`] that can be used to represent to lack of any error source.
/// Both its `Source` and `Detail` types are `()`. This can be used for primitive errors
/// that are not caused by any error source.
///
/// In practice, it is also possible to omit specifying any error source inside
/// [`define_error!`](crate::define_error), which has similar effect as using
/// `NoSource` but with the `source` field omitted entirely.
pub struct NoSource;

/// An [`ErrorSource`] that implements [`Display`](std::fmt::Display) and
/// can be traced by error tracers implementing [`ErrorMessageTracer`](crate::tracer::ErrorMessageTracer).
///
/// Both its `Source` and `Detail` types are `E`. When extraced, it also provides
/// an error trace that is traced from its string representation.
pub struct DisplayError<E>(PhantomData<E>);

pub struct DisplayOnly<E>(PhantomData<E>);

/// An [`ErrorSource`] that should implement [`Error`](std::error::Error) and
/// other constraints such as `Send`, `Sync`, `'static`, so that it can be traced
/// by error tracing libraries such as [`eyre`] and [`anyhow`]. Because these libraries
/// take ownership of the source error object, the error cannot be extracted as detail
/// at the same time.
pub struct TraceError<E>(PhantomData<E>);

pub struct TraceClone<E>(PhantomData<E>);

/// An [`ErrorSource`] that contains only the error trace with no detail.
/// This can for example be used for upstream functions that return tracers like
/// [`eyre::Report`] directly.
pub struct TraceOnly<Tracer>(PhantomData<Tracer>);

/// An [`ErrorSource`] that only provides error details but do not provide any trace.
/// This can typically comes from primitive error types that do not implement
/// [`Error`](std::error::Error). The `Detail` type is the error and the returned
/// trace is `None`.
///
/// It is also possible to omit specifying the error as an error source, and instead
/// place it as a field in the error variant. However specifying it as a `DetailOnly`
/// source may give stronger hint to the reader that the particular error variant
/// is caused by other underlying errors.
pub struct DetailOnly<Detail>(PhantomData<Detail>);

pub struct BoxDetail<Detail: ?Sized>(PhantomData<Detail>);

impl<Detail, Trace> ErrorSource<Trace> for DetailOnly<Detail> {
    type Detail = Detail;
    type Source = Detail;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>) {
        (source, None)
    }
}

impl<Trace> ErrorSource<Trace> for NoSource {
    type Detail = ();
    type Source = ();

    fn error_details(_: Self::Source) -> (Self::Detail, Option<Trace>) {
        ((), None)
    }
}

impl<Trace> ErrorSource<Trace> for TraceOnly<Trace> {
    type Detail = ();
    type Source = Trace;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Trace>) {
        ((), Some(source))
    }
}

impl<E, Tracer> ErrorSource<Tracer> for DisplayError<E>
where
    E: Display,
    Tracer: ErrorMessageTracer,
{
    type Detail = E;
    type Source = E;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
        let trace = Tracer::new_message(&source);
        (source, Some(trace))
    }
}

impl<E, Tracer> ErrorSource<Tracer> for DisplayOnly<E>
where
    E: Display,
    Tracer: ErrorMessageTracer,
{
    type Detail = ();
    type Source = E;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
        let trace = Tracer::new_message(&source);
        ((), Some(trace))
    }
}

impl<E, Tracer> ErrorSource<Tracer> for TraceClone<E>
where
    E: Clone,
    Tracer: ErrorTracer<E>,
{
    type Detail = E;
    type Source = E;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
        let detail = source.clone();
        let trace = Tracer::new_trace(source);
        (detail, Some(trace))
    }
}

impl<E, Tracer> ErrorSource<Tracer> for TraceError<E>
where
    Tracer: ErrorTracer<E>,
{
    type Detail = ();
    type Source = E;

    fn error_details(source: Self::Source) -> (Self::Detail, Option<Tracer>) {
        let trace = Tracer::new_trace(source);
        ((), Some(trace))
    }
}
