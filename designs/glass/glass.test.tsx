import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { renderToStaticMarkup } from "react-dom/server";
import { sheets } from "../../net/hosts/web/scripts/design.ts";
import { design } from "../../net/shell/active.ts";
import { lacks } from "../../net/shell/design.ts";
import { Frame } from "../../net/shell/frame.tsx";
import { glass } from "./index.ts";

test("glass draws every required part", () => {
  expect(lacks(glass)).toEqual([]);
});

test("the shell sheet wears every glass sheet in its old place among the shell's own", () => {
  const page = readFileSync(join(import.meta.dir, "..", "..", "net", "shell", "page.css"), "utf8");
  const virtual = sheets("glass", glass.css);
  const order = [...page.matchAll(/@import "([^"]+)"/g)].flatMap(([, one]) =>
    one!.startsWith("site:")
      ? [...virtual[one!]!.matchAll(/designs\/glass\/([^"]+)"/g)].map(([, file]) => `glass/${file}`)
      : [one!.split("/").slice(-2).join("/")],
  );
  expect(order).toEqual([
    "shared/palette.gen.css",
    "glass/tokens.css",
    "shared/base.css",
    "./gate.css",
    "glass/chrome.css",
    "glass/desk.css",
    "glass/appicons.css",
    "fonts/fonts.css",
    "shared/pages.css",
    "shared/settings.css",
    "glass/saver.css",
    "shared/screensavers.css",
  ]);
});

test.skipIf(design.id !== "glass")(
  "the frame glass draws holds every hook the shell finds by id, class or data",
  () => {
    const html = renderToStaticMarkup(<Frame title="MrlyProd" mark="/favicon.svg" word="MRLYPROD" cart />);
    const hooks = [
      'id="main"',
      'id="right"',
      'id="scroll"',
      'data-pane="right"',
      'class="date"',
      'class="clock"',
      'class="glyph find"',
      'class="close"',
      'class="menus"',
      'data-cart="true"',
    ];
    expect(hooks.filter((one) => !html.includes(one))).toEqual([]);
  },
);

test.skipIf(design.id !== "glass")("the bar wears the cart only for a site that asks for one", () => {
  const html = renderToStaticMarkup(<Frame title="Any" mark="/mark.svg" word="CARLO MITCHENER" />);
  expect([html.includes("data-cart"), html.includes('class="glyphs fold"')]).toEqual([false, true]);
});
