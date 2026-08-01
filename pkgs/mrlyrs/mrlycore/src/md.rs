#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    H1(String),
    H2(String),
    Para(String),
    Bullets(Vec<String>),
    Numbers(Vec<String>),
    Code(Vec<String>),
    Image {
        alt: String,
        path: String,
    },
    Table {
        head: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Quote(String),
}

pub fn blocks(md: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut items: Vec<String> = Vec::new();
    let mut ordered = false;
    let mut code: Option<Vec<String>> = None;
    let mut table: Vec<String> = Vec::new();
    let mut quote: Vec<String> = Vec::new();
    for raw in md.lines() {
        let line = raw.trim_end();
        if let Some(lines) = code.as_mut() {
            if line.starts_with("```") {
                out.push(Block::Code(std::mem::take(lines)));
                code = None;
            } else {
                lines.push(line.to_string());
            }
            continue;
        }
        if !table.is_empty() && !line.starts_with('|') {
            flush_table(&mut out, &mut table, &mut para);
        }
        if !quote.is_empty() && !line.starts_with('>') {
            flush_quote(&mut out, &mut quote);
        }
        if line.starts_with("```") {
            flush_para(&mut out, &mut para);
            flush_list(&mut out, &mut items, ordered);
            code = Some(Vec::new());
            continue;
        }
        if line.is_empty() {
            flush_para(&mut out, &mut para);
            flush_list(&mut out, &mut items, ordered);
            continue;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            flush_para(&mut out, &mut para);
            flush_list(&mut out, &mut items, ordered);
            out.push(Block::H2(rest.to_string()));
            continue;
        }
        if let Some(rest) = line.strip_prefix("# ") {
            flush_para(&mut out, &mut para);
            flush_list(&mut out, &mut items, ordered);
            out.push(Block::H1(rest.to_string()));
            continue;
        }
        if let Some(rest) = line.strip_prefix("- ") {
            flush_para(&mut out, &mut para);
            if ordered {
                flush_list(&mut out, &mut items, ordered);
            }
            ordered = false;
            items.push(rest.to_string());
            continue;
        }
        if let Some(rest) = numbered(line) {
            flush_para(&mut out, &mut para);
            if !ordered {
                flush_list(&mut out, &mut items, ordered);
            }
            ordered = true;
            items.push(rest.to_string());
            continue;
        }
        if let Some((alt, path)) = image(line) {
            flush_para(&mut out, &mut para);
            flush_list(&mut out, &mut items, ordered);
            out.push(Block::Image { alt, path });
            continue;
        }
        if line.starts_with('|') {
            flush_list(&mut out, &mut items, ordered);
            table.push(line.to_string());
            continue;
        }
        if let Some(rest) = line.strip_prefix('>') {
            flush_para(&mut out, &mut para);
            flush_list(&mut out, &mut items, ordered);
            quote.push(rest.strip_prefix(' ').unwrap_or(rest).to_string());
            continue;
        }
        flush_list(&mut out, &mut items, ordered);
        para.push(line.to_string());
    }
    if let Some(lines) = code {
        out.push(Block::Code(lines));
    }
    flush_table(&mut out, &mut table, &mut para);
    flush_quote(&mut out, &mut quote);
    flush_para(&mut out, &mut para);
    flush_list(&mut out, &mut items, ordered);
    out
}

