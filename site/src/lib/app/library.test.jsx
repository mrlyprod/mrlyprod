import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { doc, find, mount } from "../../../test/dom.js";
import { Chips, Item, roam, Shelf } from "./library.jsx";

test("arrows walk a list by one and a grid by its row, clamped at both ends, Home and End jump", () => {
  const items = [0, 1, 2, 3, 4, 5, 6];
  expect([
    roam(items, 2, "ArrowDown"),
    roam(items, 2, "ArrowDown", 3),
    roam(items, 5, "ArrowDown", 3),
    roam(items, 1, "ArrowUp", 3),
    roam(items, 2, "ArrowRight", 3),
    roam(items, 2, "ArrowRight"),
    roam(items, 4, "Home"),
    roam(items, 1, "End"),
    roam(items, 1, "x"),
    roam([], 0, "End"),
  ]).toEqual([3, 5, 6, 0, 3, null, 0, 6, null, null]);
});

test("a shelf is a named list or grid, and its picked item is the current one", () => {
  const html = renderToStaticMarkup(
    <Shelf name="Places" grid>
      <Item label="Seahorse Valley" sub="-0.75, 0.1" on />
      <Item label="Home" href="/fractals/" />
    </Shelf>,
  );
  expect(html).toBe(
    '<section class="shelf" aria-label="Places"><h3>Places</h3><div class="grid" role="list"><button type="button" data-item="" aria-current="true" class="on" role="listitem"><span class="label">Seahorse Valley</span><span class="sub">-0.75, 0.1</span></button><a href="/fractals/" data-item="" role="listitem"><span class="label">Home</span></a></div></section>',
  );
});

test("chips wider than the library are a select with the same change path until the library grows to fit them", async () => {
  const make = doc.createElement;
  const wrote = [];
  let room = 200;
  let grow;
  doc.createElement = (tag) => {
    const el = make(tag);
    return Object.assign(el, tag === "select" ? { options: [] } : {}, {
      getBoundingClientRect: () => ({ width: el.style.width === "max-content" ? 300 : room }),
    });
  };
  globalThis.ResizeObserver = class {
    constructor(fn) {
      grow = fn;
    }
    observe() {}
    disconnect() {}
  };
  try {
    await mount(async (draw, host) => {
      await draw(
        <Chips
          label="Shapes filter"
          chips={[
            ["all", "All"],
            ["named", "Named"],
            ["family", "Family"],
          ]}
          chip="all"
          onChip={(v) => wrote.push(v)}
        />,
      );
      const select = find(host, (at) => at.tagName === "SELECT");
      Object.entries(select)
        .find(([key]) => key.startsWith("__reactProps"))[1]
        .onChange({ target: { value: "family" } });
      room = 300;
      globalThis.IS_REACT_ACT_ENVIRONMENT = false;
      grow();
      globalThis.IS_REACT_ACT_ENVIRONMENT = true;
      expect([wrote, Boolean(find(host, (at) => at.getAttribute?.("role") === "radiogroup"))]).toEqual([
        ["family"],
        true,
      ]);
    });
  } finally {
    doc.createElement = make;
    delete globalThis.ResizeObserver;
  }
});
