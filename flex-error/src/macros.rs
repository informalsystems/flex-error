pub use paste::paste;

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
            [< Err $suberror >]
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

        impl core::fmt::Display for [< Err $suberror >] {
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
  ) => {
    $crate::macros::paste! [
      #[derive(Debug)]
      pub struct [< Err $suberror >] {
        $( $arg_name: $arg_type, )*
      }
    ];
  };
  ( $tracer:ty;
    $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
    $( [ $source:ty ] )?
  ) => {
    $crate::macros::paste! [
      #[derive(Debug)]
      pub struct [< Err $suberror >] {
        $( $arg_name: $arg_type, )*
        $( source: $crate::AsErrorDetail<$source, $tracer> )?
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
        let detail = [< $name Detail >]::$suberror([< Err $suberror >] {
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
            [< $name Detail >]::$suberror([< Err $suberror >] {
              $( $arg_name, )*
              source: source_detail,
            })
          })
      }
    ];
  };
}