pub fn html(md: &str) -> String {
    blocks(md)
        .iter()
        .map(|block| match block {
            Block::H1(text) => format!("<h1>{}</h1>", inline(text)),
            Block::H2(text) => format!("<h2>{}</h2>", inline(text)),
            Block::Para(text) => format!("<p>{}</p>", inline(text)),
            Block::Bullets(items) => list_html("ul", items),
            Block::Numbers(items) => list_html("ol", items),
            Block::Code(lines) => format!(
                "<pre><code>{}</code></pre>",
                lines
                    .iter()
                    .map(|line| escape(line))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Block::Image { alt, path } => format!(
                "<img data-slug=\"{}\" alt=\"{}\">",
                escape(path),
                escape(alt)
            ),
            Block::Table { head, rows } => table_html(head, rows),
            Block::Quote(text) => format!("<blockquote><p>{}</p></blockquote>", inline(text)),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn table_html(head: &[String], rows: &[Vec<String>]) -> String {
    let ths = head
        .iter()
        .map(|cell| format!("<th>{}</th>", inline(cell)))
        .collect::<String>();
    let body = rows
        .iter()
        .map(|row| {
            let tds = row
                .iter()
                .map(|cell| format!("<td>{}</td>", inline(cell)))
                .collect::<String>();
            format!("<tr>{tds}</tr>")
        })
        .collect::<String>();
    format!("<table><thead><tr>{ths}</tr></thead><tbody>{body}</tbody></table>")
}

fn list_html(tag: &str, items: &[String]) -> String {
    let list = items
        .iter()
        .map(|item| format!("<li>{}</li>", inline(item)))
        .collect::<String>();
    format!("<{tag}>{list}</{tag}>")
}

fn flush_para(blocks: &mut Vec<Block>, para: &mut Vec<String>) {
    if para.is_empty() {
        return;
    }
    blocks.push(Block::Para(para.join(" ")));
    para.clear();
}

fn flush_list(blocks: &mut Vec<Block>, items: &mut Vec<String>, ordered: bool) {
    if items.is_empty() {
        return;
    }
    let taken = std::mem::take(items);
    blocks.push(if ordered {
        Block::Numbers(taken)
    } else {
        Block::Bullets(taken)
    });
}

fn flush_table(blocks: &mut Vec<Block>, lines: &mut Vec<String>, para: &mut Vec<String>) {
    if lines.is_empty() {
        return;
    }
    let taken = std::mem::take(lines);
    match table_rows(&taken) {
        Some((head, rows)) => {
            flush_para(blocks, para);
            blocks.push(Block::Table { head, rows });
        }
        None => para.extend(taken),
    }
}

fn flush_quote(blocks: &mut Vec<Block>, lines: &mut Vec<String>) {
    if lines.is_empty() {
        return;
    }
    let text = std::mem::take(lines)
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    blocks.push(Block::Quote(text));
}

fn table_rows(lines: &[String]) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    if lines.len() < 2 {
        return None;
    }
    let head = table_cells(&lines[0])?;
    let sep = table_cells(&lines[1])?;
    if sep.len() != head.len() || !sep.iter().all(|cell| separator(cell)) {
        return None;
    }
    let mut rows = Vec::new();
    for line in &lines[2..] {
        let cells = table_cells(line)?;
        if cells.len() != head.len() {
            return None;
        }
        rows.push(cells);
    }
    Some((head, rows))
}

fn table_cells(line: &str) -> Option<Vec<String>> {
    let inner = line.strip_prefix('|')?.strip_suffix('|')?;
    Some(
        inner
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect(),
    )
}

fn separator(cell: &str) -> bool {
    let dashes = cell.strip_prefix(':').unwrap_or(cell);
    let dashes = dashes.strip_suffix(':').unwrap_or(dashes);
    !dashes.is_empty() && dashes.chars().all(|c| c == '-')
}

fn image(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("![")?;
    let close = rest.find("](")?;
    let alt = &rest[..close];
    let path = rest[close + 2..].strip_suffix(')')?;
    let path = path.strip_prefix("./").unwrap_or(path);
    if path.is_empty() {
        return None;
    }
    if !path
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '-' || c == '_' || c == '.')
    {
        return None;
    }
    Some((alt.to_string(), path.to_string()))
}

fn numbered(line: &str) -> Option<&str> {
    let digits = line.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    line[digits..].strip_prefix(". ")
}

fn inline(text: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        if let Some(stripped) = rest.strip_prefix('`') {
            if let Some(end) = stripped.find('`') {
                if end > 0 {
                    out.push_str("<code>");
                    out.push_str(&escape(&stripped[..end]));
                    out.push_str("</code>");
                    i += end + 2;
                    continue;
                }
            }
            out.push('`');
            i += 1;
            continue;
        }
        if rest.starts_with("![") {
            out.push_str("![");
            i += 2;
            continue;
        }
        if let Some(stripped) = rest.strip_prefix("**") {
            if let Some(end) = stripped.find("**") {
                if end > 0 {
                    out.push_str("<strong>");
                    out.push_str(&inline(&stripped[..end]));
                    out.push_str("</strong>");
                    i += end + 4;
                    continue;
                }
            }
            out.push_str("**");
            i += 2;
            continue;
        }
        if let Some(stripped) = rest.strip_prefix('*') {
            if let Some(end) = stripped.find('*') {
                if end > 0 {
                    out.push_str("<em>");
                    out.push_str(&inline(&stripped[..end]));
                    out.push_str("</em>");
                    i += end + 2;
                    continue;
                }
            }
            out.push('*');
            i += 1;
            continue;
        }
        if rest.starts_with('[') {
            if let Some((len, html)) = link(rest) {
                out.push_str(&html);
                i += len;
                continue;
            }
            out.push('[');
            i += 1;
            continue;
        }
        let ch = rest.chars().next().unwrap();
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
        i += ch.len_utf8();
    }
    out
}

fn link(rest: &str) -> Option<(usize, String)> {
    let close = rest.find("](")?;
    let end = rest[close + 2..].find(')')?;
    let text = &rest[1..close];
    let url = &rest[close + 2..close + 2 + end];
    let len = close + 2 + end + 1;
    if url.starts_with("http://") || url.starts_with("https://") {
        let html = format!(
            "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>",
            escape(url),
            inline(text)
        );
        return Some((len, html));
    }
    let slug = url.strip_prefix("./").unwrap_or(url);
    let slug = slug.strip_suffix(".md").unwrap_or(slug);
    if slug.is_empty() {
        return None;
    }
    if !slug
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '-' || c == '_' || c == '.')
    {
        return None;
    }
    let html = format!("<a data-slug=\"{}\">{}</a>", slug, inline(text));
    Some((len, html))
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_and_prose() {
        let md = "# Title\n\nHello world.\n\n## Section\n\nMore prose\nacross lines.";
        let expected = "<h1>Title</h1>\n<p>Hello world.</p>\n<h2>Section</h2>\n<p>More prose across lines.</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn lists() {
        let md = "- one\n- two\n\n1. first\n2. second";
        let expected = "<ul><li>one</li><li>two</li></ul>\n<ol><li>first</li><li>second</li></ol>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn list_type_switch_without_blank() {
        let md = "- one\n1. first";
        let expected = "<ul><li>one</li></ul>\n<ol><li>first</li></ol>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn emphasis() {
        let md = "Use **bold** and *italic* here.";
        let expected = "<p>Use <strong>bold</strong> and <em>italic</em> here.</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn absolute_link() {
        let md = "See [the site](https://mrly.net/snake) now.";
        let expected = "<p>See <a href=\"https://mrly.net/snake\" target=\"_blank\" rel=\"noopener\">the site</a> now.</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn relative_link_becomes_slug() {
        let md = "Read [privacy](./privacy.md) and [terms](terms).";
        let expected = "<p>Read <a data-slug=\"privacy\">privacy</a> and <a data-slug=\"terms\">terms</a>.</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn code_fence() {
        let md = "```\nlet x = 1 < 2;\n```";
        let expected = "<pre><code>let x = 1 &lt; 2;</code></pre>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn unclosed_fence_still_emits() {
        let md = "```\ntrailing";
        let expected = "<pre><code>trailing</code></pre>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_inline_html() {
        let md = "hello <script>alert(1)</script>";
        let expected = "<p>hello &lt;script&gt;alert(1)&lt;/script&gt;</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_deep_heading() {
        let md = "### deep";
        let expected = "<p>### deep</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_remote_image() {
        let md = "![alt](https://evil.example/x.png)";
        let expected = "<p>![alt](https://evil.example/x.png)</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_protocol_image() {
        let md = "![x](javascript:alert(1))";
        let expected = "<p>![x](javascript:alert(1))</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_inline_image() {
        let md = "see ![alt](x.png) here";
        let expected = "<p>see ![alt](x.png) here</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn image_block() {
        let md = "![the goose](./assets/goose.png)";
        let expected = "<img data-slug=\"assets/goose.png\" alt=\"the goose\">";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn image_escapes_alt() {
        let md = "![a \"b\" & c](../img/x.png)";
        let expected = "<img data-slug=\"../img/x.png\" alt=\"a &quot;b&quot; &amp; c\">";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn table() {
        let md = "| name | note |\n|---|---|\n| **goose** | see [home](./home.md) |\n| b | c |";
        let expected = "<table><thead><tr><th>name</th><th>note</th></tr></thead><tbody><tr><td><strong>goose</strong></td><td>see <a data-slug=\"home\">home</a></td></tr><tr><td>b</td><td>c</td></tr></tbody></table>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn table_tolerates_colons() {
        let md = "| a | b |\n|:---|---:|\n| 1 | 2 |";
        let expected = "<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody><tr><td>1</td><td>2</td></tr></tbody></table>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn table_headers_alone() {
        let md = "| a | b |\n|---|---|";
        let expected = "<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody></tbody></table>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn ragged_table_degrades() {
        let md = "| a | b |\n|---|---|\n| only |";
        let expected = "<p>| a | b | |---|---| | only |</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn unpiped_table_degrades() {
        let md = "| a | b\n|---|---|\n| 1 | 2 |";
        let expected = "<p>| a | b |---|---| | 1 | 2 |</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn degraded_table_stays_in_its_paragraph() {
        let md = "before\n| a | b |\nafter";
        let expected = "<p>before | a | b | after</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn quote() {
        let md = "> two roads\n> diverged in a *yellow* wood";
        let expected =
            "<blockquote><p>two roads diverged in a <em>yellow</em> wood</p></blockquote>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn blank_line_splits_quotes() {
        let md = "> one\n\n>two";
        let expected = "<blockquote><p>one</p></blockquote>\n<blockquote><p>two</p></blockquote>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn quote_interrupts_paragraph() {
        let md = "prose\n> quoted\nmore";
        let expected = "<p>prose</p>\n<blockquote><p>quoted</p></blockquote>\n<p>more</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn math_asterisks_survive_when_unpaired() {
        let md = "n^2*(4n+3) stays put";
        let expected = "<p>n^2*(4n+3) stays put</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn code_span() {
        let md = "run `cargo test` now";
        let expected = "<p>run <code>cargo test</code> now</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn code_span_shields_math() {
        let md = "`a*b*c` and `n^2*(4n+3)`";
        let expected = "<p><code>a*b*c</code> and <code>n^2*(4n+3)</code></p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn code_span_escapes_and_stays_literal() {
        let md = "`<b> [x](y) **z**`";
        let expected = "<p><code>&lt;b&gt; [x](y) **z**</code></p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn unclosed_backtick_stays_literal() {
        let md = "a `dangling tick";
        let expected = "<p>a `dangling tick</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_protocol_links_but_keeps_dotted_paths() {
        let md = "[x](javascript:alert(1)) and [y](../../docs/LAYERS.md)";
        let expected =
            "<p>[x](javascript:alert(1)) and <a data-slug=\"../../docs/LAYERS\">y</a></p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_table() {
        let md = "| a | b |";
        let expected = "<p>| a | b |</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn dummy_document() {
        let md = "# Dummy\n\nA fixture for the pages app.\n\n## Checklist\n\n- render **bold**\n- render *italic*\n- follow [home](./home.md)\n\n1. parse\n2. render\n\n```\ncargo test -p mrly\n```\n\nVisit [mrly](https://mrly.net).";
        let expected = "<h1>Dummy</h1>\n<p>A fixture for the pages app.</p>\n<h2>Checklist</h2>\n<ul><li>render <strong>bold</strong></li><li>render <em>italic</em></li><li>follow <a data-slug=\"home\">home</a></li></ul>\n<ol><li>parse</li><li>render</li></ol>\n<pre><code>cargo test -p mrly</code></pre>\n<p>Visit <a href=\"https://mrly.net\" target=\"_blank\" rel=\"noopener\">mrly</a>.</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn blocks_carry_raw_text() {
        let md =
            "# Title\n\nkeep **stars** raw\nacross lines\n\n- item *one*\n\n```\ncode < here\n```";
        let expected = vec![
            Block::H1("Title".to_string()),
            Block::Para("keep **stars** raw across lines".to_string()),
            Block::Bullets(vec!["item *one*".to_string()]),
            Block::Code(vec!["code < here".to_string()]),
        ];
        assert_eq!(blocks(md), expected);
    }

    #[test]
    fn blocks_carry_new_variants_raw() {
        let md = "![alt](a/b.png)\n\n| h |\n|---|\n| *v* |\n\n> said **so**";
        let expected = vec![
            Block::Image {
                alt: "alt".to_string(),
                path: "a/b.png".to_string(),
            },
            Block::Table {
                head: vec!["h".to_string()],
                rows: vec![vec!["*v*".to_string()]],
            },
            Block::Quote("said **so**".to_string()),
        ];
        assert_eq!(blocks(md), expected);
    }

    #[test]
    fn blocks_degrade_like_html() {
        let md = "### deep\n1. first\n- one\n```\nopen";
        let expected = vec![
            Block::Para("### deep".to_string()),
            Block::Numbers(vec!["first".to_string()]),
            Block::Bullets(vec!["one".to_string()]),
            Block::Code(vec!["open".to_string()]),
        ];
        assert_eq!(blocks(md), expected);
    }
}
