import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { doc, find, mount, text } from "../../../test/dom.ts";
import { Btn, Export, Fold, Icon, Menu, Pick, Row, Segment, Toggle } from "./controls.tsx";
import type { Scalar } from "./spec.ts";

type Loose = { detail: number; currentTarget: { blur: () => void } };

type Props = { onChange: (e: { target: { value: string } }) => void };

test("a Toggle writes 1 when switched on and 0 when switched off", () => {
  const wrote: number[] = [];
  const input = (value: Scalar) => Toggle({ label: "wrap", value, onChange: (v) => wrote.push(v) }).props.children[1];
  input(0).props.onChange({ target: { checked: true } });
  input(1).props.onChange({ target: { checked: false } });
  expect(wrote).toEqual([1, 0]);
});

test("a Btn or a fold header a pointer clicks lets go of focus before its action, one a key presses keeps it", () => {
  const seen: string[] = [];
  const click = Btn({ onClick: () => seen.push("act"), children: "Reroll" }).props.onClick;
  const head = Fold({ name: "Jump", open: false }).props.children[0].props.onClick;
  const at = (fn: (e: Loose) => void, detail: number) =>
    fn({ detail, currentTarget: { blur: () => seen.push(`blur ${detail}`) } });
  at(click, 1);
  at(click, 0);
  at(head, 1);
  at(head, 0);
  expect(seen).toEqual(["blur 1", "act", "act", "blur 1"]);
});

test("a fold summary ends on the expand_more chevron, after its live values", () => {
  const fold = renderToStaticMarkup(
    <Fold name="Jump" open={false} live="20">
      x
    </Fold>,
  );
  expect(fold).toContain(`<span class="live">20</span>${renderToStaticMarkup(<Icon name="expand_more" />)}</summary>`);
});

test("a pick draws the expand_more chevron in a span after its select", () => {
  const pick = renderToStaticMarkup(<Pick label="Shape" value="a" options={["a", "b"]} onChange={() => {}} />);
  expect(pick).toContain(
    `</select><span class="drop">${renderToStaticMarkup(<Icon name="expand_more" />)}</span></label>`,
  );
});

test("Export is a dialog: PNG and WebP, the sizes 1x, 2x and 4x only for a scene that can fix its frame, a Save button, then the rows of more in order", () => {
  const html = (fix?: () => void) =>
    renderToStaticMarkup(
      <Export
        canvas={null}
        name="x"
        fix={fix}
        more={[
          ["Video 9:16", () => {}],
          ["Video 16:9", () => {}],
        ]}
      />,
    );
  const radios = (text: string) =>
    [...text.matchAll(/type="radio" name="[^"]*"[^>]*? value="([^"]+)"/g)].map((found) => found[1]);
  expect(html(() => {})).toContain('<dialog class="modal export" aria-label="Export">');
  expect([radios(html(() => {})), radios(html())]).toEqual([
    ["png", "webp", "1", "2", "4"],
    ["png", "webp"],
  ]);
  expect(
    [...html().matchAll(/<button type="button"( class="primary")?>([^<]+)<\/button>/g)].map((found) => found[2]),
  ).toEqual(["Save PNG", "Video 9:16", "Video 16:9"]);
});

test("a Menu is a button and a popover listing its rows in order, a row with an icon drawing it before its text", () => {
  const html = renderToStaticMarkup(
    <Menu
      className="icon more"
      aria-label="More"
      rows={[
        ["Copy link", () => {}, "link"],
        ["Zen", () => {}],
      ]}
    >
      x
    </Menu>,
  );
  expect(html).toMatch(
    /<button type="button" class="icon more" popoverTarget="(menu[^"]+)"[^>]*aria-label="More">x<\/button><div id="\1" popover="auto" class="pop"/,
  );
  expect(
    [...html.matchAll(/<button type="button">(<svg[\s\S]*?<\/svg>)?([^<]+)<\/button>/g)].map((found) => [
      found[2],
      Boolean(found[1]),
    ]),
  ).toEqual([
    ["Copy link", true],
    ["Zen", false],
  ]);
});

test("a Segment wider than its row is a select with the same label and change path until the row grows to fit it, swapping back in the same frame", async () => {
  const make = doc.createElement;
  const wrote: Scalar[] = [];
  let room = 200;
  let grow: (() => void) | undefined;
  doc.createElement = (tag: string) => {
    const el = make(tag);
    return Object.assign(el, tag === "select" ? { options: [] } : {}, {
      classList: { contains: (one: string) => el.getAttribute("class") === one },
      getBoundingClientRect: () => ({ width: el.style.width === "max-content" ? 300 : room }),
    });
  };
  Object.assign(globalThis, {
    ResizeObserver: class {
      constructor(fn: () => void) {
        grow = fn;
      }
      observe() {}
      disconnect() {}
    },
  });
  try {
    await mount(async (draw, host) => {
      await draw(
        <Row>
          <Segment
            label="View"
            value={2}
            options={[
              [1, "Flat"],
              [2, "Solid"],
              [3, "Both"],
            ]}
            onChange={(v) => wrote.push(v)}
          />
        </Row>,
      );
      const select = find(host, (at) => at.tagName === "SELECT")!;
      (Object.entries<unknown>(select).find(([key]) => key.startsWith("__reactProps"))![1] as Props).onChange({
        target: { value: "3" },
      });
      const label = text(select.parentNode!.childNodes[0]);
      room = 300;
      Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: false });
      grow!();
      Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: true });
      expect([label, wrote, Boolean(find(host, (at) => at.getAttribute?.("role") === "radiogroup"))]).toEqual([
        "View",
        [3],
        true,
      ]);
    });
  } finally {
    doc.createElement = make;
    Reflect.deleteProperty(globalThis, "ResizeObserver");
  }
});
