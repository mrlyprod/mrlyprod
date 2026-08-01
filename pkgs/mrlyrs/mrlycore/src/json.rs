use crate::errors::{value_error, MrlyError, Result};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Json {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Map),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Map {
    entries: Vec<(String, Json)>,
}

static NULL: Json = Json::Null;

impl Json {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Json::Int(n) if *n >= 0 => Some(*n as u64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Json>> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Json>> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Map> {
        match self {
            Json::Obj(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Map> {
        match self {
            Json::Obj(m) => Some(m),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Json::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Json::Int(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Json::Str(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Json::Arr(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Json::Obj(_))
    }

    pub fn get<I: JsonIndex>(&self, index: I) -> Option<&Json> {
        index.index_into(self)
    }

    pub fn get_mut<I: JsonIndex>(&mut self, index: I) -> Option<&mut Json> {
        index.index_into_mut(self)
    }

    pub fn take(&mut self) -> Json {
        std::mem::take(self)
    }

    pub fn pretty(&self) -> String {
        let mut out = String::new();
        write_pretty(self, &mut out, 0);
        out
    }
}

impl Map {
    pub fn new() -> Map {
        Map {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Json) -> Option<Json> {
        for (k, v) in &mut self.entries {
            if *k == key {
                return Some(std::mem::replace(v, value));
            }
        }
        self.entries.push((key, value));
        None
    }

    pub fn get(&self, key: &str) -> Option<&Json> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Json> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Json> {
        let index = self.entries.iter().position(|(k, _)| k == key)?;
        Some(self.entries.remove(index).1)
    }

    pub fn entry(&mut self, key: impl Into<String>) -> Entry<'_> {
        Entry {
            map: self,
            key: key.into(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Json)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Json)> {
        self.entries.iter_mut().map(|(k, v)| (&*k, v))
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &Json> {
        self.entries.iter().map(|(_, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Json> {
        self.entries.iter_mut().map(|(_, v)| v)
    }
}

pub struct Entry<'a> {
    map: &'a mut Map,
    key: String,
}

impl<'a> Entry<'a> {
    pub fn or_insert(self, default: Json) -> &'a mut Json {
        self.or_insert_with(|| default)
    }

    pub fn or_insert_with(self, default: impl FnOnce() -> Json) -> &'a mut Json {
        let index = match self.map.entries.iter().position(|(k, _)| *k == self.key) {
            Some(index) => index,
            None => {
                self.map.entries.push((self.key, default()));
                self.map.entries.len() - 1
            }
        };
        &mut self.map.entries[index].1
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = &'a (String, Json);
    type IntoIter = std::slice::Iter<'a, (String, Json)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl IntoIterator for Map {
    type Item = (String, Json);
    type IntoIter = std::vec::IntoIter<(String, Json)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl FromIterator<(String, Json)> for Map {
    fn from_iter<T: IntoIterator<Item = (String, Json)>>(iter: T) -> Map {
        let mut map = Map::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

pub trait JsonIndex {
    fn index_into<'a>(&self, value: &'a Json) -> Option<&'a Json>;
    fn index_into_mut<'a>(&self, value: &'a mut Json) -> Option<&'a mut Json>;
    fn index_or_insert<'a>(&self, value: &'a mut Json) -> &'a mut Json;
}

impl JsonIndex for str {
    fn index_into<'a>(&self, value: &'a Json) -> Option<&'a Json> {
        value.as_object().and_then(|m| m.get(self))
    }

    fn index_into_mut<'a>(&self, value: &'a mut Json) -> Option<&'a mut Json> {
        value.as_object_mut().and_then(|m| m.get_mut(self))
    }

    fn index_or_insert<'a>(&self, value: &'a mut Json) -> &'a mut Json {
        if value.is_null() {
            *value = Json::Obj(Map::new());
        }
        match value {
            Json::Obj(map) => map.entry(self).or_insert(Json::Null),
            _ => panic!("cannot index into {value} with string"),
        }
    }
}

impl<T: JsonIndex + ?Sized> JsonIndex for &T {
    fn index_into<'a>(&self, value: &'a Json) -> Option<&'a Json> {
        (**self).index_into(value)
    }

    fn index_into_mut<'a>(&self, value: &'a mut Json) -> Option<&'a mut Json> {
        (**self).index_into_mut(value)
    }

