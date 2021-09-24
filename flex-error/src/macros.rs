pub use paste::paste;

/**
  `define_error!` is the main macro that implements a mini DSL to
  define error types using `flex-error`. The DSL syntax
  is as follows:

  ```ignore
  define_error! {
    ErrorName {
      SubErrorWithFieldsAndErrorSource
        { field1: Type1, field2: Type2, ... }
        [ ErrorSource ]
        | e | { format_args!(
          "format error message with field1: {}, field2: {}, source: {}",
          e.field1, e.field2, e.source)
        },
      SubErrorWithFieldsOnly
        { field1: Type1, field2: Type2, ... }
        | e | { format_args!(
          "format error message with field1: {}, field2: {}",
          e.field1, e.field2)
        },
      SubErrorWithSourceOnly
        [ ErrorSource ]
        | e | { format_args!(
          "format error message with source: {}",
          e.source)
        },
      SubError
        | e | { format_args!(
          "only suberror message")
        },
    }
  }
  ```

  ## Macro Expansion

  Behind the scene, for an error named `MyError`, `define_error!`
  does the following:

    - Define a struct in the form

      ```ignore
      pub struct MyError(pub MyErrorDetail, pub flex_error::DefaultTracer)
      ```

    - Define an enum in the form

      ```ignore
      pub enum MyError { ... }
      ```

      For each suberror named `MySubError`, does the following:

        - Define a variant in `MyError` in the form
          ```ignore
          pub enum MyError { ..., MySubError(MySubErrorSubdetail) ,... }
          ```

          - Implement [`core::fmt::Debug`] and [`core::fmt::Display`]
            for `MyError`.

          - If the `"std"` feature is enabled on the `flex-error` crate,
            it will generate an `impl` block for [`std::error::Error`].

          - Implement [`ErrorSource<DefaultTracer>`](crate::ErrorSource)
            for `MyError`, with `MyErrorDetail` being the `Detail` type,
            and `MyError` being the `Source` type.

          - Implement the following helper methods in `impl MyError {...}`:

            - `pub fn detail(&self) -> &MyErrorDetail`

            - `pub fn trace(&self) -> flex_error::DefaultTracer`

            - `pub fn add_trace<E: Display>(self, e: &E) -> MyError`

        - Define a struct in the form

          ```ignore
          pub struct MySubErrorSubdetail { ... }
          ```

          - For each field named `my_field: MyFieldType`, define a struct
            field in the form

            ```ignore
            struct MySubErrorSubdetail { ..., pub my_field: MyFieldType, ... }
            ```

          - If the sub-error has an error source `MySource` implementing
            [`ErrorSource<DefaultTracer>`](crate::ErrorSource), define a `source` field
            in the form

            ```ignore
            struct MySubErrorSubdetail { ..., pub source: MySource::Detail }
            ```

          - Implement [`core::fmt::Display`] in the form

            ```ignore
            impl Display for MySubErrorSubdetail { ... }
            ```

        - Define a snake-cased error constructor method in `MyError` in the form

          ```ignore
          impl MyError { pub fn my_sub_error(...) -> MyError { ... } }
          ```

          - For each field named `my_field: MyFieldType`, define a
            function argument in the form

            ```ignore
            fn my_sub_error(..., my_field: MyFieldType, ...)
            ```

          - If the sub-error has an error source `MySource` implementing
            [`ErrorSource`](crate::ErrorSource), define a `source` field in the form

            ```ignore
            fn my_sub_error(..., source: MySource::Detail)
            ```

  ## Formatter

  For each sub-error definition, a formatter needs to be provided using the
  closure syntax. For example, the following definition:


  ```ignore
  MyError {
    MySubError
      { code: u32 }
      [ MySource ]
      | e | { format_args!("error with code {}", e.code) },
    ...
  }
  ```

  will include the following expansion:

  ```
  impl ::core::fmt::Display for MySubErrorSubdetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
      let e = self;
      write!(f, "{}", format_args!("error with code {}", e.code))
    }
  }
  ```

  Note that there is no need to manually display the error source, as the
  source is already automatically traced by the error tracer.

  If a sub-error do not have any field, we can write a simpler form of the
  formatter like:

  ```ignore
  MyError {
    MySubError
      | _ | { "something went wrong" },
    ...
  }
  ```

  ## Example Definition

  We can demonstrate the macro expansion of `define_error!` with the following example:

  ```ignore
  // An external error type implementing Display
  use external_crate::ExternalError;

  define_error! {
    FooError {
      Bar
        { code: u32 }
        [ DisplayError<ExternalError> ]
        | e | { format_args!("Bar error with code {}", e.code) },
      Baz
        { extra: String }
        | e | { format_args!("General Baz error with extra detail: {}", e.extra) }
    }
  }
  ```

  The above code will be expanded into something like follows:

  ```ignore
  pub struct FooError(pub FooErrorDetail, pub flex_error::DefaultTracer);

  #[derive(Debug)]
  pub enum FooErrorDetail {
      Bar(BarSubdetail),
      Baz(BazSubdetail),
  }

  #[derive(Debug)]
  pub struct BarSubdetail {
      pub code: u32,
      pub source: ExternalError
  }

  #[derive(Debug)]
  pub struct BazSubdetail {
      pub extra: String
  }

  fn bar_error(code: u32, source: ExternalError) -> FooError { ... }
  fn baz_error(extra: String) -> FooError { ... }

  impl ::core::fmt::Display for BarSubdetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
      let e = self;
      write!(f, "{}", format_args!("Bar error with code {}", e.code))
    }
  }

  impl ::core::fmt::Display for BazSubdetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
      let e = self;
      write!(f, "{}", format_args!("General Baz error with extra detail: {}", e.code))
    }
  }

  impl Display for FooErrorDetail { ... }
  ```

  For the detailed macro expansion, you can use [cargo-expand](https://github.com/dtolnay/cargo-expand)
  to expand the Rust module that uses `define_error!` to see how the error definition
  gets expanded.

  Since `FooError` implements [`ErrorSource`](crate::ErrorSource), it can be used
  directly as a error source in other error definitions. For example:

  ```ignore
  define_error! {
    QuuxError {
      Foo
        { action: String }
        [ FooError ]
        | e | { format_args!("error arised from Foo when performing action {}", e.action) },
      ...
    }
  }
  ```

  Would be expanded to include the following definitions:

  ```ignore
  pub struct FooSubdetail {
    pub action: String,
    pub source: FooErrorDetail
  }

  pub fn foo_error(action: String, source: FooError) { ... }
  ```

  In the formatter for `QuuxErrorDetail::Foo`, we can also see that it does not
  need to include the error string from `FooError`. This is because the error
  tracer already takes care of the source error trace, so the full trace is
  automatically tracked inside `foo_error`. The outer error only need to
  add additional detail about what caused the source error to be raised.

  ## Attributes

  `define_error!` supports adding attributes to the generated error types.

  ### First `doc` Attribute

  If the first attribute is a [`doc`](https://doc.rust-lang.org/rustdoc/the-doc-attribute.html)
  attribute, it is attached to the main error struct. For example:

  ```ignore
  define_error! {
    /// Documentation for MyError
    MyError { ... }
  }
  ```

  will include the following expansion:

  ```ignore
  #[doc = "Documentation for MyError"]
  pub struct MyError(pub MyErrorDetail, pub flex_error::DefaultTracer);
  ```

  ## Common Attributes

  For all other attributes defined on the main error type,
  they are defined on the _error detail_ and _sub-errors type. For example:


  ```ignore
  define_error! {
    #[derive(Debug, Clone)]
    MyError {
      Foo
        { ... }
        | _ | { "foo error" },

      Bar
        { ... }
        | _ | { "bar error" },
    }
  }
  ```

  will include the following expansion:

  ```ignore
  pub struct MyError(pub MyErrorDetail, pub flex_error::DefaultTracer);

  #[derive(Debug, Clone)]
  pub enum MyErrorDetail { ... }

  #[derive(Debug, Clone)]
  pub struct FooSubdetail { ... }

  #[derive(Debug, Clone)]
  pub struct BarSubdetail { ... }
  ```

  Note that we do not add the `derive` attribute to the main error struct
  `MyError`. This is because the [`DefaultTracer`](crate::DefaultTracer)
  type is opaque, and auto deriving traits like `Clone` on it is
  generally not supported.

  If you need the main error type to implement certain traits,
  you can instead define your own custom `impl` definition for it.

  ## Sub Attributes

  We can also define custom attributes for only the sub-error.
  In that case, the attribute is given to the sub-detail type.
  For example:

  ```ignore
  define_error! {
    MyError {
      /// Documentation for Foo
      #[derive(Clone)]
      Foo
        { ... }
        | _ | { "foo error" },

      ...
    }
  }
  ```

  will include the following expansion:

  ```ignore
  #[doc = "Documentation for Foo"]
  #[derive(Clone)]
  pub struct FooSubdetail { ... }
  ```

  Note that if no attribute is given to the main error,
  the `#[derive(Debug)]` trait is added by default.
  So there is no need to derive it again in the
  sub-errors.

**/

