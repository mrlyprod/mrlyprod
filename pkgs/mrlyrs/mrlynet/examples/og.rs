use std::fs;
use std::process::exit;

fn main() {
    let Some(outdir) = std::env::args().nth(1) else {
        eprintln!("usage: og <outdir>");
        exit(1);
    };
    fs::create_dir_all(&outdir).unwrap();
    let mut count = 0;
    for app in mrlynet::registry::catalogue() {
        let m = app.manifest();
        if m.hidden {
            continue;
        }
        let png = mrlynet::card::card_png(&m.route, &m.title).unwrap();
        fs::write(format!("{outdir}/{}.png", m.route), png).unwrap();
        count += 1;
    }
    let png = mrlynet::card::card_png("home", "mrlyprod").unwrap();
    fs::write(format!("{outdir}/home.png"), png).unwrap();
    if let Ok(entries) = fs::read_dir("cdn/pages") {
        let mut slugs: Vec<String> = entries
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .filter_map(|name| name.strip_suffix(".md").map(str::to_string))
            .collect();
        slugs.sort();
        for slug in slugs {
            let md = fs::read_to_string(format!("cdn/pages/{slug}.md")).unwrap();
            let title = md
                .lines()
                .find_map(|line| line.strip_prefix("# "))
                .unwrap_or("MrlyProd")
                .to_string();
            let png = mrlynet::card::card_png(&format!("pages/{slug}"), &title).unwrap();
            fs::write(format!("{outdir}/pages-{slug}.png"), png).unwrap();
            count += 1;
        }
    }
    println!("wrote {count} og cards to {outdir}");
}
