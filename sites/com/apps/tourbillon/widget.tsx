import { paints } from "@mrlyprod/ui/kit/color.ts";
import { Btn, Fold, Slider, Text } from "@mrlyprod/ui/kit/controls.tsx";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useCrates, useLatest, useStatus } from "@mrlyprod/ui/kit/hooks.ts";
import type { Paint } from "@mrlyprod/ui/kit/library.tsx";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Scalar } from "@mrlyprod/ui/kit/spec.ts";
import { useTouch } from "@mrlyprod/ui/kit/touch.ts";
import { useCallback, useMemo, useRef, useState } from "react";
import { fixed } from "../shared/moire/parts.tsx";
import { stamp } from "../shared/moire/sheet.ts";
import { EYES, eyeOf, KINDS, label, NUDGE, place, SCHEDULES, settle, shown, swatch, trim } from "./engine.ts";
import type { Fault, Player, Reading, Tourbillon } from "./scene.ts";
import { make } from "./scene.ts";

export type Told = Partial<Fault & Reading>;

export type ShelvesProps = { value: Tourbillon; onChange: (patch: Patch) => void; mrly?: Mrly | null };

export type Props = WidgetProps<Mrly, Player> & { value: Tourbillon; unit?: Mrly | null };

export function rows(f: Told | null): Readout[] {
  if (!f) return [];
  if (f.error) return [["error", f.error]];
  return [
    ["angle", `${place(f.angle!, 2)} deg`],
    ["layers", f.layers!],
    ["quarter turn", f.period === null ? "never closes" : `${f.period} increments`],
    ["classes", f.classes!],
    ["pairs sharing", f.pairs!],
    ["mean", fixed(f.mean!)],
    ["rms", fixed(f.rms!)],
    ["rms x root L", fixed(f.faded!)],
    ["centre", fixed(f.centre!)],
    ["scales", f.scales!.join(" ")],
    ["angles", f.angles!.map((a) => place(a, 1)).join(" ")],
    ["peaks", f.peaks!.map(([x, y, v]) => `${fixed(x)} ${fixed(y)} ${fixed(v)}`).join(", ")],
    ...(f.unused ? [["weights", "unused by this blend"] satisfies Readout] : []),
    ["pixels", `${f.size} by ${f.size}`],
  ];
}

export const typed = (text: string) => {
  const v = Number(text);
  return String(text).trim() && Number.isFinite(v) && v >= 0 && v <= 360 ? settle(v) : null;
};

export function Shelves({ value, onChange, mrly }: ShelvesProps) {
  const [kind, setKind] = useState<Scalar>("unit");
  const [find, setFind] = useState("");
  const tick = useTheme();
  const eyes = useMemo(() => (mrly ? mrly.tourbillon.eyes(EYES) : []), [mrly]);
  const now = eyeOf(eyes, value.increment);
  const look = [
    value.limit,
    value.set,
    value.weights,
    value.mode,
    value.blend,
    value.ramp,
    value.levels,
    value.invert,
    value.tint,
    value.seed,
    tick,
  ];
  const thumb =
    (over: Partial<Tourbillon>, at: number): Paint =>
    (ctx, w) =>
      stamp(ctx, swatch(mrly!, { ...value, ...over }, at, w, paints(ctx.canvas, value.tint)), w);
  return (
    <Library chips={KINDS} chip={kind} onChip={setKind} find={find} onFind={kind === "all" ? setFind : undefined}>
      {mrly && kind === "sched" && (
        <Shelf name="Schedules">
          {SCHEDULES.map(([key, text]) => (
            <Item
              key={key}
              label={text}
              on={value.sched === key}
              onPick={() => onChange({ sched: key })}
              art={<Art paint={thumb({ sched: key }, value.increment)} deps={[key, value.increment, ...look]} />}
            />
          ))}
        </Shelf>
      )}
      {mrly && kind !== "sched" && (
        <Shelf name="Quarter turns" grid>
          {shown(eyes, kind, find).map(([eye, at]) => (
            <Item
              key={at}
              label={trim(eye.angle)}
              sub={eye.denom === 1 ? "" : `${eye.numer}/${eye.denom}`}
              title={label(eye)}
              on={now === String(at)}
              onPick={() => onChange({ increment: settle(eye.angle), sched: "degrees" })}
              art={<Art px={64} paint={thumb({ sched: "degrees" }, eye.angle)} deps={[at, ...look]} />}
            />
          ))}
        </Shelf>
      )}
    </Library>
  );
}

export function Widget({ value, onChange, onReady, onStatus, unit }: Props) {
  const mrly = useCrates(CRATES, unit);
  const [facts, setFacts] = useState<Told | null>(null);
  const handle = useRef<Handle<Player> | null>(null);
  const change = useLatest(onChange);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, mrly, onFacts: setFacts, onTurn: (patch) => change.current(patch) }),
    [mrly],
  );
  const ready = (scene: Handle<Player> | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const nudge = (by: number) => handle.current?.nudge?.(by);
  const touch = useTouch(handle, change);
  useStatus(onStatus, rows(facts));
  const shownNow = facts?.playing ? facts.angle! : value.increment;
  const sched = SCHEDULES.find(([key]) => key === value.sched)?.[1] ?? value.sched;
  const write = (text: string) => {
    const v = typed(text);
    if (v !== null) onChange({ increment: v });
  };
  return (
    <>
      {mrly && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!mrly && <span>loading</span>}</Bar>
      <Shelves value={value} onChange={onChange} mrly={mrly} />
      <Bar side="right">
        <Fold name="Spin" live={`${place(shownNow, 2)} deg · ${sched}`}>
          <Slider
            label="Increment"
            value={Number(shownNow.toFixed(2))}
            min={0}
            max={360}
            step={0.5}
            unit="deg"
            onChange={(v) => onChange({ increment: settle(v) })}
          />
          <Text label="Degrees" inputMode="decimal" value={trim(shownNow)} commit onChange={write} />
          <Btn disabled={!mrly} onClick={() => nudge(-1)}>{`Nudge down ${NUDGE} deg`}</Btn>
          <Btn disabled={!mrly} onClick={() => nudge(1)}>{`Nudge up ${NUDGE} deg`}</Btn>
        </Fold>
      </Bar>
    </>
  );
}
