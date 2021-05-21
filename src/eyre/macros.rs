
#[macro_export]
macro_rules! define_error {
  ( $name:ident; $(
      $suberror:ident
      $( ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) )?
      $( [ $($source:tt)* ] )?
      =>
      | $formatter_arg:ident | $formatter:expr
    ),* $(,)?
  ) => {
    paste::paste![
      #[derive(Debug)]
      pub enum [< $name Detail >] {
          $(
            $suberror (
              [< $suberror SubError >]
            ),
          )*
      }

      $(
        #[derive(Debug)]
        pub struct [< $suberror SubError >] {
          $(
            $( $arg_name: $arg_type, )*
          )?
          $(
            source: $crate::error_source_detail!( $($source)* )
          )?
        }

        impl core::fmt::Display for [< $suberror SubError >] {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let $formatter_arg = self;
            write!(f, "{}",  $formatter)
          }
        }
      )*

      pub type $name = $crate::eyre::Report< [< $name Detail >] >;

      impl core::fmt::Display for [< $name Detail >] {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            $(
              Self::$suberror( suberror ) => {
                write!( f, "{}",  suberror );
                Ok(())
              }
            ),*
          }
        }
      }

      $(
        $crate::define_error_constructor! {
          $name;
          $suberror;
          ( $( $( $arg_name : $arg_type ),* )? )
          $( [ $($source)* ] )?
        }
      )*
    ];
  };
}

#[macro_export]
macro_rules! error_source_var {
  ( report[$source:ty] ) => { source };
  ( basic[$source:ty] ) => { source };
  () => {}
}

#[macro_export]
macro_rules! error_source_detail {
  ( report[$source:ty] ) => {
    $crate::eyre::ReportDetail($source)
  };
  ( basic[$source:ty] ) => {
    $source
  };
}

#[macro_export]
macro_rules! define_error_constructor {
  ( $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
    [ report[$source:ty] ]
  ) => {
    paste::paste! [
      pub fn [< $kind:lower_error >](
        $( $arg_name: $arg_type ),*
        source: $source
      ) -> $name
      {
        use $crate::eyre::Report;

        let Report {
          source: source_detail,
          report: source_report,
        } = source;

        let detail = [< $name Detail >]::$suberror(
          $( $arg_name ),*
          source_detail,
        );

        let message = format!("{}", detail);
        let report = source_report.wrap_err(message);

        Report {
          detail,
          report,
        }
      }
    ]
  };

  ( $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
    [ basic[$source:ty] ]
  ) => {
    paste::paste! [
      pub fn [< $suberror:lower _error >](
        $( $arg_name: $arg_type ),*,
        source: $source
      ) -> $name
      {
        use $crate::eyre::{Report, EyreReport};

        let suberror = [< $suberror SubError >] {
          $( $arg_name ),*,
          source,
        };

        let detail = [< $name Detail >]::$suberror(suberror);

        let message = format!("{}", detail);
        let report = EyreReport::msg(message);

        Report {
          detail,
          report,
        }
      }
    ];
  }
}
