import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef } from "react";
import { text } from "../../lib/app/export.ts";
import type { Had, Readout } from "../../lib/app/hooks.ts";
import { useReroll } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import type { Stack } from "./engine.ts";
import { quadruple, ROOTS, roll, said, step } from "./engine.ts";
import type { Apollonian, Handled, Knobs, Told } from "./scene.ts";
import { file, make, portrait } from "./scene.ts";

/* TYPES */

type Props = WidgetProps<Had<"num">, Handled> & { value: Knobs & { seed: number } };

type Hooks = Partial<Pick<Props, "onChange" | "onStatus">>;

const ART = 112;

const at = (e: PointerEvent<HTMLCanvasElement>): [number, number] => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

function stack(s: Stack | null): Readout[] {
  if (!s) return [];
  return [
    ["stack", `Q = ${s.order}`],
    ["brightness", `${s.bright} of Q(Q + 1)/2 = ${s.want}`],
    ["nodes lit", String(s.nodes)],
    ["touched", String(s.touched)],
    ["missed", String(s.missed)],
    [`off-Ford below ${s.reach}`, String(s.offford)],
  ];
}

export function rows(f: Told | null): Readout[] {
  if (!f) return [];
  const line: Readout[] =
    f.line === null
      ? [["line", "none in this root, so no Ford circles and no stack"]]
      : [["Ford", `${f.ford} of ${f.line} on the line`]];
  const census: Readout[] = [
    ["drawn", String(f.drawn)],
    ["quadruples", String(f.quads)],
    ["broken", String(f.broken)],
    ["strayed", String(f.strayed)],
  ];
  const off: Readout[] = f.line === null ? [] : [["off-Ford", String(f.line - f.ford!)]];
  return [
    ...(f.circle ? [["circle", said(f.circle)] satisfies Readout] : []),
    ["root", f.title],
    ["N(T)", `${f.circles} to T = ${f.cap}`],
    ...line,
    ...stack(f.shadow),
    ...census,
    ...off,
  ];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }: Props) {
  const unit = units?.num;
  const live = useRef<Handle<Apollonian> | null>(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef<Hooks>({});
  hooks.current = { onChange, onStatus };
  const tick = useTheme();
  useReroll(value.seed, (next) => hooks.current.onChange?.(roll(rng(next))));
  const tell = useCallback((facts: Told) => hooks.current.onStatus?.(rows(facts)), []);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, num: unit, onTell: tell }),
    [unit, tell],
  );
  const deeper = (by: number) => {
    const cap = step(now.current.cap, by);
    if (cap !== now.current.cap) hooks.current.onChange?.({ cap });
  };
  const ready = (handle: Handle<Apollonian> | null) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, deeper });
  };
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    if (e.button) return;
    live.current?.tap(...at(e));
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current), "svg");
  };
  const dump = () => {
    const list = live.current?.json?.();
    if (list) text(list, file(now.current), "json");
  };
  const have = Boolean(unit);
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["SVG", keep],
              ["JSON", dump],
            ]
          : [],
      ),
    [have],
  );
  return (
    <>
      {unit && (
        <Library>
          <Shelf name="Packings" grid>
            {ROOTS.map((root) => (
              <Item
                key={root}
                label={quadruple(root)}
                sub={root === "strip" ? "strip" : "bounded"}
                on={value.root === root}
                onPick={() => onChange?.({ root })}
                art={
                  <Art
                    px={ART}
                    paint={(ctx, w, h, dpr) => portrait(unit, root, ctx, w, h, dpr, paints(ctx.canvas, value.tint))}
                    deps={[unit, value.tint, tick]}
                  />
                }
              />
            ))}
          </Shelf>
        </Library>
      )}
      {unit && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} />}
    </>
  );
}
