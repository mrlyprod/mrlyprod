import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { renderToStaticMarkup } from "react-dom/server";
import { sheets } from "../../net/hosts/web/scripts/design.ts";
import { design } from "../../net/shell/active.ts";
import { Frame } from "../../net/shell/frame.tsx";
import { modern } from "./index.ts";

test("the shell sheet wears every modern sheet in its old place among the shell's own", () => {
  const page = readFileSync(join(import.meta.dir, "..", "..", "net", "shell", "page.css"), "utf8");
  const virtual = sheets("modern", modern.css);
  const order = [...page.matchAll(/@import "([^"]+)"/g)].flatMap(([, one]) =>
    one!.startsWith("site:")
      ? [...virtual[one!]!.matchAll(/designs\/modern\/([^"]+)"/g)].map(([, file]) => `modern/${file}`)
      : [one!.split("/").slice(-2).join("/")],
  );
  expect(order).toEqual([
    "shared/palette.gen.css",
    "modern/tokens.css",
    "shared/base.css",
    "shared/appicons.css",
    "./gate.css",
    "modern/chrome.css",
    "modern/desk.css",
    "modern/appicons.css",
    "fonts/fonts.css",
    "shared/pages.css",
    "shared/settings.css",
    "modern/saver.css",
    "shared/screensavers.css",
  ]);
});

test.skipIf(design.id !== "modern")("the bar wears the cart only for a site that asks for one", () => {
  const html = renderToStaticMarkup(<Frame title="Any" mark="/mark.svg" word="CARLO MITCHENER" />);
  expect([html.includes("data-cart"), html.includes('class="logo"')]).toEqual([false, true]);
});
