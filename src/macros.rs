#[macro_export]
macro_rules! make_error {
    ($name:ident
     $(,
         $code:expr => $variant:ident => $msg:literal
     )* $(,)?
    ) => {
        #[derive(Debug, ::thiserror::Error)]
        pub enum $name {
            $(
                #[error($msg)]
                $variant,
            )*
        }

        impl ::std::convert::TryFrom<::reqwest::StatusCode> for $name {
            type Error = ();
            fn try_from(code: ::reqwest::StatusCode) -> Result<Self, Self::Error> {
                match code.as_u16() {
                    $(
                        $code => Ok($name::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}
