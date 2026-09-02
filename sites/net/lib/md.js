const ESC = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" };
const SEP = /^\|?\s*:?-+:?\s*(\|\s*:?-+:?\s*)*\|?\s*$/;
const BLOCK = /^(#{1,6} |```|\$\$|- |\d+\. |> |\|)/;

export const escape = (s) => String(s).replace(/[&<>"]/g, (c) => ESC[c]);

export function slug(text) {
  return text.toLowerCase().replace(/[^\p{L}\p{N} _-]/gu, "").replace(/ /g, "-");
}

const TEX = { lceil: "\u2308", rceil: "\u2309", lfloor: "\u230a", rfloor: "\u230b", ne: "\u2260", neq: "\u2260", le: "\u2264", leq: "\u2264", ge: "\u2265", geq: "\u2265", times: "\u00d7", infty: "\u221e", pi: "\u03c0", cdot: "\u00b7", to: "\u2192", left: "", right: "" };

export function plain(s) {
  return s
    .replace(/\\([a-zA-Z]+)\s?/g, (m, name) => (name in TEX ? TEX[name] : m))
    .replace(/\s+([\u2309\u230b])/g, "$1")
    .replace(/!\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/`([^`]*)`/g, "$1")
    .replace(/\*\*([^*]*)\*\*/g, "$1")
    .replace(/\*([^*]*)\*/g, "$1")
    .replace(/\$\$?([^$]*)\$\$?/g, "$1")
    .replace(/\s+/g, " ")
    .trim();
}

export function title(md) {
  const m = md.match(/^# (.+)$/m);
  return m ? plain(m[1]) : "";
}

export function summary(md, max = 200) {
  const skip = /^(#{1,6} |```|\$\$|\||!\[)/;
  const line = md.split("\n").find((l) => l.trim() && !skip.test(l)) ?? "";
  const text = plain(line.replace(/^(- |\d+\. |> )/, ""));
  if (text.length <= max) return text;
  const cut = text.lastIndexOf(" ", max);
  return text.slice(0, cut > 0 ? cut : max);
}

function bracket(src, i) {
  let depth = 0;
  let j = i;
  for (; j < src.length; j++) {
    if (src[j] === "[") depth++;
    else if (src[j] === "]" && --depth === 0) break;
  }
  if (j >= src.length || src[j + 1] !== "(") return null;
  let k = j + 2;
  for (depth = 1; k < src.length; k++) {
    if (src[k] === "(") depth++;
    else if (src[k] === ")" && --depth === 0) break;
  }
  if (k >= src.length) return null;
  return { text: src.slice(i + 1, j), url: src.slice(j + 2, k).trim(), end: k + 1 };
}

const WORD = /[\p{L}\p{N}]/u;
const blank = (c) => c === undefined || c === " " || c === "\n";

function closing(src, from, mark) {
  if (blank(src[from]) || WORD.test(src[from - mark.length - 1] ?? " ")) return -1;
  let j = src.indexOf(mark, from + 1);
  while (j > 0 && (blank(src[j - 1]) || WORD.test(src[j + mark.length] ?? " "))) j = src.indexOf(mark, j + 1);
  return j;
}

const unescape = (tex) => tex.replace(/\\\\([!-\/:-@[-`{-~])/g, "\\$1");

export function inline(src, ctx) {
  let out = "";
  let i = 0;
  while (i < src.length) {
    const c = src[i];
    if (c === "`") {
      const j = src.indexOf("`", i + 1);
      if (j > i + 1) {
        out += "<code>" + escape(src.slice(i + 1, j)) + "</code>";
        i = j + 1;
        continue;
      }
    } else if (c === "$") {
      const display = src[i + 1] === "$";
      const mark = display ? "$$" : "$";
      const j = src.indexOf(mark, i + mark.length);
      if (j > i + mark.length - 1 && src.slice(i + mark.length, j).trim()) {
        out += ctx.math(unescape(src.slice(i + mark.length, j)), display);
        i = j + mark.length;
        continue;
      }
    } else if (c === "!" && src[i + 1] === "[") {
      const m = bracket(src, i + 1);
      if (m) {
        out += `<img src="${escape(ctx.link(m.url))}" alt="${escape(m.text)}">`;
        i = m.end;
        continue;
      }
    } else if (c === "[") {
      const m = bracket(src, i);
      if (m) {
        out += `<a href="${escape(ctx.link(m.url))}">${inline(m.text, ctx)}</a>`;
        i = m.end;
        continue;
      }
    } else if (c === "*") {
      const strong = src[i + 1] === "*";
      const mark = strong ? "**" : "*";
      const j = closing(src, i + mark.length, mark);
      if (j > 0) {
        const tag = strong ? "strong" : "em";
        out += `<${tag}>${inline(src.slice(i + mark.length, j), ctx)}</${tag}>`;
        i = j + mark.length;
        continue;
      }
    }
    out += ESC[c] ?? c;
    i++;
  }
  return out;
}

function cells(row) {
  const trimmed = row.trim().replace(/^\|/, "").replace(/\|$/, "");
  return trimmed.split(/(?<!\\)\|/).map((c) => c.trim().replace(/\\\|/g, "|"));
}

function table(rows, ctx) {
  const align = cells(rows[1]).map((c) => {
    const l = c.startsWith(":");
    const r = c.endsWith(":");
    return l && r ? "center" : r ? "right" : l ? "left" : "";
  });
  const cell = (tag, text, k) => {
    const cls = align[k] && align[k] !== "left" ? ` class="${align[k]}"` : "";
    return `<${tag}${cls}>${inline(text, ctx)}</${tag}>`;
  };
  const head = cells(rows[0]).map((c, k) => cell("th", c, k)).join("");
  const body = rows
    .slice(2)
    .map((r) => "<tr>" + cells(r).map((c, k) => cell("td", c, k)).join("") + "</tr>")
    .join("\n");
  return `<div class="table"><table><thead><tr>${head}</tr></thead><tbody>\n${body}\n</tbody></table></div>`;
}

function heading(line, ctx) {
  const level = line.indexOf(" ");
  const text = line.slice(level + 1).trim();
  const base = slug(plain(text));
  const seen = ctx.ids.get(base) ?? 0;
  ctx.ids.set(base, seen + 1);
  const id = seen ? `${base}-${seen}` : base;
  return `<h${level} id="${id}">${inline(text, ctx)}</h${level}>`;
}

function paragraph(text, ctx) {
  const image = text.match(/^!\[([^\]]*)\]\(([^)]*)\)$/);
  if (image) {
    const src = escape(ctx.link(image[2].trim()));
    return `<figure><img src="${src}" alt="${escape(image[1])}"><figcaption>${inline(image[1], ctx)}</figcaption></figure>`;
  }
  return `<p>${inline(text, ctx)}</p>`;
}

export function render(md, opts = {}) {
  const ctx = {
    link: opts.link ?? ((u) => u),
    math: opts.math ?? ((t) => `<code>${escape(t)}</code>`),
    ids: new Map(),
  };
  const lines = md.replace(/\r\n?/g, "\n").split("\n");
  const out = [];
  let i = 0;
  const run = (test) => {
    const start = i;
    while (i < lines.length && test(lines[i])) i++;
    return lines.slice(start, i);
  };
  while (i < lines.length) {
    const line = lines[i];
    if (line.startsWith("```")) {
      i++;
      const code = run((l) => !l.startsWith("```"));
      i++;
      out.push(`<pre><code>${escape(code.join("\n"))}</code></pre>`);
    } else if (line.startsWith("$$")) {
      const single = line.trim().length > 4 && line.trim().endsWith("$$");
      let tex;
      if (single) {
        tex = line.trim().slice(2, -2);
        i++;
      } else {
        i++;
        const body = run((l) => !l.trim().endsWith("$$"));
        const last = i < lines.length ? lines[i].trim().slice(0, -2) : "";
        i++;
        tex = line.slice(2) + "\n" + body.join("\n") + "\n" + last;
      }
      out.push(ctx.math(unescape(tex.trim()), true));
    } else if (/^#{1,6} /.test(line)) {
      out.push(heading(line, ctx));
      i++;
    } else if (line.startsWith("|") && i + 1 < lines.length && SEP.test(lines[i + 1])) {
      out.push(table(run((l) => l.startsWith("|")), ctx));
    } else if (line.startsWith("- ")) {
      const items = run((l) => l.startsWith("- "));
      out.push("<ul>\n" + items.map((l) => `<li>${inline(l.slice(2), ctx)}</li>`).join("\n") + "\n</ul>");
    } else if (/^\d+\. /.test(line)) {
      const items = run((l) => /^\d+\. /.test(l));
      out.push("<ol>\n" + items.map((l) => `<li>${inline(l.replace(/^\d+\. /, ""), ctx)}</li>`).join("\n") + "\n</ol>");
    } else if (line.startsWith("> ")) {
      const quote = run((l) => l.startsWith("> "));
      out.push(`<blockquote><p>${inline(quote.map((l) => l.slice(2)).join("\n"), ctx)}</p></blockquote>`);
    } else if (!line.trim()) {
      i++;
    } else {
      const text = [lines[i++], ...run((l) => l.trim() && !BLOCK.test(l))];
      out.push(paragraph(text.join("\n"), ctx));
    }
  }
  return out.join("\n");
}

export function front(md) {
  const m = md.match(/^---\n([\s\S]*?)\n---\n?/);
  if (!m) return { data: {}, body: md };
  const data = {};
  for (const line of m[1].split("\n")) {
    const at = line.indexOf(":");
    if (at > 0) data[line.slice(0, at).trim()] = line.slice(at + 1).trim();
  }
  return { data, body: md.slice(m[0].length) };
}
