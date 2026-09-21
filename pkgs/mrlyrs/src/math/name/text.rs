use crate::core::errors::{value_error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Atom {
    Number(String),
    Bool(bool),
    Word(String),
    List(Vec<Atom>),
}

impl Atom {
    fn json(&self) -> String {
        match self {
            Atom::Number(digits) => digits.clone(),
            Atom::Bool(flag) => flag.to_string(),
            Atom::Word(word) => format!("\"{word}\""),
            Atom::List(items) => {
                let inner: Vec<String> = items.iter().map(Atom::json).collect();
                format!("[{}]", inner.join(","))
            }
        }
    }
    fn plain(&self) -> String {
        match self {
            Atom::Number(digits) => digits.clone(),
            Atom::Bool(flag) => flag.to_string(),
            Atom::Word(word) => word.clone(),
            Atom::List(items) => items.iter().map(Atom::plain).collect::<Vec<_>>().join(","),
        }
    }
    fn typed(token: &str) -> Result<Atom> {
        match token {
            "true" => Ok(Atom::Bool(true)),
            "false" => Ok(Atom::Bool(false)),
            _ if !token.is_empty() && token.bytes().all(|b| b.is_ascii_digit()) => {
                Ok(Atom::Number(token.to_string()))
            }
            _ if token.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
                && token
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_') =>
            {
                Ok(Atom::Word(token.to_string()))
            }
            _ => value_error(format!(
                "value {token:?} strays outside a-z, 0-9 and underscore."
            )),
        }
    }
    fn list(text: &str) -> Result<Atom> {
        if text.is_empty() {
            return Ok(Atom::List(Vec::new()));
        }
        let items: Result<Vec<Atom>> = text.split(',').map(Atom::typed).collect();
        Ok(Atom::List(items?))
    }
}

struct Reader<'a> {
    text: &'a str,
    at: usize,
}

impl<'a> Reader<'a> {
    fn peek(&self) -> Option<u8> {
        self.text.as_bytes().get(self.at).copied()
    }
    fn take(&mut self, byte: u8) -> Result<()> {
        if self.peek() == Some(byte) {
            self.at += 1;
            return Ok(());
        }
        value_error(format!(
            "json {:?} wants {:?} at {}.",
            self.text, byte as char, self.at
        ))
    }
    fn until(&mut self, stop: impl Fn(u8) -> bool) -> &'a str {
        let start = self.at;
        while self.peek().is_some_and(|b| !stop(b)) {
            self.at += 1;
        }
        &self.text[start..self.at]
    }
    fn word(&mut self) -> Result<String> {
        self.take(b'"')?;
        let word = self.until(|b| b == b'"').to_string();
        self.take(b'"')?;
        Ok(word)
    }
    fn atom(&mut self) -> Result<Atom> {
        match self.peek() {
            Some(b'"') => Ok(Atom::Word(self.word()?)),
            Some(b'[') => {
                self.take(b'[')?;
                let mut items = Vec::new();
                while self.peek() != Some(b']') {
                    items.push(self.atom()?);
                    if self.peek() == Some(b',') {
                        self.at += 1;
                    }
                }
                self.take(b']')?;
                Ok(Atom::List(items))
            }
            Some(b) if b.is_ascii_digit() => Ok(Atom::Number(
                self.until(|b| !b.is_ascii_digit()).to_string(),
            )),
            Some(b't') | Some(b'f') => {
                let word = self.until(|b| !b.is_ascii_lowercase());
                Atom::typed(word)
            }
            _ => value_error(format!(
                "json {:?} holds no value at {}.",
                self.text, self.at
            )),
        }
    }
}

pub(crate) fn pairs(json: &str) -> Result<Vec<(String, Atom)>> {
    let mut reader = Reader { text: json, at: 0 };
    reader.take(b'{')?;
    let mut out = Vec::new();
    while reader.peek() != Some(b'}') {
        let key = reader.word()?;
        reader.take(b':')?;
        out.push((key, reader.atom()?));
        if reader.peek() == Some(b',') {
            reader.at += 1;
        }
    }
    reader.take(b'}')?;
    if reader.at != json.len() {
        return value_error(format!("json {json:?} carries a tail."));
    }
    Ok(out)
}