    fn index_or_insert<'a>(&self, value: &'a mut Json) -> &'a mut Json {
        (**self).index_or_insert(value)
    }
}

impl JsonIndex for String {
    fn index_into<'a>(&self, value: &'a Json) -> Option<&'a Json> {
        self.as_str().index_into(value)
    }

    fn index_into_mut<'a>(&self, value: &'a mut Json) -> Option<&'a mut Json> {
        self.as_str().index_into_mut(value)
    }

    fn index_or_insert<'a>(&self, value: &'a mut Json) -> &'a mut Json {
        self.as_str().index_or_insert(value)
    }
}

impl JsonIndex for usize {
    fn index_into<'a>(&self, value: &'a Json) -> Option<&'a Json> {
        value.as_array().and_then(|a| a.get(*self))
    }

    fn index_into_mut<'a>(&self, value: &'a mut Json) -> Option<&'a mut Json> {
        value.as_array_mut().and_then(|a| a.get_mut(*self))
    }

    fn index_or_insert<'a>(&self, value: &'a mut Json) -> &'a mut Json {
        match value {
            Json::Arr(a) => a.get_mut(*self).expect("index out of bounds"),
            _ => panic!("cannot index into {value} with number"),
        }
    }
}

impl<I: JsonIndex> std::ops::Index<I> for Json {
    type Output = Json;

    fn index(&self, index: I) -> &Json {
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I: JsonIndex> std::ops::IndexMut<I> for Json {
    fn index_mut(&mut self, index: I) -> &mut Json {
        index.index_or_insert(self)
    }
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Bool(b)
    }
}

impl From<&str> for Json {
    fn from(s: &str) -> Json {
        Json::Str(s.to_string())
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::Str(s)
    }
}

impl<T: Clone + Into<Json>> From<&T> for Json {
    fn from(value: &T) -> Json {
        value.clone().into()
    }
}

impl<T: Clone + Into<Json>, const N: usize> From<[T; N]> for Json {
    fn from(items: [T; N]) -> Json {
        Json::Arr(items.into_iter().map(Into::into).collect())
    }
}

impl From<Map> for Json {
    fn from(m: Map) -> Json {
        Json::Obj(m)
    }
}

impl From<()> for Json {
    fn from(_: ()) -> Json {
        Json::Null
    }
}

macro_rules! from_int {
    ($($t:ty)*) => {
        $(impl From<$t> for Json {
            fn from(n: $t) -> Json {
                Json::Int(n as i64)
            }
        })*
    };
}

from_int!(i8 i16 i32 i64 isize u8 u16 u32);

impl From<u64> for Json {
    fn from(n: u64) -> Json {
        Json::Int(i64::try_from(n).expect("integer too large for the Tree"))
    }
}

impl From<usize> for Json {
    fn from(n: usize) -> Json {
        Json::Int(i64::try_from(n).expect("integer too large for the Tree"))
    }
}

impl<T: Into<Json>> From<Vec<T>> for Json {
    fn from(items: Vec<T>) -> Json {
        Json::Arr(items.into_iter().map(Into::into).collect())
    }
}

impl<T: Clone + Into<Json>> From<&[T]> for Json {
    fn from(items: &[T]) -> Json {
        Json::Arr(items.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Json>> From<Option<T>> for Json {
    fn from(option: Option<T>) -> Json {
        match option {
            Some(value) => value.into(),
            None => Json::Null,
        }
    }
}

impl<T: Into<Json>> FromIterator<T> for Json {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Json {
        Json::Arr(iter.into_iter().map(Into::into).collect())
    }
}

impl PartialEq<i64> for Json {
    fn eq(&self, other: &i64) -> bool {
        self.as_i64() == Some(*other)
    }
}

impl PartialEq<Json> for i64 {
    fn eq(&self, other: &Json) -> bool {
        other.as_i64() == Some(*self)
    }
}

impl PartialEq<i32> for Json {
    fn eq(&self, other: &i32) -> bool {
        self.as_i64() == Some(*other as i64)
    }
}

impl PartialEq<Json> for i32 {
    fn eq(&self, other: &Json) -> bool {
        other.as_i64() == Some(*self as i64)
    }
}

impl PartialEq<&str> for Json {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == Some(*other)
    }
}

impl PartialEq<Json> for &str {
    fn eq(&self, other: &Json) -> bool {
        other.as_str() == Some(*self)
    }
}

impl PartialEq<str> for Json {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == Some(other)
    }
}

impl PartialEq<String> for Json {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == Some(other.as_str())
    }
}

impl PartialEq<bool> for Json {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == Some(*other)
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        write_compact(self, &mut out);
        f.write_str(&out)
    }
}

