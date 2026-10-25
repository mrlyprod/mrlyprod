import { expect, test } from "bun:test";
import { useEffect, useRef } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { classed, find, mount, text } from "../../../test/dom.ts";
import { source } from "../glass/feed.ts";
import { Bar, calm, Frame, fit, floats, pushed, slot } from "./frame.tsx";

function Fed() {
  const at = useRef<HTMLCanvasElement>(null);
  useEffect(() => source(at.current!), []);
  return <canvas ref={at} />;
}

test("on the server a frame is its stage alone: a filled Bar claims its side only after layout", () => {
  expect(
    renderToStaticMarkup(
      <Frame label="Sky">
        <canvas />
        <Bar side="right">
          <input />
        </Bar>
        <Bar side="status">seed 7</Bar>
      </Frame>,
    ),
  ).toBe(
    '<section class="frame" aria-label="Sky" style="--left:280px;--right:280px"><div class="canvas"><canvas></canvas></div></section>',
  );
});

test("outside a frame the right bar lands in the shell's #right .controls and no other side lands anywhere", () => {
  const controls = { id: "controls" } as Element;
  const doc = { querySelector: (selector: string) => (selector === "#right .controls" ? controls : null) };
  expect([slot(null, "right", doc), slot(null, "status", doc), slot(null, "left", doc)]).toEqual([
    controls,
    null,
    null,
  ]);
});

test("the right bar lets go of a control a pointer changed and keeps one a key stepped", () => {
  const on: Record<string, EventListener> = {};
  const box = Object.assign({} as EventTarget, {
    addEventListener: (name: string, fn: EventListener) => (on[name] = fn),
    removeEventListener: (name: string) => delete on[name],
  });
  const blurred: string[] = [];
  const change = (by: string) => on.change(Object.assign({} as Event, { target: { blur: () => blurred.push(by) } }));
  const quit = calm(box);
  on.keydown(new Event("keydown"));
  change("key");
  on.pointerdown(new Event("pointerdown"));
  change("pointer");
  on.keydown(new Event("keydown"));
  change("key again");
  quit();
  expect([blurred, Object.keys(on)]).toEqual([["pointer"], []]);
});

test("open panes push the canvas by the wider one on both sides, and the panes float once the canvas would be too narrow", () => {
  const widths = { left: 240, right: 300 };
  expect([
    pushed({ left: true, right: false }, widths, 10),
    pushed({ left: true, right: true }, widths, 10),
    pushed({}, widths, 10),
  ]).toEqual([260, 320, 0]);
  expect([floats(1000, 320), floats(1000, 330)]).toEqual([false, true]);
});

test("the status pill drops whole facts from the end, and lets its hints go from the end only when its first fact would not fit", () => {
  const rights = [40, 90, 150, 200];
  expect([
    fit(rights, 200),
    fit(rights, 160),
    fit(rights, 100),
    fit(rights, 10, 1),
    fit(rights, 60, 2, [500, 560, 620], 680),
    fit(rights, 20, 2, [500, 560, 620], 680),
    fit([], 10),
  ]).toEqual([
    [4, 0],
    [3, 0],
    [2, 0],
    [1, 0],
    [2, 2],
    [2, 1],
    [0, 0],
  ]);
});

test("the tail bar lands after every right bar, whatever mounts first, and opens the right pane on its own", async () => {
  const pane = await mount(async (draw, host) => {
    const at = () => find(host, classed("right"))!;
    await draw(
      <Frame label="Sky">
        <Bar side="tail">seed</Bar>
      </Frame>,
    );
    const alone = text(at());
    await draw(
      <Frame label="Sky">
        <Bar side="tail">seed</Bar>
        <Bar side="right">look</Bar>
      </Frame>,
    );
    return [alone, text(at())];
  });
  expect(pane).toEqual(["seed", "lookseed"]);
});

test("a frame with no canvas fed to the glass frosts its panes and pills, and a fed one keeps them lenses", async () => {
  const looks = await mount(async (draw, host) => {
    const look = () =>
      ["right", "actions", "status"].map((name) => {
        const at = find(host, classed(name))!;
        return [at.attributes.class, at.attributes["data-lens"] ?? null];
      });
    const bars = [
      <Bar key="right" side="right">
        look
      </Bar>,
      <Bar key="actions" side="actions">
        go
      </Bar>,
      <Bar key="status" side="status">
        seed
      </Bar>,
    ];
    await draw(<Frame label="Read">{bars}</Frame>);
    const read = look();
    await draw(
      <Frame label="Sky">
        <Fed />
        {bars}
      </Frame>,
    );
    return [read, look()];
  });
  expect(looks).toEqual([
    [
      ["pane right frost", null],
      ["actions frost", null],
      ["status frost", null],
    ],
    [
      ["pane right", "panel"],
      ["actions", "pill"],
      ["status", "pill"],
    ],
  ]);
});
