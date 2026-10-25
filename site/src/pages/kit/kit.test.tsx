import { expect, mock, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { defaults, tidy } from "../../lib/app/spec.ts";
import { HUES } from "../../lib/color.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Name } from "../../lib/theme/theme.ts";
import type { Fill } from "./look.tsx";

const ROW = { route: "/kit/", kind: "app", title: "Kit", lead: "Every control.", meta: { id: "kit" } };

mock.module("site:routes", () => ({ rows: [ROW], site: { launchpad: { sections: [], dock: [] } } }));
mock.module("site:icons", () => ({ arts: {}, defs: "" }));

const { Kit } = await import("./kit.tsx");
const { bands } = await import("./look.tsx");
const { SPEC } = await import("./parts.tsx");

const html = renderToStaticMarkup(<Kit row={ROW} />);

test("the page names its sections in order and links each from the top", () => {
  const heads = [...html.matchAll(/<h2 id="([a-z]+)">([^<]+)<\/h2>/g)].map(([, id, text]) => `${id}:${text}`);
  expect(heads).toEqual([
    "theme:Theme and tint",
    "glass:Glass",
    "icons:Icon styles",
    "controls:Controls",
    "library:Library",
    "designs:Design picker",
    "type:Type and shape",
  ]);
  expect([...html.matchAll(/<a class="chip" href="#([a-z]+)"/g)].map(([, id]) => id)).toEqual(
    heads.map((one) => one.split(":")[0]),
  );
});

test("the glass section draws a pill, a panel, a menu and a frost box over one picture", () => {
  const glass = html.slice(html.indexOf('<div class="backdrop">'), html.indexOf('<h2 id="icons"'));
  expect([...glass.matchAll(/data-lens="([a-z]+)"/g)].map(([, kind]) => kind)).toEqual(["pill", "panel", "menu"]);
  expect(glass.match(/<canvas/g)).toHaveLength(1);
  expect(glass).toContain('class="frost"');
});

test("the icon section shows the six looks, each under its own data-icons", () => {
  expect([...html.matchAll(/<figure data-icons="([a-z-]*)"/g)].map(([, look]) => look)).toEqual([
    "",
    "dark",
    "clear-light",
    "clear-dark",
    "tinted-light",
    "tinted-dark",
  ]);
});

test("the knobs spec holds one row of every kind, each default a valid value", () => {
  expect(SPEC.map((row) => row.kind).sort()).toEqual(["list", "number", "pick", "segment", "slider", "text", "toggle"]);
  expect(tidy(SPEC, defaults(SPEC))).toEqual(defaults(SPEC));
});

test("the bands lay the paper, one column of every hue, then the stripes", () => {
  const fills: [Fill["fillStyle"], number, number, number, number][] = [];
  const ctx: Fill = { fillStyle: "", fillRect: (...box) => fills.push([ctx.fillStyle, ...box]) };
  bands(ctx, 120, 60, { paper: "#fff", ink: "#000" });
  expect(fills.slice(0, 1 + HUES.length).map(([color]) => color)).toEqual([
    "#fff",
    ...HUES.map((hue) => palette[hue as Name]),
  ]);
  expect(fills.slice(1, 1 + HUES.length).reduce((width, [, , , w]) => width + w, 0)).toBe(120);
  expect(fills.slice(1 + HUES.length).map(([color]) => color)).toEqual(["#fff", "#000", "#fff"]);
});
