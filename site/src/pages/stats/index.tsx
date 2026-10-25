import { useEffect, useMemo, useRef, useState } from "react";
import { Btn, Icon } from "../../lib/app/controls.tsx";
import { Bar, usePanes } from "../../lib/app/frame.tsx";
import { Item, Library, Shelf } from "../../lib/app/library.tsx";
import { COPY, library, own, Reader, rule, useCopy } from "../../lib/app/reader.tsx";
import { paints } from "../../lib/color.ts";
import { fed, source } from "../../lib/glass/feed.ts";
import type { Data, Look, Model } from "./stats.ts";
import { ago, draw, errors, facts, PATH, poll, series, views } from "./stats.ts";

/* TYPES */

type ChartProps = { model: Model };

const LOADING: Model = { kind: "note", text: "Loading" };

/* LOOK */

function look(canvas: HTMLCanvasElement): Look {
  const css = getComputedStyle(canvas);
  const read = (name: string, fallback: string) => css.getPropertyValue(name).trim() || fallback;
  const { paper, accent, ink } = paints(canvas);
  return {
    paper,
    accent,
    ink,
    dim: read("--dim", ink),
    line: read("--line", ink),
    font: getComputedStyle(document.body).fontFamily,
  };
}

/* CHART */

function Chart({ model }: ChartProps) {
  const at = useRef<HTMLCanvasElement>(null);
  const latest = useRef(model);
  latest.current = model;
  const paint = useRef(() => {});
  useEffect(() => {
    const canvas = at.current!;
    const frame = canvas.closest(".frame")!;
    const drop = source(canvas, () => frame.getBoundingClientRect());
    const ctx = canvas.getContext("2d")!;
    let hover: number | null = null;
    const render = () => {
      const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
      const w = Math.max(1, Math.round(canvas.clientWidth * dpr));
      const h = Math.max(1, Math.round(canvas.clientHeight * dpr));
      if (canvas.width !== w) canvas.width = w;
      if (canvas.height !== h) canvas.height = h;
      draw(ctx, w, h, dpr, latest.current, look(canvas), hover);
      fed(canvas);
    };
    const move = (e: PointerEvent) => {
      const box = canvas.getBoundingClientRect();
      hover = (e.clientX - box.left) * (canvas.width / Math.max(1, box.width));
      render();
    };
    const leave = () => {
      hover = null;
      render();
    };
    paint.current = render;
    const tape = new ResizeObserver(render);
    const shade = matchMedia("(prefers-color-scheme: dark)");
    tape.observe(canvas);
    shade.addEventListener("change", render);
    addEventListener("theme", render);
    canvas.addEventListener("pointermove", move);
    canvas.addEventListener("pointerleave", leave);
    render();
    return () => {
      paint.current = () => {};
      tape.disconnect();
      shade.removeEventListener("change", render);
      removeEventListener("theme", render);
      canvas.removeEventListener("pointermove", move);
      canvas.removeEventListener("pointerleave", leave);
      drop();
    };
  }, []);
  useEffect(() => paint.current(), [model]);
  return <canvas ref={at} className="stats" role="img" aria-label={model.label ?? model.text} />;
}

/* WIDGET */

function Widget() {
  const panes = usePanes();
  const [data, setData] = useState<Data>(undefined);
  const [view, setView] = useState("cdn");
  const [copied, copy] = useCopy();
  useEffect(() => poll((next) => setData((was) => next ?? was ?? null)), []);
  const list = views(data);
  const found = errors(data);
  const model = useMemo(() => (data === undefined ? LOADING : series(data, view)), [data, view]);
  const raw = () => open(PATH, "_blank", "noopener");
  const menus = [
    { name: "File", items: [COPY, { label: "Open Raw", act: "raw" }] },
    {
      name: "View",
      items: rule(
        list.map((one) => ({ label: one.label, act: `view:${one.id}`, radio: true, on: one.id === view })),
        [library(panes)],
      ),
    },
  ];
  const run = (act: string) => {
    if (act === "copy") copy();
    if (act === "raw") raw();
    if (act.startsWith("view:")) setView(act.slice(5));
  };
  return (
    <Reader title="Stats" menus={menus} run={run} panes={panes}>
      <Chart model={model} />
      <Library name="Stats">
        <Shelf name="Views">
          {list.map((one) => (
            <Item key={one.id} label={one.label} sub={one.head} on={one.id === view} onPick={() => setView(one.id)} />
          ))}
        </Shelf>
        {found.length > 0 && (
          <Shelf name="Errors">
            {found.map((one, n) => (
              <Item
                key={n}
                label={one.message}
                sub={ago(one.at)}
                title={`${one.name} ${one.level} ${one.at}`}
                onPick={() => setView("errors")}
              />
            ))}
          </Shelf>
        )}
      </Library>
      <Bar side="actions">
        <Btn className="share" onClick={copy}>
          <Icon name="ios_share" />
          Share
        </Btn>
        <Btn onClick={raw}>Raw</Btn>
      </Bar>
      <Bar side="status">
        {data ? (
          <>
            <span>updated {ago(data.at)}</span>
            <span>last {data.hours} h</span>
            {facts(data, view).map((one) => (
              <span key={one} className="out">
                {one}
              </span>
            ))}
          </>
        ) : (
          <span>{data === null ? "No data yet." : "Loading"}</span>
        )}
        {copied && <span>link copied</span>}
      </Bar>
    </Reader>
  );
}

export const { layout, render, mount, unmount } = own(() => <Widget />);
