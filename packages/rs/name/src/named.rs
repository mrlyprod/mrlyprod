#[macro_export]
macro_rules! named_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident => $word:literal,)+
        }
    ) => {
        $(#[$enum_meta])*
        pub enum $name {
            $($(#[$variant_meta])* $variant,)+
        }

        impl $name {
            pub const fn all() -> [$name; [$($name::$variant),+].len()] {
                [$($name::$variant),+]
            }
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => $word,)+
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::__private::Error;
            fn from_str(word: &str) -> ::std::result::Result<$name, Self::Err> {
                match word {
                    $($word => ::std::result::Result::Ok($name::$variant),)+
                    other => ::std::result::Result::Err($crate::__private::Error::Value(::std::format!(
                        "unknown {} {other:?}.",
                        ::std::stringify!($name).to_lowercase()
                    ))),
                }
            }
        }
    };
}
