import { expect, mock, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import type { Row } from "../../scripts/build.ts";
import type { Own } from "./router.ts";

mock.module("site:apps", () => ({
  apps: { settings: async () => "settings" },
  own: new Set(["screensavers", "settings"]),
}));
mock.module("site:icons", () => ({ arts: {}, defs: "" }));

const { Gate, page } = await import("./app.tsx?gate");

const row = (id: string, more: Row["meta"] = {}): Row => ({
  route: `/${id}/`,
  kind: "app",
  title: id,
  source: null,
  date: null,
  lead: `${id} line`,
  figure: id,
  hidden: false,
  meta: { id, ...more },
});

test("on a phone a scene app, Kit and Screensavers draw the gate in a page window, other apps their own page", async () => {
  const kept = Object.getOwnPropertyDescriptor(globalThis, "document");
  Object.assign(globalThis, { document: { documentElement: { dataset: { form: "phone" } } } });
  const gate = page(row("life", { scene: true })) as Own;
  const same = [page(row("kit")), page(row("screensavers"))].every((one) => one === gate);
  const settings: unknown = await page(row("settings"));
  document.documentElement.dataset.form = "desk";
  const desk = (page(row("life", { scene: true })) as Own).layout?.window;
  if (kept) Object.defineProperty(globalThis, "document", kept);
  else Reflect.deleteProperty(globalThis, "document");
  expect([gate.layout?.window, same, settings, desk]).toEqual(["page", true, "settings", "app"]);
});

test("the gate shows the icon, title, line and still figure, the need and both buttons", () => {
  const html = renderToStaticMarkup(<Gate row={row("life", { scene: true })} />);
  expect(
    [
      'class="face"',
      "<h1>life</h1>",
      "life line",
      'data-figure="life"',
      "Needs a bigger screen",
      "Open on a computer",
      "Copy link",
    ].filter((one) => !html.includes(one)),
  ).toEqual([]);
});