fn write_compact(value: &Json, out: &mut String) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Bool(true) => out.push_str("true"),
        Json::Bool(false) => out.push_str("false"),
        Json::Int(n) => out.push_str(&n.to_string()),
        Json::Str(s) => write_escaped(s, out),
        Json::Arr(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_compact(item, out);
            }
            out.push(']');
        }
        Json::Obj(map) => {
            out.push('{');
            for (i, (key, item)) in map.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_escaped(key, out);
                out.push(':');
                write_compact(item, out);
            }
            out.push('}');
        }
    }
}

fn write_pretty(value: &Json, out: &mut String, level: usize) {
    match value {
        Json::Arr(items) if !items.is_empty() => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                out.push_str(if i == 0 { "\n" } else { ",\n" });
                indent(out, level + 1);
                write_pretty(item, out, level + 1);
            }
            out.push('\n');
            indent(out, level);
            out.push(']');
        }
        Json::Obj(map) if !map.is_empty() => {
            out.push('{');
            for (i, (key, item)) in map.iter().enumerate() {
                out.push_str(if i == 0 { "\n" } else { ",\n" });
                indent(out, level + 1);
                write_escaped(key, out);
                out.push_str(": ");
                write_pretty(item, out, level + 1);
            }
            out.push('\n');
            indent(out, level);
            out.push('}');
        }
        other => write_compact(other, out),
    }
}

fn indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("  ");
    }
}

fn write_escaped(s: &str, out: &mut String) {
    out.push('"');
    let bytes = s.as_bytes();
    let mut start = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        let escape: Option<String> = match byte {
            b'"' => Some("\\\"".to_string()),
            b'\\' => Some("\\\\".to_string()),
            0x08 => Some("\\b".to_string()),
            0x09 => Some("\\t".to_string()),
            0x0a => Some("\\n".to_string()),
            0x0c => Some("\\f".to_string()),
            0x0d => Some("\\r".to_string()),
            0x00..=0x1f => Some(format!("\\u{byte:04x}")),
            _ => None,
        };
        if let Some(escape) = escape {
            out.push_str(&s[start..i]);
            out.push_str(&escape);
            start = i + 1;
        }
    }
    out.push_str(&s[start..]);
    out.push('"');
}

pub fn parse(text: &str) -> Result<Json> {
    let mut parser = Parser {
        text,
        bytes: text.as_bytes(),
        pos: 0,
    };
    parser.skip_whitespace();
    let value = parser.value(0)?;
    parser.skip_whitespace();
    if parser.pos != parser.bytes.len() {
        return parser.fail("trailing characters");
    }
    Ok(value)
}

