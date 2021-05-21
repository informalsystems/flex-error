
#[macro_export]
macro_rules! define_error {
  ( $name:ident; $(
      $suberror:ident
      $( ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) )?
      [ $source:ty ]
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
          source: $crate::eyre::AsErrorDetail<$source>
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
                write!( f, "{}",  suberror )
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
          [ $source ]
        }
      )*
    ];
  };
}

#[macro_export]
macro_rules! define_error_constructor {
  ( $name:ident;
    $suberror:ident;
    ( $( $arg_name:ident: $arg_type:ty ),* )
    [ $source:ty ]
  ) => {
    paste::paste! [
      pub fn [< $suberror:lower _error >](
        $( $arg_name: $arg_type, )*
        source: $crate::eyre::AsErrorSource< $source >
      ) -> $name
      {
        use $crate::eyre::{Report, EyreReport, error_details};

        let (source_detail, m_source_report) = error_details::< $source >(source);

        let suberror = [< $suberror SubError >] {
          $( $arg_name, )*
          source: source_detail,
        };

        let detail = [< $name Detail >]::$suberror(suberror);

        let message = format!("{}", detail);
        let report = match m_source_report {
          Some(source_report) => source_report.wrap_err(message),
          None => EyreReport::msg(message)
        };

        Report {
          detail,
          report,
        }
      }
    ];
  }
}
