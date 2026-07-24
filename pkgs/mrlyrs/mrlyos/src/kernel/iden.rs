use serde_json::{json, Value as Json};

#[derive(Clone, Debug, PartialEq)]
pub struct Iden {
    pub id: String,
    pub handle: String,
    pub verified: bool,
}

impl Iden {
    pub fn new(id: &str) -> Iden {
        Iden {
            id: id.to_string(),
            handle: format!("@{}", id),
            verified: false,
        }
    }
    pub fn verify(mut self) -> Iden {
        self.verified = true;
        self
    }
    pub fn to_json(&self) -> Json {
        json!({ "id": self.id, "handle": self.handle, "verified": self.verified })
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