fn kinded(json: &str) -> (String, Vec<(String, Atom)>) {
    let mut list = pairs(json).expect("a canonical name reads back");
    let (key, kind) = list.remove(0);
    assert_eq!(key, "kind");
    let Atom::Word(kind) = kind else {
        panic!("the kind is a word");
    };
    (kind, list)
}

pub(crate) fn json(kind: &str, fields: &[(String, Atom)]) -> String {
    let mut out = format!("{{\"kind\":\"{kind}\"");
    for (key, value) in fields {
        out.push_str(&format!(",\"{key}\":{}", value.json()));
    }
    out.push('}');
    out
}

fn key(text: &str) -> Result<&str> {
    if !text.is_empty() && text.bytes().all(|b| b.is_ascii_lowercase()) {
        return Ok(text);
    }
    value_error(format!("key {text:?} strays outside a-z."))
}

pub(crate) fn url(json: &str) -> String {
    let (kind, fields) = kinded(json);
    if fields.is_empty() {
        return format!("/{kind}");
    }
    let query: Vec<String> = fields
        .iter()
        .map(|(key, value)| format!("{key}={}", value.plain()))
        .collect();
    format!("/{kind}?{}", query.join("&"))
}

pub(crate) fn url_to_json(text: &str, kind: &str, lists: &[&str]) -> Result<String> {
    let Some(rest) = text.strip_prefix('/') else {
        return value_error(format!("url {text:?} does not open with a slash."));
    };
    let (path, query) = rest.split_once('?').unwrap_or((rest, ""));
    if path != kind {
        return value_error(format!("url {text:?} is not a {kind}."));
    }
    let mut fields = Vec::new();
    for pair in query.split('&').filter(|pair| !pair.is_empty()) {
        let Some((name, value)) = pair.split_once('=') else {
            return value_error(format!("url field {pair:?} has no value."));
        };
        let listed = lists.contains(&name)
            && value
                .split(',')
                .all(|token| !matches!(Atom::typed(token), Ok(Atom::Word(_))));
        let atom = if value.contains(',') || listed {
            Atom::list(value)?
        } else {
            Atom::typed(value)?
        };
        fields.push((key(name)?.to_string(), atom));
    }
    Ok(json(kind, &fields))
}

pub(crate) fn file(json: &str) -> String {
    let (kind, fields) = kinded(json);
    let mut out = kind;
    for (key, value) in &fields {
        let spelt = match value {
            Atom::List(_) => format!("[{}]", value.plain()),
            _ => value.plain(),
        };
        out.push_str(&format!("_{key}={spelt}"));
    }
    out
}

fn opens_field(text: &str) -> bool {
    let letters = text.bytes().take_while(u8::is_ascii_lowercase).count();
    letters > 0 && text.as_bytes().get(letters) == Some(&b'=')
}

pub(crate) fn file_to_json(text: &str, kind: &str) -> Result<String> {
    let mut cuts = vec![0];
    for (i, b) in text.bytes().enumerate() {
        if b == b'_' && opens_field(&text[i + 1..]) {
            cuts.push(i);
        }
    }
    cuts.push(text.len());
    let mut parts = cuts.windows(2).map(|w| &text[w[0]..w[1]]);
    let head = parts.next().unwrap_or("");
    if head != kind {
        return value_error(format!("file {text:?} is not a {kind}."));
    }
    let mut fields = Vec::new();
    for part in parts {
        let Some((name, value)) = part[1..].split_once('=') else {
            return value_error(format!("file field {part:?} has no value."));
        };
        let atom = match value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
            Some(inner) => Atom::list(inner)?,
            None => Atom::typed(value)?,
        };
        fields.push((key(name)?.to_string(), atom));
    }
    Ok(json(kind, &fields))
}

