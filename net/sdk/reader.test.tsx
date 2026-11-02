import { expect, test } from "bun:test";
import { COPY, layout, library, own, rule } from "./reader.tsx";

test("a reader page owns the app window and draws a noscript line before it mounts", () => {
  const page = own(() => null);
  const host = Object.assign({} as HTMLElement, { innerHTML: "" });
  page.render({ title: "Stats" }, {}, host);
  expect([page.layout, host.innerHTML]).toEqual([
    layout,
    "<noscript><p>Stats draws in the browser and needs JavaScript.</p></noscript>",
  ]);
  expect(layout).toEqual({ wide: true, bare: true, sheet: "mrly", window: "app" });
});

test("rule joins menu parts with one separator and drops empty parts; the library item mirrors the pane", () => {
  expect(rule([COPY], [], [{ label: "Raw", act: "raw" }])).toEqual([COPY, null, { label: "Raw", act: "raw" }]);
  expect(library({ open: { left: false } })).toEqual({
    label: "Show Library",
    act: "pane:left",
    keys: ["mod", "["],
    on: false,
  });
});
