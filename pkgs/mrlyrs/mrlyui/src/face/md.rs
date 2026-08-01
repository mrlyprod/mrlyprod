use super::layout::{row, Item, Op};
use super::text;
use crate::tokens::{Theme, INDENT, PAD, ROW};
use mrlycore::md::{blocks, Block};

pub(crate) fn items(md: &str, theme: &Theme, width: usize) -> Vec<Item> {
    let mut out = Vec::new();
    for block in blocks(md) {
        match block {
            Block::H1(t) => out.push(row(strip(&t), 0, 2, theme.ink)),
            Block::H2(t) => out.push(row(strip(&t), 0, 1, theme.accent)),
            Block::Para(t) => {
                for line in fold(words(&t), width) {
                    out.push(runs(&line, PAD, theme.ink));
                }
            }
            Block::Bullets(list) => {
                for entry in &list {
                    out.extend(list_item("-", entry, theme, width));
                }
            }
            Block::Numbers(list) => {
                for (i, entry) in list.iter().enumerate() {
                    out.extend(list_item(&format!("{}.", i + 1), entry, theme, width));
                }
            }
            Block::Code(lines) => out.push(code_item(&lines, theme, width)),
            Block::Image { alt, .. } => {
                for line in wrap(&strip(&alt), width) {
                    out.push(row(line, 0, 1, theme.muted));
                }
            }
            Block::Quote(t) => {
                for line in fold(words(&t), width) {
                    out.push(runs(&line, PAD, theme.muted));
                }
            }
            Block::Table { head, rows } => {
                out.push(table_line(&head, theme.accent));
                for cells in &rows {
                    out.push(table_line(cells, theme.ink));
                }
            }
        }
    }
    out
}

fn table_line(cells: &[String], color: [u8; 4]) -> Item {
    let joined = cells
        .iter()
        .map(|cell| strip(cell))
        .collect::<Vec<_>>()
        .join("  ");
    row(joined, 0, 1, color)
}

fn list_item(marker: &str, entry: &str, theme: &Theme, width: usize) -> Vec<Item> {
    let hang = PAD + INDENT;
    let field = width.saturating_sub(INDENT);
    let mut out = Vec::new();
    for (i, line) in fold(words(entry), field).into_iter().enumerate() {
        let mut item = runs(&line, hang, theme.ink);
        if i == 0 {
            item.ops
                .push(Op::text(PAD, 0, marker.to_string(), 1, theme.muted));
        }
        out.push(item);
    }
    out
}

fn runs(line: &[Word], x: usize, color: [u8; 4]) -> Item {
    let mut ops = Vec::new();
    let mut at = x;
    let mut i = 0;
    while i < line.len() {
        let link = line[i].1;
        let mut j = i;
        let mut run = String::new();
        while j < line.len() && line[j].1 == link {
            if j > i {
                run.push(' ');
            }
            run.push_str(&line[j].0);
            j += 1;
        }
        let w = text::width(&run, 1);
        ops.push(match link {
            true => Op::link(at, 0, run, 1, color),
            false => Op::text(at, 0, run, 1, color),
        });
        at += w + text::gap(1);
        i = j;
    }
    Item { height: ROW, ops }
}

fn code_item(lines: &[String], theme: &Theme, width: usize) -> Item {
    let shown = lines.len().max(1);
    let height = shown * ROW + 2;
    let mut ops = vec![Op::Rect {
        x: PAD,
        y: 0,
        w: width,
        h: height - 4,
        color: theme.faint,
    }];
    for (i, line) in lines.iter().enumerate() {
        ops.push(Op::text(
            PAD + 4,
            3 + i * ROW,
            text::truncate(line, width.saturating_sub(8), 1),
            1,
            theme.ink,
        ));
    }
    Item {
        height: height + 4,
        ops,
    }
}

pub(crate) type Word = (String, bool);

