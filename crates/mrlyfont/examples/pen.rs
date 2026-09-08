use mrlyfont::letters::{digits, extras, lowers, specials, uppers};
use mrlyfont::{draft, floor, glyph, strokes, trim, Glyph};

fn key(c: char) -> String {
    if c.is_ascii() {
        format!("{c:?}")
    } else {
        format!("'\\u{{{:04x}}}'", c as u32)
    }
}

fn tokens(stroke: &[(usize, usize)]) -> String {
    let cells: Vec<String> = stroke.iter().map(|(r, c)| format!("{r}{c}")).collect();
    cells.join(" ")
}

fn entry(c: char, strokes: &[Vec<(usize, usize)>]) -> String {
    if strokes.is_empty() {
        return format!("    ({}, &[]),\n", key(c));
    }
    let mut out = format!("    ({}, &[\n", key(c));
    for stroke in strokes {
        out.push_str(&format!("        \"{}\",\n", tokens(stroke)));
    }
    out.push_str("    ]),\n");
    out
}

fn table(name: &str, doc: &str, glyphs: &[Glyph]) -> String {
    let mut out =
        format!("/// The pens of the {doc}.\n#[rustfmt::skip]\npub const {name}: &[Pen] = &[\n");
    for g in glyphs {
        out.push_str(&entry(g.char, &strokes(g.char)));
    }
    out.push_str("];\n");
    out
}

fn main() {
    match std::env::args().nth(1).and_then(|a| a.chars().next()) {
        Some(c) => {
            let Some(g) = glyph(c) else {
                eprintln!("{c} is not in the font");
                std::process::exit(1);
            };
            let rows = trim(&g.rows);
            print!("{}", entry(c, &draft(&rows)));
            println!("floor {}", floor(&rows));
        }
        None => {
            let tables = [
                ("UPPERS", "twenty-six uppers", uppers()),
                ("LOWERS", "twenty-six lowers", lowers()),
                ("DIGITS", "ten digits", digits()),
                ("EXTRAS", "punctuation, symbol and arrow glyphs", extras()),
                ("SPECIALS", "four seven-row specials", specials()),
            ];
            let out: Vec<String> = tables.iter().map(|(n, d, g)| table(n, d, g)).collect();
            print!("{}", out.join("\n"));
        }
    }
}
