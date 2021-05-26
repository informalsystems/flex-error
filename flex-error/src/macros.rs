pub use paste::paste;

/// This is the main macro that implements a mini DSL to
/// define error types using `flex-error`. The DSL syntax
/// is as follows:
///
/// ```
/// define_error! { ErrorName;
///   SubErrorWithFieldsAndErrorSource
///     { field1: Type1, field2: Type2, ... }
///     [ ErrorSource ]
///     | e | { format_args!(
///       "format error message with field1: {}, field2: {}, source: {}",
///       e.field1, e.field2, e.source)
///     },
///   SubErrorWithFieldsOnly
///     { field1: Type1, field2: Type2, ... }
///     | e | { format_args!(
///       "format error message with field1: {}, field2: {}",
///       e.field1, e.field2)
///     },
///   SubErrorWithSourceOnly
///     [ ErrorSource ]
///     | e | { format_args!(
///       "format error message with source: {}",
///       e.source)
///     },
///   SubError
///     | e | { format_args!(
///       "only suberror message")
///     },
/// }
/// ```
///
/// Behind the scene, `define_error` does the following:
///
///   - Define an enum with the postfix `Detail`, e.g. an error named
///     `FooError` would have the enum `FooErrorDetail` defined.
///
///   - Define the error name as a type alias to
///     [`ErrorReport<ErrorNameDetail, DefaultTracer>`](crate::ErrorReport).
///     e.g.`type FooError = ErrorReport<FooErrorDetail, DefaultTracer>;`.
///
///   - For each suberror, does the following:
///
///       - Define a variant with the suberror name in the detail enum.
///         e.g. a `BarSubError` in `FooError` becomes a `BarSubError`
///         variant in `FooErrorDetail`.
///       - Define a struct with the `Subdetail` postfix. e.g.
///         `BarSubError` would have a `BarSubErrorSubdetail` struct.
///
///         - The struct contains all named fields if specified.
///
///         - If an error source is specified, a `source` field is
///           also defined with the type
///           [`AsErrorDetail<ErrorSource>`](crate::AsErrorDetail).
///           e.g. a suberror with
///           [`DisplayError<SourceError>`](crate::DisplayError)
///           would have the field `source: SourceError`.
///
///       - Implement [`Display`](std::fmt::Display) for the suberror
///         using the provided formatter to format the arguments.
///         The argument type of the formatter is the suberror subdetail struct.
///
///       - Define a suberror constructor function in snake case with the postfix
///         `_error`.
#[macro_export]
macro_rules! define_error {
  ( $($expr:tt)+ ) => {
    define_error_with_tracer![ $crate::DefaultTracer; $( $expr )* ];
  };
}

#[macro_export]
macro_rules! define_error_with_tracer {
  ( $tracer:ty; $name:ident; $(
      $suberror:ident
      $( { $( $arg_name:ident : $arg_type:ty ),* $(,)? } )?
      $( [ $source:ty ] )?
      | $formatter_arg:pat | $formatter:expr
    ),* $(,)?
  ) => {
    $crate::macros::paste![
      #[derive(Debug)]
      pub enum [< $name Detail >] {
        $(
          $suberror (
            [< $suberror Subdetail >]
          ),
        )*
      }

      $(
        $crate::define_suberror! {
          $tracer;
          $name;
          $suberror;
          ( $( $( $arg_name : $arg_type ),* )? )
          $( [ $source ] )?
        }

        impl core::fmt::Display for [< $suberror Subdetail >] {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let $formatter_arg = self;
            write!(f, "{}",  $formatter)
          }
        }
      )*

      pub type $name = $crate::ErrorReport< [< $name Detail >], $tracer >;

      impl core::fmt::Display for [< $name Detail >] {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            $(
              Self::$suberror( suberror ) => {
                write!( f, "{}",  suberror )
              }
            ),*
          }
        }
      }

      $(
        $crate::define_error_constructor! {
          $tracer;
          $name;
          $suberror;
          ( $( $( $arg_name : $arg_type ),* )? )
          $( [ $source ] )?
        }
      )*
    ];
  };
}

#[macro_export]
macro_rules! define_suberror {
  ( $tracer:ty;
    $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
    $( [ $source:ty ] )?
  ) => {
    $crate::macros::paste! [
      #[derive(Debug)]
      pub struct [< $suberror Subdetail >] {
        $( pub $arg_name: $arg_type, )*
        $( pub source: $crate::AsErrorDetail<$source, $tracer> )?
      }
    ];
  };
}

#[macro_export]
macro_rules! define_error_constructor {
  ( $tracer:ty;
    $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
  ) => {
    $crate::macros::paste! [
      pub fn [< $suberror:snake _error >](
        $( $arg_name: $arg_type, )*
      ) -> $name
      {
        let detail = [< $name Detail >]::$suberror([< $suberror Subdetail >] {
          $( $arg_name, )*
        });

        let trace = $tracer::new_message(&detail);
        $crate::ErrorReport {
          detail,
          trace,
        }
      }
    ];
  };
  ( $tracer:ty;
    $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
    [ $source:ty ]
  ) => {
    $crate::macros::paste! [
      pub fn [< $suberror:snake _error >](
        $( $arg_name: $arg_type, )*
        source: $crate::AsErrorSource< $source, $tracer >
      ) -> $name
      {
        $crate::ErrorReport::trace_from::<$source, _>(source,
          | source_detail | {
            [< $name Detail >]::$suberror([< $suberror Subdetail >] {
              $( $arg_name, )*
              source: source_detail,
            })
          })
      }
    ];
  };
}
