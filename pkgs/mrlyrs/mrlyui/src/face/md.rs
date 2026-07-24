use super::layout::{row, Item, Op, FIELD, INDENT, PAD, ROW};
use super::text;
use super::Theme;
use mrlycore::md::{blocks, Block};

pub(crate) fn items(md: &str, theme: &Theme) -> Vec<Item> {
    let mut out = Vec::new();
    for block in blocks(md) {
        match block {
            Block::H1(t) => out.push(row(strip(&t), 0, 2, theme.ink)),
            Block::H2(t) => out.push(row(strip(&t), 0, 1, theme.accent)),
            Block::Para(t) => {
                for line in wrap(&strip(&t), FIELD) {
                    out.push(row(line, 0, 1, theme.ink));
                }
            }
            Block::Bullets(list) => {
                for entry in &list {
                    out.extend(list_item("-", &strip(entry), theme));
                }
            }
            Block::Numbers(list) => {
                for (i, entry) in list.iter().enumerate() {
                    out.extend(list_item(&format!("{}.", i + 1), &strip(entry), theme));
                }
            }
            Block::Code(lines) => out.push(code_item(&lines, theme)),
        }
    }
    out
}

fn list_item(marker: &str, entry: &str, theme: &Theme) -> Vec<Item> {
    let hang = PAD + INDENT;
    let field = (super::WIDTH - PAD).saturating_sub(hang);
    let mut out = Vec::new();
    for (i, line) in wrap(entry, field).into_iter().enumerate() {
        let mut ops = vec![Op::Text {
            x: hang,
            y: 0,
            text: line,
            scale: 1,
            color: theme.ink,
        }];
        if i == 0 {
            ops.push(Op::Text {
                x: PAD,
                y: 0,
                text: marker.to_string(),
                scale: 1,
                color: theme.muted,
            });
        }
        out.push(Item { height: ROW, ops });
    }
    out
}

fn code_item(lines: &[String], theme: &Theme) -> Item {
    let shown = lines.len().max(1);
    let height = shown * ROW + 2;
    let mut ops = vec![Op::Rect {
        x: PAD,
        y: 0,
        w: FIELD,
        h: height - 4,
        color: theme.faint,
    }];
    for (i, line) in lines.iter().enumerate() {
        ops.push(Op::Text {
            x: PAD + 4,
            y: 3 + i * ROW,
            text: text::truncate(line, FIELD - 8, 1),
            scale: 1,
            color: theme.ink,
        });
    }
    Item {
        height: height + 4,
        ops,
    }
}

fn wrap(paragraph: &str, field: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut line_w = 0;
    for word in paragraph.split_whitespace() {
        let word_w = text::width(word, 1);
        if line.is_empty() {
            if word_w > field {
                lines.push(text::truncate(word, field, 1));
            } else {
                line = word.to_string();
                line_w = word_w;
            }
            continue;
        }
        if line_w + 5 + word_w <= field {
            line.push(' ');
            line.push_str(word);
            line_w += 5 + word_w;
        } else {
            lines.push(std::mem::take(&mut line));
            if word_w > field {
                lines.push(text::truncate(word, field, 1));
                line_w = 0;
            } else {
                line = word.to_string();
                line_w = word_w;
            }
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn strip(text: &str) -> String {
    let mut out = String::new();
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
                out.push_str(&strip(&inner));
                i += len;
                continue;
            }
        }
        let ch = rest.chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
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
