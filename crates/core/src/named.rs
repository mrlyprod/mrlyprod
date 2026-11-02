/// Declares a fieldless enum whose variants each spell one word, with `all`, `name`, `Display` and `FromStr`.
///
/// ```
/// mrly_core::named_enum! {
///     #[derive(Clone, Copy, Debug, PartialEq)]
///     pub enum Fate {
///         Dead => "dead",
///         Alive => "alive",
///     }
/// }
/// assert_eq!(Fate::all(), [Fate::Dead, Fate::Alive]);
/// assert_eq!("alive".parse::<Fate>().unwrap(), Fate::Alive);
/// assert!("asleep".parse::<Fate>().is_err());
/// ```
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
            #[doc = ::std::concat!("Returns every ", ::std::stringify!($name), " in canonical order.")]
            pub const fn all() -> [$name; [$($name::$variant),+].len()] {
                [$($name::$variant),+]
            }
            #[doc = ::std::concat!("Returns the ", ::std::stringify!($name), "'s display name.")]
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
            type Err = $crate::error::Error;
            fn from_str(word: &str) -> ::std::result::Result<$name, Self::Err> {
                match word {
                    $($word => ::std::result::Result::Ok($name::$variant),)+
                    other => ::std::result::Result::Err($crate::error::Error::Value(::std::format!(
                        "unknown {} {other:?}.",
                        ::std::stringify!($name).to_lowercase()
                    ))),
                }
            }
        }
    };
}
