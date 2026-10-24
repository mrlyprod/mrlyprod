import { afterAll, expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { Gallery, Widget } from "./widget.jsx";

const rows = [{ route: "/research/", kind: "app", title: "Research", lead: "The notes.", meta: {} }];

const saved = Object.getOwnPropertyDescriptor(globalThis, "location");

const at = (pathname) => {
  globalThis.location = { pathname };
  return renderToStaticMarkup(<Widget rows={rows} site={{}} draw={() => Promise.reject()} />);
};

afterAll(() => {
  if (saved) Object.defineProperty(globalThis, "location", saved);
  else delete globalThis.location;
});

test("before the topics load the app window holds a scrolling page with the title and lead and no dialog", () => {
  const html = at("/research/");
  expect(html).toContain('<section class="frame" aria-label="Research"');
  expect(html).toContain('<div class="page"><div class="lede"><h1 id="research">Research</h1>');
  expect(html).toContain("The notes.");
  expect(html).not.toContain("<dialog");
});

test("a topic path waits for its note with no lede over it", () => {
  expect(at("/research/farey/")).toContain('<div class="page"></div>');
});

test("the gallery draws every figure of a topic under its name and nothing for a topic without figures", () => {
  const html = renderToStaticMarkup(<Gallery names={["research-farey", "paper-farey-cover"]} />);
  expect(html).toContain('<section class="gallery grid" aria-label="Figures">');
  expect(html).toContain('<figure data-figure="research-farey">');
  expect(html).toContain("<figcaption>paper-farey-cover</figcaption>");
  expect(html.match(/<figure /g)).toHaveLength(2);
  expect(renderToStaticMarkup(<Gallery />)).toBe("");
});
