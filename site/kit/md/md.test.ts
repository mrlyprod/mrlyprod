import { describe, expect, test } from "bun:test";
import { figures, front, inline, render, slug, summary } from "./md.ts";

const math = (tex: string, display: boolean) => `<m${display ? " d" : ""}>${tex}</m>`;

const link = (url: string) => (/^(https?:|#|\/)/.test(url) ? url : `/L/${url}`);

describe("md", () => {
  test("headings carry the slug of their text and repeat with a counter", () => {
    const html = render("## Base $\\pi$ and *em*\n\n## Base $\\pi$ and *em*", { math });
    expect(html).toContain(`<h2 id="${slug("base π and em")}">`);
    expect(html).toContain(`<h2 id="${slug("base π and em")}-1">`);
  });

  test("a lone image naming a figure is a figure host with its caption", () => {
    expect(render("![the *walk*](walks-fig)", { link })).toBe('<figure data-figure="walks-fig"><canvas role="img" aria-label="the walk"></canvas><figcaption>the <em>walk</em></figcaption></figure>');
  });

  test("a lone image of a file is a figure around the resolved image, an inline image stays an image", () => {
    expect(render("![the *walk*](walk.png)", { link })).toBe('<figure><img src="/L/walk.png" alt="the walk"><figcaption>the <em>walk</em></figcaption></figure>');
    expect(render("see ![walk](walks-fig) here", { link })).toBe('<p>see <img src="/L/walks-fig" alt="walk"> here</p>');
  });

  test("the resolver is told which targets are images, a lone line, an inline one and a reference included", () => {
    const seen: string[] = [];
    const spy = (url: string, image?: boolean) => (seen.push(`${url}:${image ? "image" : "link"}`), url);
    render("![a](one.png)\n\nsee ![b](two) and [c](three) and ![d][four]\n\n[four]: four\n[five]: five", { link: spy });
    expect(seen.sort()).toEqual(["five:link", "four:image", "one.png:image", "three:link", "two:image"]);
  });

  test("every image after the first of a text loads lazily, figure or inline", () => {
    const html = render("![a](one.png)\n\nsee ![b](two) and ![c](three)\n\n![d](four.png)", { link });
    expect(html.match(/<img[^>]*>/g)).toEqual([
      '<img src="/L/one.png" alt="a">',
      '<img src="/L/two" alt="b" loading="lazy" decoding="async">',
      '<img src="/L/three" alt="c" loading="lazy" decoding="async">',
      '<img src="/L/four.png" alt="d" loading="lazy" decoding="async">',
    ]);
    expect(render("![a](one) ![b][two]\n\n[two]: two", { link })).toContain('<img src="/L/two" alt="b" loading="lazy" decoding="async">');
    expect(render("![a](one.png)", { link, lazy: true })).toContain('<img src="/L/one.png" alt="a" loading="lazy" decoding="async">');
  });

  test("figures lists the lone images that name a figure, not a file, an inline image or code", () => {
    expect(figures("![a](site-home)\n\nsee ![b](x) here\n\n![c](pic.png)\n\n```\n![d](y)\n```\n\n![e](blog-x)")).toEqual(["site-home", "blog-x"]);
  });

  test("math is inline at $, display at $$ on its own line or block, with escaped braces unescaped", () => {
    expect(render("$a<b$ and $$x^2$$", { math })).toBe("<p><m>a<b</m> and <m>x^2</m></p>");
    expect(render("$$x^2$$", { math })).toBe("<m d>x^2</m>");
    expect(render("$$\ny\n$$", { math })).toBe("<m d>y</m>");
    expect(render("$F = \\\\{0\\\\}$", { math })).toBe("<p><m>F = \\{0\\}</m></p>");
    expect(render("$5 and $10")).toBe("<p>$5 and $10</p>");
  });

  test("a table is wrapped, aligned by class and one row per line", () => {
    const html = render("| a | b | c |\n|:--|:-:|--:|\n| 1 | 2 | 3 |");
    expect(html).toBe('<div class="table"><table><thead><tr><th>a</th><th class="center">b</th><th class="right">c</th></tr></thead><tbody>\n<tr><td>1</td><td class="center">2</td><td class="right">3</td></tr>\n</tbody></table></div>');
  });

  test("an asterisk between two word characters is literal, around words it is emphasis, in code it is untouched", () => {
    expect(render("3*n*(n+1) and *\"2*20^n + 4*8^n\"* and `a*b*c`")).toBe('<p>3*n*(n+1) and <em>"2*20^n + 4*8^n"</em> and <code>a*b*c</code></p>');
  });

  test("every link and bare url goes through the resolver", () => {
    expect(render("[a](x.md) and https://oeis.org/A1", { link })).toBe('<p><a href="/L/x.md">a</a> and <a href="https://oeis.org/A1">https://oeis.org/A1</a></p>');
  });

  test("raw html is shown as text", () => {
    expect(render("n<m and <b>x</b>")).toBe("<p>n&lt;m and &lt;b>x&lt;/b></p>");
  });

  test("lists are tight, so a dated claim line starts its item", () => {
    expect(render("- 2026-09-19 [Proved] one\n- two\n\n1. a\n2. b")).toBe("<ul>\n<li>2026-09-19 [Proved] one</li>\n<li>two</li>\n</ul>\n<ol>\n<li>a</li>\n<li>b</li>\n</ol>");
  });

  test("inline drops the paragraph", () => {
    expect(inline("a **b** [c](d)", { link })).toBe('a <strong>b</strong> <a href="/L/d">c</a>');
  });

  test("a summary over the budget ends at its last sentence inside it, at a word only when no sentence ends there", () => {
    const one = "First one ends here. Second, e.g. this, runs on";
    expect(summary(one, 40)).toBe("First one ends here.");
    expect(summary("Words with no stop at all run past the budget", 20)).toBe("Words with no stop");
    expect(summary("Done. See Fig. 3 and Dr. Smith then", 31)).toBe("Done.");
    expect(summary('He said "stop." Then on and on', 20)).toBe('He said "stop."');
    expect(summary("Then (it ran.) And on and on", 18)).toBe("Then (it ran.)");
  });

  test("front matter is key colon value lines", () => {
    expect(front("---\ntitle: T\nlead: L\n---\nbody")).toEqual({ data: { title: "T", lead: "L" }, body: "body" });
    expect(front("body").data).toEqual({});
  });
});
