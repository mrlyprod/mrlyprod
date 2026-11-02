import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { renderToStaticMarkup } from "react-dom/server";
import { sheets } from "../../../scripts/design.ts";
import { design } from "../../shell/active.ts";
import { lacks } from "../../shell/design.ts";
import { Frame } from "../../shell/frame.tsx";
import { glass } from "./index.ts";

test("glass draws every required part", () => {
  expect(lacks(glass)).toEqual([]);
});

test("the shell sheet wears every glass sheet in its old place among the shell's own", () => {
  const page = readFileSync(join(import.meta.dir, "..", "..", "styles", "page.css"), "utf8");
  const virtual = sheets("glass", glass.css);
  const order = [...page.matchAll(/@import "([^"]+)"/g)].flatMap(([, one]) =>
    one!.startsWith("site:")
      ? [...virtual[one!]!.matchAll(/designs\/glass\/([^"]+)"/g)].map(([, file]) => `glass/${file}`)
      : [one!],
  );
  expect(order).toEqual([
    "./palette.gen.css",
    "glass/tokens.css",
    "./base.css",
    "./gate.css",
    "glass/chrome.css",
    "glass/desk.css",
    "glass/appicons.css",
    "./fonts/fonts.css",
    "./pages.css",
    "./settings.css",
    "glass/saver.css",
    "./screensavers.css",
  ]);
});

test.skipIf(design.id !== "glass")(
  "the frame glass draws holds every hook the shell finds by id, class or data",
  () => {
    const html = renderToStaticMarkup(<Frame title="MrlyProd" />);
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
