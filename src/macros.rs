/// A macro to automatically run a blocking version of some async function,
/// used for the blocking module.
#[macro_export]
macro_rules! run_blocking {
    ($original: expr) => {
        $crate::blocking::RT
            .handle()
            .block_on(async move { $original.await })
    };
}

/// A more advanced macro that will implement an endpoint for both async code
/// and blocking.
///
/// The macro takes a variable number of functions with the signature you'd
/// expect for an endpoint: public, async, and with a docstring. In order to
/// capture the docstring, it has to use the `#[doc]` macro.
///
/// These functions will be added to both the async client (the default one)
/// and the blocking one (only when the `blocking` feature is used).
#[macro_export]
macro_rules! endpoint_impl {
    ($(#[doc = $doc:expr]
       pub async fn $name:ident (
           // With this, it's possible to use `self` in the functions declared
           // inside this macro, but it's limited to an immutable reference
           // for now.
           &$self:ident,
           // The function may take a variable number of arguments.
           $($param:ident : $paramty:ty),*
        ) -> $ret:ty $code:block
    )*) => {
        impl $crate::client::Spotify {
            $(
                #[doc = $doc]
                pub async fn $name (&$self, $($param : $paramty),*) -> $ret $code
            )*
        }

        #[cfg(feature = "blocking")]
        impl $crate::blocking::client::Spotify {
            $(
                #[doc = $doc]
                pub fn $name (&$self, $($param : $paramty),*) -> $ret {
                    $crate::run_blocking! {
                        $crate::client::Spotify::$name($self.0, $($param),*)
                    }
                }
            )*
        }
    };
}