pub(crate) fn mrly(json: &str, bare: &[&str]) -> String {
    let (kind, fields) = kinded(json);
    let mut parts = Vec::new();
    for (key, value) in &fields {
        parts.push(match value {
            Atom::Bool(true) => key.clone(),
            Atom::Word(word) if bare.contains(&key.as_str()) => word.clone(),
            Atom::List(items) => {
                let inner: Vec<String> = items.iter().map(Atom::plain).collect();
                format!("{key} [{}]", inner.join(" "))
            }
            other => format!("{key} {}", other.plain()),
        });
    }
    if parts.is_empty() {
        return kind;
    }
    format!("{kind} {}", parts.join(", "))
}

pub(crate) fn longest<'a>(text: &'a str, names: &[String]) -> Option<(usize, &'a str)> {
    let mut best: Option<(usize, &str)> = None;
    for (i, name) in names.iter().enumerate() {
        let Some(rest) = text.strip_prefix(name.as_str()) else {
            continue;
        };
        if best.is_none_or(|(_, tail)| rest.len() < tail.len()) {
            best = Some((i, rest));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const KOCH: &str =
        r#"{"kind":"bang","dim":2,"lattice":"hex","base":3,"code":39,"twist":[0,1,5,0]}"#;

    #[test]
    fn the_canonical_text_reads_into_pairs_and_back() {
        let (kind, fields) = kinded(KOCH);
        assert_eq!(kind, "bang");
        assert_eq!(fields.len(), 5);
        assert_eq!(fields[1], ("lattice".into(), Atom::Word("hex".into())));
        assert_eq!(json(&kind, &fields), KOCH);
        assert!(pairs("{\"kind\":\"bang\"} ").is_err());
        assert!(pairs("[1]").is_err());
    }

    #[test]
    fn the_views_hold_verbatim() {
        assert_eq!(
            url(KOCH),
            "/bang?dim=2&lattice=hex&base=3&code=39&twist=0,1,5,0"
        );
        assert_eq!(
            file(KOCH),
            "bang_dim=2_lattice=hex_base=3_code=39_twist=[0,1,5,0]"
        );
        assert_eq!(
            mrly(KOCH, &["lattice"]),
            "bang dim 2, hex, base 3, code 39, twist [0 1 5 0]"
        );
        assert_eq!(
            mrly(KOCH, &[]),
            "bang dim 2, lattice hex, base 3, code 39, twist [0 1 5 0]"
        );
    }

    #[test]
    fn the_decodable_views_read_back() {
        assert_eq!(url_to_json(&url(KOCH), "bang", &["twist"]).unwrap(), KOCH);
        assert_eq!(file_to_json(&file(KOCH), "bang").unwrap(), KOCH);
        let rule = r#"{"kind":"rule","birth":[3],"survive":"grid_squares_ones","wrap":true}"#;
        assert_eq!(
            url_to_json(&url(rule), "rule", &["birth", "survive"]).unwrap(),
            rule
        );
        assert_eq!(file_to_json(&file(rule), "rule").unwrap(), rule);
        let empty = r#"{"kind":"rule","birth":[],"survive":[]}"#;
        assert_eq!(
            url_to_json(&url(empty), "rule", &["birth", "survive"]).unwrap(),
            empty
        );
        assert_eq!(file_to_json(&file(empty), "rule").unwrap(), empty);
        assert!(url_to_json("/rule?dim=2", "bang", &[]).is_err());
        assert!(url_to_json("bang?dim=2", "bang", &[]).is_err());
        assert!(url_to_json("/bang?dim", "bang", &[]).is_err());
        assert!(url_to_json("/bang?Dim=2", "bang", &[]).is_err());
        assert!(url_to_json("/bang?dim=2&lattice=Hex", "bang", &[]).is_err());
        assert!(file_to_json("rule_dim=2", "bang").is_err());
        assert!(file_to_json("bang_dim=2_x", "bang").is_err());
    }

    #[test]
    fn a_bare_kind_prints_alone() {
        let plain = r#"{"kind":"rule"}"#;
        assert_eq!(url(plain), "/rule");
        assert_eq!(file(plain), "rule");
        assert_eq!(mrly(plain, &[]), "rule");
        assert_eq!(url_to_json("/rule", "rule", &[]).unwrap(), plain);
        assert_eq!(file_to_json("rule", "rule").unwrap(), plain);
    }
}
