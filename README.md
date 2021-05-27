# flex-error

`flex-error` is a lightweight Rust library that uses macros and traits to switch between different error tracing implementations and no_std. The library currently supports 3 modes via Cargo feature flags: `eyre_tracer` (default), `anyhow_tracer`, and `string_tracer` (no_std).

The library separates out several concepts as traits: `ErrorDetail`, `ErrorTrace`, and `ErrorSource`.
  - The `ErrorDetail` is responsible to structured metadata information about a specific error.
  - The `ErrorTracer` is responsible for tracing error chains and backtraces.
  - The `ErrorSource` allows generic conversion of external error types into an ErrorDetail with optional ErrorTrace.
  - An application error is of type `ErrorReport<ErrorDetail, ErrorTracer>`, which holds both the error details and trace.

With the separation of concerns, `flex-error` allows applications to easily switch between different error reporting implementations, such as `eyre` and `anyhow`, by implementing `ErrorTracer` for the respective reporters.

`flex-error` defines a `define_error!` macro that define custom `Detail` types and error types as alias to `ErrorReport<Detail, DefaultTracer>`. The `DefaultTracer` type is set globally by the feature flag, so that application error types do not have to be over-generalized. The trade off is that it is not possible to use multiple `ErrorTracer` implementations at the same time across different crates that use `flex-error`.

## Demo

The [flex-error-demo-full](./flex-error-demo-full) directory contains a [demo](./flex-error-demo-full/src/main.rs) program that reports error using `eyre`. When run with `RUST_BACKTRACE=1`, it should produce something like follows:

```bash
$ RUST_BACKTRACE=1 cargo run flex-error-demo-full

Error:
   0: error caused by foo: Foo has failed
   1: system error
   2: error1

Location:
   flex-error/flex-error/src/tracer_impl/eyre.rs:25

  ---------------------------------- BACKTRACE -----------------------------------
                                ⋮ 5 frames hidden ⋮
   6: flex_error::tracer_impl::eyre::<impl flex_error::tracer::ErrorTracer<E> for eyre::Report>::new_trace::h471dd777954521dc
      at flex-error/flex-error/src/tracer_impl/eyre.rs:25
   7: flex_error::tracer::<impl flex_error::source::ErrorSource<Tracer> for flex_error::source::StdError<E>>::error_details::hba2719390b7121af
      at flex-error/flex-error/src/tracer.rs:26
   8: flex_error::report::ErrorReport<Detail,Trace>::trace_from::h39963733bcaf4442
      at flex-error/flex-error/src/report.rs:32
   9: flex_error_demo_full::foo::system_error::h9d131e83ad96a9bb
      at flex-error/flex-error/src/macros.rs:111
  10: flex_error_demo_full::main::hb0f615a97c840949
      at flex-error/flex-error-demo-full/src/main.rs:50
```