pub(crate) fn fold(words: Vec<Word>, field: usize) -> Vec<Vec<Word>> {
    let mut lines: Vec<Vec<Word>> = Vec::new();
    let mut line: Vec<Word> = Vec::new();
    let mut line_w = 0;
    for (word, link) in words {
        let word_w = text::width(&word, 1);
        let long = word_w > field;
        let cut = || (text::truncate(&word, field, 1), link);
        if line.is_empty() {
            if long {
                lines.push(vec![cut()]);
            } else {
                line_w = word_w;
                line.push((word, link));
            }
            continue;
        }
        if line_w + text::gap(1) + word_w <= field {
            line_w += text::gap(1) + word_w;
            line.push((word, link));
            continue;
        }
        lines.push(std::mem::take(&mut line));
        if long {
            lines.push(vec![cut()]);
            line_w = 0;
        } else {
            line_w = word_w;
            line.push((word, link));
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

pub(crate) fn wrap(paragraph: &str, field: usize) -> Vec<String> {
    let plain = paragraph
        .split_whitespace()
        .map(|w| (w.to_string(), false))
        .collect();
    fold(plain, field)
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|(w, _)| w)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

fn strip(text: &str) -> String {
    marked(text).0
}

fn marked(text: &str) -> (String, Vec<(usize, usize)>) {
    let mut out = String::new();
    let mut links = Vec::new();
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        if rest.starts_with("![") {
            out.push_str("![");
            i += 2;
            continue;
        }
        if rest.starts_with("**") {
            i += 2;
            continue;
        }
        if rest.starts_with('*') {
            i += 1;
            continue;
        }
        if rest.starts_with('[') {
            if let Some((len, inner)) = link_text(rest) {
                let at = out.len();
                out.push_str(&strip(&inner));
                links.push((at, out.len()));
                i += len;
                continue;
            }
        }
        let ch = rest.chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    (out, links)
}

pub(crate) fn words(text: &str) -> Vec<Word> {
    let (flat, links) = marked(text);
    let mut out = Vec::new();
    let mut at = 0;
    for word in flat.split_whitespace() {
        let start = flat[at..].find(word).map_or(at, |i| at + i);
        let end = start + word.len();
        let link = links.iter().any(|(a, b)| start < *b && end > *a);
        out.push((word.to_string(), link));
        at = end;
    }
    out
}

fn link_text(rest: &str) -> Option<(usize, String)> {
    let close = rest.find("](")?;
    let end = rest[close + 2..].find(')')?;
    Some((close + 2 + end + 1, rest[1..close].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_blocks_degrade_to_text_lines() {
        let theme = Theme::new("pages", false);
        let md =
            "![the goose](a/goose.png)\n\n| h1 | h2 |\n|---|---|\n| **a** | b |\n\n> so it *goes*";
        let out = items(md, &theme, crate::tokens::CONTENT);
        assert_eq!(out.len(), 4);
        let texts: Vec<String> = out
            .iter()
            .flat_map(|item| &item.ops)
            .filter_map(|op| match op {
                Op::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(texts, vec!["the goose", "h1  h2", "a  b", "so it goes"]);
    }

    #[test]
    fn a_link_run_carries_the_flag() {
        let theme = Theme::new("pages", false);
        let out = items("Prose with a [link](https://mrly.net) inside.", &theme, 300);
        let unders: Vec<String> = out
            .iter()
            .flat_map(|i| i.ops.iter())
            .filter_map(|op| match op {
                Op::Text {
                    text, under: true, ..
                } => Some(text.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(unders, vec!["link".to_string()]);
        let plain: String = out
            .iter()
            .flat_map(|i| i.ops.iter())
            .filter_map(|op| match op {
                Op::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(plain, "Prose with a link inside.");
    }

    #[test]
    fn strip_flattens_emphasis_and_links() {
        assert_eq!(strip("use **bold** and *italic*"), "use bold and italic");
        assert_eq!(strip("see [the site](https://mrly.net)"), "see the site");
        assert_eq!(strip("![alt](x.png)"), "![alt](x.png)");
    }

    #[test]
    fn wrap_breaks_on_word_boundaries() {
        let lines = wrap("one two three four five six seven eight nine ten", 80);
        assert!(lines.len() > 1);
        for line in &lines {
            assert!(text::width(line, 1) <= 80);
        }
    }

    #[test]
    fn wrap_truncates_an_overlong_word() {
        let lines = wrap("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 40);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].ends_with(".."));
    }
}