struct Parser<'a> {
    text: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn fail<T>(&self, message: &str) -> Result<T> {
        value_error(format!("json: {message} at byte {}", self.pos))
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(b' ' | b'\t' | b'\n' | b'\r') = self.peek() {
            self.pos += 1;
        }
    }

    fn eat(&mut self, expected: u8, name: &str) -> Result<()> {
        if self.peek() == Some(expected) {
            self.pos += 1;
            Ok(())
        } else {
            self.fail(name)
        }
    }

    fn literal(&mut self, word: &str, value: Json) -> Result<Json> {
        if self.bytes[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Ok(value)
        } else {
            self.fail("invalid literal")
        }
    }

    fn value(&mut self, depth: usize) -> Result<Json> {
        if depth > 128 {
            return self.fail("nesting too deep");
        }
        match self.peek() {
            Some(b'n') => self.literal("null", Json::Null),
            Some(b't') => self.literal("true", Json::Bool(true)),
            Some(b'f') => self.literal("false", Json::Bool(false)),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b'[') => self.array(depth),
            Some(b'{') => self.object(depth),
            Some(b'-' | b'0'..=b'9') => self.integer(),
            Some(_) => self.fail("unexpected character"),
            None => self.fail("unexpected end of input"),
        }
    }

    fn array(&mut self, depth: usize) -> Result<Json> {
        self.pos += 1;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            self.skip_whitespace();
            items.push(self.value(depth + 1)?);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Arr(items));
                }
                _ => return self.fail("expected , or ]"),
            }
        }
    }

    fn object(&mut self, depth: usize) -> Result<Json> {
        self.pos += 1;
        let mut map = Map::new();
        self.skip_whitespace();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(Json::Obj(map));
        }
        loop {
            self.skip_whitespace();
            if self.peek() != Some(b'"') {
                return self.fail("expected object key");
            }
            let key = self.string()?;
            self.skip_whitespace();
            self.eat(b':', "expected :")?;
            self.skip_whitespace();
            let value = self.value(depth + 1)?;
            map.insert(key, value);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Obj(map));
                }
                _ => return self.fail("expected , or }"),
            }
        }
    }

    fn integer(&mut self) -> Result<Json> {
        let negative = self.peek() == Some(b'-');
        if negative {
            self.pos += 1;
        }
        let mut value: i64 = 0;
        match self.peek() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                while let Some(digit @ b'0'..=b'9') = self.peek() {
                    value = value
                        .checked_mul(10)
                        .and_then(|v| v.checked_sub((digit - b'0') as i64))
                        .ok_or(MrlyError::Value("json: integer overflow".to_string()))?;
                    self.pos += 1;
                }
            }
            _ => return self.fail("expected digit"),
        }
        match self.peek() {
            Some(b'.' | b'e' | b'E') => self.fail("floats are not Tree material"),
            Some(b'0'..=b'9') => self.fail("leading zero"),
            _ => {
                if negative {
                    Ok(Json::Int(value))
                } else {
                    value
                        .checked_neg()
                        .map(Json::Int)
                        .ok_or(MrlyError::Value("json: integer overflow".to_string()))
                }
            }
        }
    }

    fn string(&mut self) -> Result<String> {
        self.pos += 1;
        let mut out = String::new();
        let mut start = self.pos;
        loop {
            match self.peek() {
                Some(b'"') => {
                    out.push_str(&self.text[start..self.pos]);
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    out.push_str(&self.text[start..self.pos]);
                    self.pos += 1;
                    self.escape(&mut out)?;
                    start = self.pos;
                }
                Some(0x00..=0x1f) => return self.fail("control character in string"),
                Some(_) => self.pos += 1,
                None => return self.fail("unterminated string"),
            }
        }
    }

    fn escape(&mut self, out: &mut String) -> Result<()> {
        let code = self.peek();
        self.pos += 1;
        match code {
            Some(b'"') => out.push('"'),
            Some(b'\\') => out.push('\\'),
            Some(b'/') => out.push('/'),
            Some(b'b') => out.push('\u{0008}'),
            Some(b'f') => out.push('\u{000c}'),
            Some(b'n') => out.push('\n'),
            Some(b'r') => out.push('\r'),
            Some(b't') => out.push('\t'),
            Some(b'u') => {
                let first = self.hex4()?;
                let c = if (0xd800..0xdc00).contains(&first) {
                    if self.peek() != Some(b'\\') {
                        return self.fail("unpaired surrogate");
                    }
                    self.pos += 1;
                    if self.peek() != Some(b'u') {
                        return self.fail("unpaired surrogate");
                    }
                    self.pos += 1;
                    let second = self.hex4()?;
                    if !(0xdc00..0xe000).contains(&second) {
                        return self.fail("unpaired surrogate");
                    }
                    let combined = 0x10000 + ((first - 0xd800) << 10) + (second - 0xdc00);
                    char::from_u32(combined)
                        .ok_or(MrlyError::Value("json: invalid escape".to_string()))?
                } else if (0xdc00..0xe000).contains(&first) {
                    return self.fail("unpaired surrogate");
                } else {
                    char::from_u32(first)
                        .ok_or(MrlyError::Value("json: invalid escape".to_string()))?
                };
                out.push(c);
            }
            _ => return self.fail("invalid escape"),
        }
        Ok(())
    }

    fn hex4(&mut self) -> Result<u32> {
        let mut value = 0u32;
        for _ in 0..4 {
            let digit = match self.peek() {
                Some(b @ b'0'..=b'9') => (b - b'0') as u32,
                Some(b @ b'a'..=b'f') => (b - b'a') as u32 + 10,
                Some(b @ b'A'..=b'F') => (b - b'A') as u32 + 10,
                _ => return self.fail("invalid hex escape"),
            };
            value = value * 16 + digit;
            self.pos += 1;
        }
        Ok(value)
    }
}

