#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    H1(String),
    H2(String),
    Para(String),
    Bullets(Vec<String>),
    Numbers(Vec<String>),
    Code(Vec<String>),
}

pub fn blocks(md: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut items: Vec<String> = Vec::new();
    let mut ordered = false;
    let mut code: Option<Vec<String>> = None;
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
        flush_list(&mut out, &mut items, ordered);
        para.push(line.to_string());
    }
    if let Some(lines) = code {
        out.push(Block::Code(lines));
    }
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
        })
        .collect::<Vec<_>>()
        .join("\n")
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
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '-' || c == '_')
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
    fn refuses_deep_heading_and_quote() {
        let md = "### deep\n\n> quoted";
        let expected = "<p>### deep</p>\n<p>&gt; quoted</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_image_syntax() {
        let md = "![alt](https://evil.example/x.png)";
        let expected = "<p>![alt](https://evil.example/x.png)</p>";
        assert_eq!(html(md), expected);
    }

    #[test]
    fn refuses_bad_link_targets() {
        let md = "[x](javascript:alert(1)) and [y](../../etc/passwd)";
        let expected = "<p>[x](javascript:alert(1)) and [y](../../etc/passwd)</p>";
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
