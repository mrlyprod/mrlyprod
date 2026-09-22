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
            #[doc = concat!("Returns every ", stringify!($name), " in canonical order.")]
            pub const fn all() -> [$name; [$($name::$variant),+].len()] {
                [$($name::$variant),+]
            }
            #[doc = concat!("Returns the ", stringify!($name), "'s display name.")]
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
            type Err = $crate::core::error::Error;
            fn from_str(word: &str) -> ::std::result::Result<$name, Self::Err> {
                match word {
                    $($word => Ok($name::$variant),)+
                    other => Err($crate::core::error::Error::Value(format!(
                        "unknown {} {other:?}.",
                        stringify!($name).to_lowercase()
                    ))),
                }
            }
        }
    };
}

pub(crate) use named_enum;
