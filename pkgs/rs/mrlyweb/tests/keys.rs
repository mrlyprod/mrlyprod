use mrlyos::kernel::Iden;
use mrlyweb::registry::catalogue;
use std::collections::BTreeSet;

const DIRS: [&str; 4] = ["up", "down", "left", "right"];

#[test]
fn bound_keys_name_dirs_and_live_verbs() {
    let iden = Iden::new("guest");
    for app in catalogue() {
        let manifest = app.manifest();
        if manifest.keys.is_empty() {
            continue;
        }
        let names: Vec<String> = app.actions(&iden).iter().map(|v| v.name.clone()).collect();
        for (dir, call) in &manifest.keys {
            let route = app.route();
            assert!(DIRS.contains(&dir.as_str()), "{route} binds {dir}");
            assert_eq!(call.now, None, "{route} stamps a time on {dir}");
            assert!(
                names.contains(&call.verb),
                "{route} binds {dir} to {}, which it does not offer",
                call.verb
            );
        }
    }
}

#[test]
fn the_arcade_five_are_the_only_keyed_apps() {
    let bound: BTreeSet<String> = catalogue()
        .into_iter()
        .filter(|a| !a.manifest().keys.is_empty())
        .map(|a| a.route().to_string())
        .collect();
    let want: BTreeSet<String> = ["crush", "escape", "snake", "tennis", "twenty48"]
        .iter()
        .map(|r| r.to_string())
        .collect();
    assert_eq!(bound, want);
}