#[macro_export]
macro_rules! define_error {
  ( $name:ident
    { $($suberrors:tt)* }
  ) => {
    $crate::define_error_with_tracer![
      @tracer( $crate::DefaultTracer ),
      @attr[ derive(Debug) ],
      @name( $name ),
      @suberrors{ $($suberrors)* }
    ];
  };
  ( #[doc = $doc:literal] $( #[$attr:meta] )*
    $name:ident
    { $($suberrors:tt)* }
  ) => {
    $crate::define_error_with_tracer![
      @tracer( $crate::DefaultTracer ),
      @doc( $doc ),
      @attr[ $( $attr ),* ],
      @name( $name ),
      @suberrors{ $($suberrors)* }
    ];
  };
  ( $( #[$attr:meta] )*
    $name:ident
    { $($suberrors:tt)* }
  ) => {
    $crate::define_error_with_tracer![
      @tracer( $crate::DefaultTracer ),
      @attr[ $( $attr ),* ],
      @name( $name ),
      @suberrors{ $($suberrors)* }
    ];
  };
  ( @with_tracer[ $tracer:ty ]
    $name:ident,
    { $($suberrors:tt)* }
  ) => {
    $crate::define_error_with_tracer![
      @tracer( $tracer ),
      @attr[ derive(Debug) ],
      @name( $name ),
      @suberrors{ $($suberrors)* }
    ];
  };
  ( @with_tracer[ $tracer:ty ]
    $( #[$attr:meta] )*
    $name:ident,
    @suberrors{ $($suberrors:tt)* }
  ) => {
    $crate::define_error_with_tracer![
      @tracer( $tracer ),
      @attr[ $( $attr ),* ],
      @name( $name ),
      @suberrors{ $($suberrors)* }
    ];
  };
}

/// This macro allows error types to be defined with custom error tracer types
/// other than [`DefaultTracer`](crate::DefaultTracer). Behind the scene,
/// a macro call to `define_error!{ ... } really expands to
/// `define_error_with_tracer!{ flex_error::DefaultTracer; ... }`
#[macro_export]
#[doc(hidden)]
macro_rules! define_error_with_tracer {
  ( @tracer( $tracer:ty ),
    $( @doc($doc:literal), )?
    @attr[ $( $attr:meta ),* ],
    @name($name:ident),
    @suberrors{ $($suberrors:tt)* } $(,)?
  ) => {
    $crate::macros::paste![
      $crate::define_main_error!(
        @tracer( $tracer ),
        $( @doc( $doc ), )?
        @name( $name )
      );

      $crate::define_error_detail!(
        @attr[ $( $attr ),* ] ,
        @name( $name ),
        @suberrors{ $($suberrors)* });

      $crate::define_suberrors! {
        @tracer($tracer),
        @attr[ $( $attr ),* ],
        @name($name),
        { $( $suberrors )* }
      }
    ];
  };
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_main_error {
  ( @tracer( $tracer:ty ),
    $( @doc( $doc:literal ), )?
    @name( $name:ident ) $(,)?
  ) => {
    $crate::macros::paste![
      $crate::define_main_error_struct!(
        @tracer( $tracer ),
        $( @doc($doc), )?
        @name( $name )
      );

      impl $crate::ErrorSource<$tracer> for $name {
        type Source = Self;
        type Detail = [< $name Detail >];

        fn error_details($name(detail, trace): Self) -> ([< $name Detail >], Option<$tracer>) {
            (detail, Some(trace))
        }
      }

      impl ::core::fmt::Debug for $name
      where
          $tracer: ::core::fmt::Debug,
      {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
              ::core::fmt::Debug::fmt(self.trace(), f)
          }
      }

      impl ::core::fmt::Display for $name
      where
          $tracer: ::core::fmt::Debug,
      {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>)
            -> ::core::fmt::Result
          {
              // Always use `Debug` to format error traces, as eyre do not
              // include full back trace information in normal Display mode.
              ::core::fmt::Debug::fmt(self.trace(), f)
          }
      }

      $crate::define_std_err_impl!(
        @tracer( $tracer ),
        @name( $name )
      );

      impl $name {
        pub fn detail(&self) -> &[< $name Detail >] {
            &self.0
        }


        pub fn into_detail(self) -> [< $name Detail >] {
            self.0
        }

        pub fn trace(&self) -> &$tracer {
            &self.1
        }

        pub fn into_trace(self) -> $tracer {
            self.1
        }

        pub fn add_trace<E: ::core::fmt::Display>(self, message: &E) -> Self
        where
            $tracer: $crate::ErrorMessageTracer,
        {
            let detail = self.0;
            let trace = $crate::ErrorMessageTracer::add_message(self.1, message);
            $name(detail, trace)
        }

        pub fn trace_from<E, Cont>(source: E::Source, cont: Cont) -> Self
        where
            E: $crate::ErrorSource<$tracer>,
            $tracer: $crate::ErrorMessageTracer,
            Cont: FnOnce(E::Detail) -> [< $name Detail >],
        {
            let (detail1, m_trace1) = E::error_details(source);
            let detail2 = cont(detail1);
            match m_trace1 {
                Some(trace1) => {
                    let trace2 = $crate::ErrorMessageTracer::add_message(trace1, &detail2);
                    $name(detail2, trace2)
                }
                None => {
                    let trace2 = $crate::ErrorMessageTracer::new_message(&detail2);
                    $name(detail2, trace2)
                }
            }
        }
      }
    ];
  }
}

// define the impl for `std::error::Error` only in std mode
#[cfg(feature = "std")]
#[macro_export]
#[doc(hidden)]
macro_rules! define_std_err_impl {
  ( @tracer( $tracer:ty ),
    @name( $name:ident ) $(,)?
  ) => {
    $crate::macros::paste![
      impl $crate::StdError for $name
      where
          [< $name Detail >]: ::core::fmt::Display,
          $tracer: ::core::fmt::Debug + ::core::fmt::Display,
          $tracer: $crate::ErrorMessageTracer,
      {
          fn source(&self) -> ::core::option::Option<&(dyn $crate::StdError + 'static)> {
              $crate::ErrorMessageTracer::as_error(self.trace())
          }
      }
    ];
  }
}

// do not define the impl for `std::error::Error` when in no_std mode
#[cfg(not(feature = "std"))]
#[macro_export]
#[doc(hidden)]
macro_rules! define_std_err_impl {
    ( @tracer( $tracer:ty ),
    @name( $name:ident ) $(,)?
  ) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_main_error_struct {
  ( @tracer( $tracer:ty ),
    $( @doc( $doc:literal ), )?
    @name( $name:ident ) $(,)?
  ) => {
    $crate::macros::paste![
      $( #[doc = $doc] )?
      pub struct $name (pub [< $name Detail >], pub $tracer);
    ];
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! with_suberrors {
  ( @cont($cont:path),
    @ctx[ $($args:tt)* ],
    @suberrors{
      $(
        $( #[$sub_attr:meta] )*
        $suberror:ident
        $( { $( $arg_name:ident : $arg_type:ty ),* $(,)? } )?
        $( [ $source:ty ] )?
        | $formatter_arg:pat | $formatter:expr
      ),* $(,)?
    } $(,)?
  ) => {
    $cont!( @ctx[ $( $args )* ], @suberrors{ $( $suberror ),* } );
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_error_detail {
  ( @attr[ $( $attr:meta ),* ],
    @name( $name:ident ),
    @suberrors{ $($suberrors:tt)* } $(,)?
  ) => {
    $crate::with_suberrors!(
      @cont($crate::define_error_detail_enum),
      @ctx[
        @attr[ $( $attr ),* ],
        @name($name)
      ],
      @suberrors{ $( $suberrors )* }
    );

    $crate::with_suberrors!(
      @cont($crate::define_error_detail_display),
      @ctx[
        @name($name)
      ],
      @suberrors{ $( $suberrors )* }
    );
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_error_detail_enum {
  ( @ctx[
      @attr[ $( $attr:meta ),* ],
      @name($name:ident)
    ],
    @suberrors{ $( $suberror:ident ),* } $(,)?
  ) => {
    $crate::macros::paste! [
      $( #[$attr] )*
      pub enum [< $name Detail >] {
        $(
          $suberror (
            [< $suberror Subdetail >]
          )
        ),*
      }
    ];
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_error_detail_display {
  ( @ctx[
      @name( $name:ident )
    ],
    @suberrors{ $( $suberror:ident ),* } $(,)?
  ) => {
    $crate::macros::paste! [
      impl ::core::fmt::Display for [< $name Detail >] {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>)
          -> ::core::fmt::Result
        {
          match self {
            $(
              Self::$suberror( suberror ) => {
                ::core::write!( f, "{}",  suberror )
              }
            ),*
          }
        }
      }
    ];
  }
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_suberrors {
  ( @tracer($tracer:ty),
    @attr[ $( $attr:meta ),* ],
    @name($name:ident),
    {} $(,)?
  ) => { };
  ( @tracer($tracer:ty),
    @attr[ $( $attr:meta ),* ],
    @name($name:ident),
    {
      $( #[$sub_attr:meta] )*
      $suberror:ident
        $( { $( $arg_name:ident : $arg_type:ty ),* $(,)? } )?
        $( [ $source:ty ] )?
        | $formatter_arg:pat | $formatter:expr

      $( , $($tail:tt)* )?
    }
  ) => {
    $crate::macros::paste![
      $crate::define_suberror! {
        @tracer( $tracer ),
        @attr[ $( $attr ),* ],
        @sub_attr[ $( $sub_attr ),* ],
        @name( $name ),
        @suberror( $suberror ),
        @args( $( $( $arg_name : $arg_type ),* )? )
        $( @source[ $source ] )?
      }

      impl ::core::fmt::Display for [< $suberror Subdetail >] {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          use ::core::format_args;
          let $formatter_arg = self;
          ::core::write!(f, "{}",  $formatter)
        }
      }

      impl $name {
        $crate::define_error_constructor! {
          @tracer( $tracer ),
          @name( $name ),
          @suberror( $suberror ),
          @args( $( $( $arg_name : $arg_type ),* )? )
          $( @source[ $source ] )?
        }
      }
    ];

    $crate::define_suberrors! {
      @tracer($tracer),
      @attr[ $( $attr ),* ],
      @name($name),
      { $( $( $tail )* )? }
    }
  };
}

/// Internal macro used to define suberror structs
#[macro_export]
#[doc(hidden)]
macro_rules! define_suberror {
  ( @tracer( $tracer:ty ),
    @attr[ $( $attr:meta ),* ],
    @sub_attr[ $( $sub_attr:meta ),* ],
    @name( $name:ident ),
    @suberror( $suberror:ident ),
    @args( $( $arg_name:ident: $arg_type:ty ),* )
    @source[ Self ]
  ) => {
    $crate::macros::paste! [
      $( #[ $attr ] )*
      $( #[ $sub_attr ] )*
      pub struct [< $suberror Subdetail >] {
        $( pub $arg_name: $arg_type, )*
        pub source: $crate::alloc::boxed::Box< [< $name Detail >] >
      }
    ];
  };
  ( @tracer( $tracer:ty ),
    @attr[ $( $attr:meta ),* ],
    @sub_attr[ $( $sub_attr:meta ),* ],
    @name( $name:ident ),
    @suberror( $suberror:ident ),
    @args( $( $arg_name:ident: $arg_type:ty ),* )
    $( @source[ $source:ty ] )?
  ) => {
    $crate::macros::paste! [
      $( #[ $attr ] )*
      $( #[ $sub_attr ] )*
      pub struct [< $suberror Subdetail >] {
        $( pub $arg_name: $arg_type, )*
        $( pub source: $crate::AsErrorDetail<$source, $tracer> )?
      }
    ];
  };
}

/// Internal macro used to define suberror constructor functions
#[macro_export]
#[doc(hidden)]
macro_rules! define_error_constructor {
  ( @tracer( $tracer:ty ),
    @name( $name:ident ),
    @suberror( $suberror:ident ),
    @args( $( $arg_name:ident: $arg_type:ty ),* ) $(,)?
  ) => {
    $crate::macros::paste! [
      pub fn [< $suberror:snake >](
        $( $arg_name: $arg_type, )*
      ) -> $name
      {
        let detail = [< $name Detail >]::$suberror([< $suberror Subdetail >] {
          $( $arg_name, )*
        });

        let trace = < $tracer as $crate::ErrorMessageTracer >::new_message(&detail);
        $name(detail, trace)
      }
    ];
  };
  ( @tracer( $tracer:ty ),
    @name( $name:ident ),
    @suberror( $suberror:ident ),
    @args( $( $arg_name:ident: $arg_type:ty ),* )
    @source[ Self ]
  ) => {
    $crate::macros::paste! [
      pub fn [< $suberror:snake >](
        $( $arg_name: $arg_type, )*
        source: $name
      ) -> $name
      {
        let detail = [< $name Detail >]::$suberror([< $suberror Subdetail >] {
          $( $arg_name, )*
          source: Box::new(source.0),
        });

        let trace = source.1.add_message(&detail);

        $name(detail, trace)
      }
    ];
  };
  ( @tracer( $tracer:ty ),
    @name( $name:ident ),
    @suberror( $suberror:ident ),
    @args( $( $arg_name:ident: $arg_type:ty ),* )
    @source[ $source:ty ]
  ) => {
    $crate::macros::paste! [
      pub fn [< $suberror:snake >](
        $( $arg_name: $arg_type, )*
        source: $crate::AsErrorSource< $source, $tracer >
      ) -> $name
      {
        $name::trace_from::<$source, _>(source,
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
