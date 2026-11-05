import type { Action } from "@mrlyprod/ui/kit/controls.tsx";
import { Group } from "@mrlyprod/ui/kit/controls.tsx";
import { Browser } from "@mrlyprod/ui/kit/designs/browse.tsx";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Design } from "@mrlyprod/ui/kit/designs/store.ts";
import { choose, place, read } from "@mrlyprod/ui/kit/designs/store.ts";
import { text } from "@mrlyprod/ui/kit/export.ts";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useUnit } from "@mrlyprod/ui/kit/hooks.ts";
import { Library } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import { share } from "@mrlyprod/ui/kit/query.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Seeded, Units } from "@mrlyprod/ui/kit/spec.ts";
import type { PointerEvent } from "react";
import { useCallback, useEffect, useRef, useState } from "react";
import type { Three } from "../shared/three/three.ts";
import { useThree } from "../shared/three/three.ts";
import type { Designs, Info } from "./scene.ts";
import { make, named, PAGE, step } from "./scene.ts";

type Props = Partial<WidgetProps<Units, Designs>> &
  Pick<WidgetProps<Units, Designs>, "onChange"> & { value: Seeded; unit?: Mrly | null; three?: Three | null };

type Press = { x: number; y: number; far: number };

const TAP = 6;

const SLOT = PAGE.design;

export function rows(info: Info) {
  const out: Readout[] = [["Design", info.name ? `${info.name} ${info.code}` : info.code]];
  if (info.rep !== info.code) out.push(["Class", info.rep]);
  out.push(["Orbit", String(info.orbit)]);
  if (info.degree >= 0) out.push(["Degree", String(info.degree)]);
  if (info.anf) out.push(["Normal form", info.anf]);
  out.push(
    ["Filled", `${info.fills} of ${info.grid}`],
    ["Ratio", info.ratio.toFixed(4)],
    ["Dimension", info.dimension.toFixed(4)],
  );
  out.push(info.dim === 2 ? ["Perimeter", String(info.perimeter)] : ["Surface", String(info.surface)]);
  out.push(
    ["Euler", String(info.euler)],
    ["Codes", info.codes.toLocaleString()],
    ["Classes", info.classes.toLocaleString()],
    [`Distinct, dim 1 to ${info.upto}`, info.terms.join(", ")],
  );
  return out;
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit, three: given }: Props) {
  const math = useUnit("math", unit);
  const { dim, base, code } = read(SLOT, value);
  const three = useThree(dim === 3, given);
  const live = useRef<Handle<Designs> | null>(null);
  const press = useRef<Press | null>(null);
  const change = useRef(onChange);
  change.current = onChange;
  const [info, setInfo] = useState<Info | null>(null);
  const tell = useCallback((next: Info) => setInfo(next), []);
  const here = info?.dim === dim && info?.base === base ? info : null;
  const now = useRef(value);
  now.current = { ...value, code: here?.code ?? code };
  useEffect(() => onStatus?.(here ? rows(here) : []), [here]);
  const staged = Boolean(math && (dim !== 3 || three));
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, math: math!, three, tell }),
    [math, three, tell],
  );
  const ready = (handle: Handle<Designs> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const down = (e: PointerEvent<HTMLCanvasElement>) => {
    if (e.button) return;
    press.current = { x: e.clientX, y: e.clientY, far: 0 };
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e: PointerEvent<HTMLCanvasElement>) => {
    const from = press.current;
    if (!from) return;
    const box = e.currentTarget.getBoundingClientRect();
    const dx = e.clientX - from.x;
    const dy = e.clientY - from.y;
    press.current = { x: e.clientX, y: e.clientY, far: from.far + Math.hypot(dx, dy) };
    live.current?.turn?.(dx / box.width, dy / box.height);
  };
  const up = (e: PointerEvent<HTMLCanvasElement>) => {
    const from = press.current;
    press.current = null;
    if (math && from && e.type === "pointerup" && from.far < TAP) change.current(step(math, now.current, 1));
  };
  const keep = (body: string | undefined, kind: string) => body && text(body, named(now.current), kind);
  useEffect(
    () =>
      onExport?.([
        ["SVG", () => keep(live.current?.svg?.(), "svg")],
        ...(dim === 3 ? [["OBJ", () => keep(live.current?.obj?.(), "obj")] satisfies Action] : []),
      ]),
    [dim],
  );
  const pick = (one: Design) => {
    onChange(place(SLOT, one));
    choose(one);
  };
  return (
    <>
      {staged && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onPointerDown={down}
          onPointerMove={move}
          onPointerUp={up}
          onPointerCancel={up}
        />
      )}
      <Library name="Gallery">
        <Browser compact slot={SLOT} value={value} math={math} onPick={pick} />
      </Library>
      {dim === 3 && here && (
        <Bar side="right">
          <Group after="Look">
            <a href={`/three/${share({ base, code: here.code, level: here.level })}`}>Open in Three</a>
          </Group>
        </Bar>
      )}
    </>
  );
}
