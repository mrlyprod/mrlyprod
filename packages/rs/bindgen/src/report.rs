use crate::js;
use crate::model::{Cross, Function, Manifest, Source};
use std::collections::{BTreeMap, BTreeSet};

fn skipped(ms: &[Manifest]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for m in ms {
        out.extend(
            m.functions
                .iter()
                .filter(|f| f.source != Source::NamedEnum)
                .filter_map(|f| match &f.cross {
                    Cross::Skip { reason } => Some((f.path.clone(), reason.clone())),
                    _ => None,
                }),
        );
        out.extend(js::field_skips(m));
    }
    let mut seen = BTreeSet::new();
    out.retain(|(path, _)| seen.insert(path.clone()));
    out
}

pub fn skips(ms: &[Manifest]) -> String {
    let mut out = String::new();
    for (path, reason) in skipped(ms) {
        out.push_str(&format!("{path} {reason}\n"));
    }
    out
}

pub fn summary(ms: &[Manifest], macro_body_fns: usize) -> Vec<String> {
    let functions: Vec<&Function> = ms.iter().flat_map(|m| &m.functions).collect();
    let mut counts: BTreeMap<String, [usize; 3]> = BTreeMap::new();
    for f in &functions {
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
    let written = functions
        .iter()
        .filter(|f| f.source == Source::Written && !f.constant)
        .count();
    let constant = functions
        .iter()
        .filter(|f| f.source == Source::Written && f.constant)
        .count();
    let generated = functions
        .iter()
        .filter(|f| f.source == Source::NamedEnum)
        .count();
    let traits = functions
        .iter()
        .filter(|f| f.source == Source::Trait)
        .count();
    let defaults = functions
        .iter()
        .filter(|f| f.source == Source::Default)
        .count();
    lines.push(format!(
        "functions {}: ok {}, skip {}, private {}; {written} pub fn + {constant} pub const fn + {generated} macro-written + {traits} trait methods + {defaults} defaults; {macro_body_fns} pub fn lines inside macro_rules",
        functions.len(),
        total[0],
        total[1],
        total[2]
    ));
    lines
}