#[macro_export]
macro_rules! json {
    ($($json:tt)+) => {
        $crate::json_internal!($($json)+)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_internal {
    (@array [$($elems:expr,)*]) => {
        vec![$($elems,)*]
    };
    (@array [$($elems:expr),*]) => {
        vec![$($elems),*]
    };
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!(null)] $($rest)*)
    };
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!(true)] $($rest)*)
    };
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!(false)] $($rest)*)
    };
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!([$($array)*])] $($rest)*)
    };
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!({$($map)*})] $($rest)*)
    };
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!($next),] $($rest)*)
    };
    (@array [$($elems:expr,)*] $last:expr) => {
        $crate::json_internal!(@array [$($elems,)* $crate::json_internal!($last)])
    };
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        $crate::json_internal!(@array [$($elems,)*] $($rest)*)
    };
    (@object $object:ident () () ()) => {};
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        $crate::json_internal!(@object $object () ($($rest)*) ($($rest)*));
    };
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!(null)) $($rest)*);
    };
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!(true)) $($rest)*);
    };
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!(false)) $($rest)*);
    };
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!([$($array)*])) $($rest)*);
    };
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!({$($map)*})) $($rest)*);
    };
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!($value)) , $($rest)*);
    };
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        $crate::json_internal!(@object $object [$($key)+] ($crate::json_internal!($value)));
    };
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        $crate::json_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };
    (null) => {
        $crate::json::Json::Null
    };
    (true) => {
        $crate::json::Json::Bool(true)
    };
    (false) => {
        $crate::json::Json::Bool(false)
    };
    ([]) => {
        $crate::json::Json::Arr(vec![])
    };
    ([ $($tt:tt)+ ]) => {
        $crate::json::Json::Arr($crate::json_internal!(@array [] $($tt)+))
    };
    ({}) => {
        $crate::json::Json::Obj($crate::json::Map::new())
    };
    ({ $($tt:tt)+ }) => {
        $crate::json::Json::Obj({
            let mut object = $crate::json::Map::new();
            $crate::json_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };
    ($other:expr) => {
        $crate::json::Json::from(&$other)
    };
}
