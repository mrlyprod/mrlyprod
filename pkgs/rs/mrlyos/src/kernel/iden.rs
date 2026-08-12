use mrlycore::{json, Json};

/// Who is acting: an id, its handle, and whether it is verified.
#[derive(Clone, Debug, PartialEq)]
pub struct Iden {
    /// The bare identifier.
    pub id: String,
    /// The @-prefixed handle.
    pub handle: String,
    /// Whether the identity is verified.
    pub verified: bool,
}

impl Iden {
    /// Builds an unverified identity, deriving the handle from the id.
    ///
    /// ```
    /// use mrlyos::Iden;
    /// assert_eq!(Iden::new("aria").handle, "@aria");
    /// ```
    pub fn new(id: &str) -> Iden {
        Iden {
            id: id.to_string(),
            handle: format!("@{}", id),
            verified: false,
        }
    }
    /// Marks the identity verified and returns it.
    pub fn verify(mut self) -> Iden {
        self.verified = true;
        self
    }
    /// Returns the identity as plain JSON.
    pub fn to_json(&self) -> Json {
        json!({ "id": &self.id, "handle": &self.handle, "verified": self.verified })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn derives_handle() {
        assert_eq!(Iden::new("aria").handle, "@aria");
    }
    #[test]
    fn verify_sets_flag() {
        assert!(Iden::new("aria").verify().verified);
    }
}
