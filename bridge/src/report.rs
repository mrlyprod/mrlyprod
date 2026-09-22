use crate::model::{Cross, Manifest, Source};
use std::collections::BTreeMap;

pub fn skip_lines(m: &Manifest) -> Vec<String> {
    m.functions
        .iter()
        .filter(|f| f.source == Source::Written)
        .filter_map(|f| match &f.cross {
            Cross::Skip { reason } => Some(format!("{} {reason}", f.path)),
            _ => None,
        })
        .collect()
}

pub fn check_skip(m: &Manifest, skip_txt: &str) -> Vec<String> {
    let mut errors = vec![];
    let skipped: Vec<&str> = m
        .functions
        .iter()
        .filter(|f| f.source == Source::Written && matches!(f.cross, Cross::Skip { .. }))
        .map(|f| f.path.as_str())
        .collect();
    let listed: Vec<&str> = skip_txt
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .collect();
    for path in &skipped {
        if !listed.contains(path) {
            errors.push(format!("skip.txt is missing {path}"));
        }
    }
    for path in &listed {
        if !skipped.contains(path) {
            errors.push(format!("skip.txt names nothing: {path}"));
        }
    }
    errors.sort();
    errors.dedup();
    errors
}

pub fn summary(m: &Manifest, macro_body_fns: usize) -> Vec<String> {
    let mut counts: BTreeMap<String, [usize; 3]> = BTreeMap::new();
    for f in &m.functions {
        let unit = f.module.split("::").next().unwrap_or_default().to_string();
        let slot = counts.entry(unit).or_default();
        match f.cross {
            Cross::Ok => slot[0] += 1,
            Cross::Skip { .. } => slot[1] += 1,
            Cross::Private => slot[2] += 1,
        }
    }
    let mut lines: Vec<String> = counts
        .iter()
        .map(|(unit, [ok, skip, private])| {
            format!("{unit}: ok {ok}, skip {skip}, private {private}")
        })
        .collect();
    let total: [usize; 3] = counts
        .values()
        .fold([0; 3], |a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2]]);
    let written = m
        .functions
        .iter()
        .filter(|f| f.source == Source::Written && !f.constant)
        .count();
    let constant = m
        .functions
        .iter()
        .filter(|f| f.source == Source::Written && f.constant)
        .count();
    let generated = m
        .functions
        .iter()
        .filter(|f| f.source == Source::NamedEnum)
        .count();
    lines.push(format!(
        "functions {}: ok {}, skip {}, private {}; {written} pub fn + {constant} pub const fn + {generated} macro-written; {macro_body_fns} pub fn lines inside macro_rules",
        m.functions.len(),
        total[0],
        total[1],
        total[2]
    ));
    lines
}
