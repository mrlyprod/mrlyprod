import { describe, expect, test } from "bun:test";
import { figures, front, plain, slug, summary } from "./md.ts";

describe("md", () => {
  test("plain strips the marks and spells the TeX a lead or a heading carries", () => {
    expect(plain("the **bold** `code` [link](/x/) and $\\pi$")).toBe("the bold code link and π");
    expect(slug(plain("Base $\\pi$ and *em*"))).toBe("base-π-and-em");
  });

  test("figures lists the lone images that name a figure, not a file, an inline image, a fenced one or a heading's", () => {
    expect(
      figures(
        "![a](home)\n\nsee ![b](x) here\n\n![c](pic.png)\n\n```\n![d](y)\n```\n\n## T\n![e](blog-x)\n\n- ![f](listed)",
      ),
    ).toEqual(["home", "blog-x", "listed"]);
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
