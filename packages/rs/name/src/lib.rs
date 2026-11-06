mod named;

#[doc(hidden)]
pub mod __private {
    pub use mrly_error::Error;
    pub use serde;
}

use mrly_error::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod sha;
mod text;

pub use text::longest;

pub trait Named: Serialize + DeserializeOwned + Sized {
    const KIND: &'static str;
    const LISTS: &'static [&'static str] = &[];
    const BARE: &'static [&'static str] = &[];
    fn checked(self) -> Result<Self>;
    fn to_json(&self) -> String {
        mrly_json::write(self).expect("a name serializes")
    }
    fn from_json(text: &str) -> Result<Self> {
        let value: Self = mrly_json::read(text)?;
        value.checked()
    }
    fn to_id(&self) -> String {
        sha::short(self.to_json().as_bytes())
    }
    fn to_url(&self) -> Result<String> {
        text::url(&self.to_json())
    }
    fn from_url(text: &str) -> Result<Self> {
        Self::from_json(&text::url_to_json(text, Self::KIND, Self::LISTS)?)
    }
    fn to_file(&self) -> Result<String> {
        text::file(&self.to_json())
    }
    fn from_file(text: &str) -> Result<Self> {
        Self::from_json(&text::file_to_json(text, Self::KIND)?)
    }
    fn to_prose(&self) -> Result<String> {
        text::prose(&self.to_json(), Self::BARE)
    }
}

#[macro_export]
macro_rules! kind {
    ($word:literal) => {
        #[derive(
            ::std::clone::Clone,
            ::std::marker::Copy,
            ::std::fmt::Debug,
            ::std::default::Default,
            ::std::cmp::PartialEq,
            ::std::cmp::Eq,
            ::std::hash::Hash,
        )]
        pub struct Kind;

        impl $crate::__private::serde::Serialize for Kind {
            fn serialize<S: $crate::__private::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str($word)
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for Kind {
            fn deserialize<D: $crate::__private::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Kind, D::Error> {
                let word =
                    <::std::string::String as $crate::__private::serde::Deserialize>::deserialize(
                        deserializer,
                    )?;
                if word == $word {
                    ::std::result::Result::Ok(Kind)
                } else {
                    ::std::result::Result::Err(
                        <D::Error as $crate::__private::serde::de::Error>::custom(::std::format!(
                            "kind {word:?} is not {:?}",
                            $word
                        )),
                    )
                }
            }
        }
    };
}
