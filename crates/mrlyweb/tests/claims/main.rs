mod checks;
mod ledger;
mod registry;
mod report;

use registry::{Cost, Verdict};
use report::Run;
use std::path::Path;

#[test]
fn the_ledger_claims_hold() {
    let all = std::env::var("MRLY_CLAIMS")
        .map(|name| name == "all")
        .unwrap_or(false);
    let lane = if all { Cost::Dear } else { Cost::Cheap };
    let page = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../research/DISCOVERIES.md");
    let text = std::fs::read_to_string(&page).expect("the ledger is readable");
    let book = match ledger::parse(&text) {
        Ok(book) => book,
        Err(why) => panic!("{why}"),
    };
    let mut runs = Vec::new();
    for &(key, cost, check) in registry::entries() {
        let outcome = match (cost, lane) {
            (Cost::Dear, Cost::Cheap) => None,
            _ => Some(check()),
        };
        runs.push(Run { key, outcome });
    }
    let sheet = report::tally(&book, &runs);
    report::print(&sheet);
    assert_eq!(
        sheet.count(Verdict::Failed),
        0,
        "the ledger holds failed claims"
    );
    assert_eq!(
        sheet.count(Verdict::Orphan),
        0,
        "the registry holds orphan checks"
    );
}
