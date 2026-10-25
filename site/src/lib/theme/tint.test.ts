import { expect, test } from "bun:test";
import { HUES } from "./hues.ts";
import { tintCss } from "./tint.ts";

test("the tint style takes the named hue by default, an unknown one not at all, and offers every hue on demand", () => {
  const css = tintCss("indigo");
  expect(css).toContain(":root:not([data-tint]) { --accent: var(--indigo); --link: var(--indigo-link); }");
  expect(tintCss("nothing")).not.toContain(":not([data-tint])");
  expect(
    HUES.filter(
      (hue) => !css.includes(`:root[data-tint="${hue}"] { --accent: var(--${hue}); --link: var(--${hue}-link); }`),
    ),
  ).toEqual([]);
});
