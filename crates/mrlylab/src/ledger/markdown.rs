use super::records::record_by_id;
use super::{keys, sequence, Axis, Key, Record, Sequence, Tier, BUDGET, RECORDS, SPACES, TERMS};

const PROSE: &str = include_str!("prose.md");

fn label(key: &Key) -> &'static str {
    match (key.dimension, key.code) {
        (2, 1) | (3, 1) => "corner",
        (2, 3) => "tree",
        (2, 7) => "carpet",
        (2, 9) | (3, 129) => "void",
        (2, 11) => "corner and centre",
        (2, 15) | (3, 255) => "solid",
        (3, 23) => "sponge",
        _ => "design",
    }
}

fn link(id: &str) -> String {
    format!("[{id}](https://oeis.org/{id})")
}

fn spelled(row: &Sequence) -> String {
    let terms: Vec<String> = row.terms.iter().map(|term| term.to_string()).collect();
    let mut text = format!("`{}`", terms.join(", "));
    if row.capped {
        text.push_str(" to the cell budget");
    }
    text
}

fn table(axis: Axis) -> String {
    let index = match axis {
        Axis::Level => "`L = 1`",
        Axis::Side => "`k = 2`",
    };
    let mut out = format!("| design | key | closed form | terms from {index} | record | shift | status |\n|---|---|---|---|---|---|---|\n");
    let mut keyed: Vec<(Key, &Record)> = RECORDS
        .iter()
        .filter_map(|record| {
            record
                .key
                .filter(|key| key.axis == axis)
                .map(|key| (key, record))
        })
        .collect();
    keyed.sort_by_key(|(key, _)| (key.dimension, key.code, key.measure));
    for (key, record) in keyed {
        let row = sequence(&key, TERMS, BUDGET).expect("every keyed record reads");
        let (matched, shift) = row.record.expect("every keyed record matches");
        assert_eq!(matched.id, record.id, "the row reads its own record");
        let closed = row
            .closed
            .as_ref()
            .map_or_else(|| "none".to_string(), |form| format!("`{}`", form.text()));
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | **{}** |\n",
            label(&key),
            key.name(),
            closed,
            spelled(&row),
            link(record.id),
            shift,
            record.status.text()
        ));
    }
    out.trim_end().to_string()
}

fn records() -> String {
    let mut out = String::from(
        "| record | name | offset | first terms | key | shift | status |\n|---|---|---|---|---|---|---|\n",
    );
    for record in RECORDS {
        let (key, shift) = match record.key {
            Some(key) => (format!("`{}`", key.name()), record.shift.to_string()),
            None => (String::new(), String::new()),
        };
        out.push_str(&format!(
            "| {} | {} | {} | `{}` | {} | {} | **{}** |\n",
            link(record.id),
            record.name,
            record.offset,
            record.terms,
            key,
            shift,
            record.status.text()
        ));
    }
    out.trim_end().to_string()
}

fn tally() -> String {
    let designs: usize = SPACES
        .iter()
        .map(|&(dimension, base)| {
            super::designs(dimension, base)
                .expect("the ledger spaces are walkable")
                .len()
        })
        .sum();
    format!(
        "The registry walks {designs} designs across {} dimension and base pairs and holds {} closed rows and {} convolved rows of {TERMS} terms each; the grid tiers render on demand within a budget of {BUDGET} cells a term.",
        SPACES.len(),
        keys(Tier::Closed).len(),
        keys(Tier::Convolved).len()
    )
}

fn fill(tag: &str) -> String {
    let record = |id: &str| record_by_id(id).expect("the placeholder names a record");
    match tag.split_once(' ') {
        Some(("terms", id)) => format!("`{}`", record(id).terms),
        Some(("formula", id)) => format!("`{}`, offset {}", record(id).formula, record(id).offset),
        _ => match tag {
            "sides" => table(Axis::Side),
            "levels" => table(Axis::Level),
            "records" => records(),
            "tally" => tally(),
            other => panic!("unknown placeholder {other}"),
        },
    }
}

/// Renders the sequences page: the prose with every generated table and number filled in.
pub fn markdown() -> String {
    let mut out = String::new();
    let mut rest = PROSE;
    while let Some(open) = rest.find("{{") {
        out.push_str(&rest[..open]);
        let close = open + rest[open..].find("}}").expect("every placeholder closes");
        out.push_str(&fill(rest[open + 2..close].trim()));
        rest = &rest[close + 2..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn the_page_on_disk_is_the_rendered_page() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../research/sequences.md");
        let disk = fs::read_to_string(&path).expect("the page is on disk");
        let page = markdown();
        assert!(!page.contains("{{"));
        assert!(page.contains(
            "`mrly_bang_d2_7.fills.side` | `3k^2 - 2k` | `8, 21, 40, 65, 96, 133, 176, 225`"
        ));
        assert_eq!(page, disk, "run `cargo run -p mrlylab --bin ledger`");
    }
}
