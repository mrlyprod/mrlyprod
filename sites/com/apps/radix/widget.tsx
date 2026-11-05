import { paints } from "@mrlyprod/ui/kit/color.ts";
import { Btn, Fold, Pick, Text } from "@mrlyprod/ui/kit/controls.tsx";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import { thumb } from "@mrlyprod/ui/kit/designs/thumb.ts";
import { text } from "@mrlyprod/ui/kit/export.ts";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useUnits } from "@mrlyprod/ui/kit/hooks.ts";
import type { Change } from "@mrlyprod/ui/kit/knobs.tsx";
import { Knob } from "@mrlyprod/ui/kit/knobs.tsx";
import type { Paint } from "@mrlyprod/ui/kit/library.tsx";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch } from "@mrlyprod/ui/kit/spec.ts";
import type { ReactNode } from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { Design, Dialed, Facts, Pair, Word } from "./engine.ts";
import {
  BASES,
  BOX,
  boxed,
  boxes,
  boxOf,
  cap,
  coded,
  DIM,
  dial,
  every,
  flat,
  LETTERS,
  PRESETS,
  pair,
  RINGS,
  rebase,
  spell,
  toggle,
  turn,
  twisted,
  wears,
} from "./engine.ts";
import type { Doors, Knobs, Listed, Moves, Radix, Sketch } from "./scene.ts";
import { file, make, SPEC, presets as shelf, sketch, units as UNITS } from "./scene.ts";

type Put = (patch: Patch | null) => void;

export type ChipsProps = { dialed: Dialed | null; onToggle: (cls: number) => void };

export type PaneProps = {
  value: Knobs;
  units: Doors | null;
  dialed: Dialed | null;
  facts: Facts | null;
  label: string;
  onChange: Change;
  change: Put;
  moves: Pick<Moves, "flat" | "every">;
};

export type DesignsProps = {
  value: Knobs;
  units: Doors;
  dialed: Dialed | null;
  presets: Listed[];
  name: string;
  change: Put;
};

type Contract = WidgetProps<Doors, Radix>;

type Props = Partial<Contract> & Pick<Contract, "value" | "onChange"> & { value: Knobs & { seed: number } };

type Now = {
  value: Knobs & { seed: number };
  onChange: Contract["onChange"];
  dialed: Dialed | null;
  units: Doors | null;
  name: string;
};

const row = (key: string) => SPEC.find((one) => one.key === key);

export const rows = (f: Facts | null): Readout[] =>
  f
    ? [
        ["level", `${f.level} of ${f.cap}`],
        ["fill", f.fill],
        ["distinct", f.distinct],
        ["glue", f.glued ? "fewer points than words" : "none, one point a word"],
        ["dimension", f.dimension.toFixed(6)],
        ["base", `${f.base} in ${f.ring}, norm ${f.norm}`],
        ["digits", `${f.digits} of ${f.norm} classes`],
        ["units", `${f.units} on this lattice`],
        ["code", f.code],
        ["canonical", f.canonical ? "every digit" : "not every digit"],
        ...(f.error ? [["error", f.error] satisfies Readout] : []),
      ]
    : [];

export const degrees = (k: number, n: number) => `${Math.round((360 * k) / n)}°`;

export const deeper = (value: { level: number; d?: unknown }) => (value.level >= cap(value) ? 1 : value.level + 1);

export function Chips({ dialed, onToggle }: ChipsProps) {
  if (!dialed) return null;
  const { letter, residues, held } = dialed;
  return (
    <div className="designs" role="group" aria-label="Digits">
      {residues.map((z, cls) => {
        const one = held.find((h) => h.cls === cls);
        const word = spell(letter, ...(one?.z ?? z));
        return (
          <Btn
            key={z.join(":")}
            on={Boolean(one)}
            aria-pressed={Boolean(one)}
            disabled={Boolean(one) && held.length === 1}
            title={`class ${cls}: ${one ? `digit ${word} held` : `${word} out`}`}
            onClick={() => onToggle(cls)}
          >
            {word}
          </Btn>
        );
      })}
    </div>
  );
}

export function Pane({ value, units, dialed, facts, label, onChange, change, moves }: PaneProps) {
  const letter = dialed?.letter ?? "w";
  const held = dialed?.held ?? [];
  const turned = held.filter(({ u }) => u > 0).length;
  const fold = (title: string, summary: string, children: ReactNode) => (
    <Fold key={title} name={title} live={summary}>
      {children}
    </Fold>
  );
  return [
    fold(
      "Design",
      `${label} · level ${value.level} · ${value.line ? "line" : "cells"}`,
      <>
        <Chips dialed={dialed} onToggle={(cls) => change(toggle(dialed!, cls))} />
        <span className="tabs">
          <Btn disabled={!dialed} onClick={moves.every}>
            Every residue
          </Btn>
          <Btn disabled={!twisted(value)} onClick={moves.flat}>
            Drop the twists
          </Btn>
        </span>
        <Knob row={row("level")!} value={value.level} values={value} onChange={onChange} />
        <Knob row={row("line")!} value={value.line} values={value} onChange={onChange} />
      </>,
    ),
    fold(
      "Twists",
      turned ? `${turned} of ${held.length} turned` : "none",
      held.map(({ cls, z, u }) => (
        <Pick
          key={cls}
          label={`Twist on ${spell(letter, ...z)}`}
          value={String(u)}
          options={dialed!.units.map((w, k) => [
            String(k),
            `${degrees(k, dialed!.units.length)} · ${spell(letter, ...w)}`,
          ])}
          onChange={(v) => change(turn(dialed!, cls, Number(v)))}
        />
      )),
    ),
    fold(
      "Code",
      [`Z[${letter}]`, spell(letter, ...(pair(value.base) as Pair)), facts?.code].filter(Boolean).join(" · "),
      <Text
        label="Code"
        value={facts?.code ?? ""}
        commit
        inputMode="numeric"
        onChange={(code) => dialed && change(coded(dialed, units!.math, code))}
      />,
    ),
  ];
}

const titles = Object.fromEntries(PRESETS.map(([id, title]) => [id, title]));

const cells = (patch: Design) => ({ ...patch, line: 0 });

export function Designs({ value, units, dialed, presets, name, change }: DesignsProps) {
  const tick = useTheme();
  const [find, setFind] = useState("");
  const { num, math } = units;
  const want = find.trim().toLowerCase();
  const hit = (...words: string[]) => !want || words.some((one) => one.toLowerCase().includes(want));
  const lattices = useMemo(
    () =>
      RINGS.map(([ring, title]): [Word, string, [string, string, string, Sketch][]] => {
        const known = num.gauss.Ring.named(ring) ?? "Gaussian";
        return [
          ring,
          title,
          BASES[ring].map((base) => [
            base,
            spell(LETTERS[ring], ...(pair(base) as Pair)),
            `norm ${num.gauss.Ring.norm(known, ...(pair(base) as Pair))}`,
            cells(rebase(num, math, ring, base)),
          ]),
        ];
      }),
    [units],
  );
  const squares = useMemo(() => boxes(math), [units]);
  const box = dialed ? boxOf(dialed) : "";
  const deps = [units, value.tint, tick];
  const draw =
    (patch: Sketch): Paint =>
    (ctx, w, _h, dpr) =>
      sketch(ctx, units, patch, w, dpr, paints(ctx.canvas, value.tint).accent);
  const square = ([code, word]: [string, string]) => (
    <Item
      key={code}
      label={word || code}
      sub={word ? code : undefined}
      on={box === code}
      onPick={() => change(cells(boxed(num, math, code)))}
      art={
        <Art
          deps={[...deps, code]}
          paint={(ctx, w) => thumb(ctx, math, DIM, BOX, code, w, paints(ctx.canvas, value.tint))}
        />
      }
    />
  );
  const starts = presets.filter(([id]) => hit(titles[id]));
  const named = squares.named.filter(([code, word]) => hit(word, code));
  const rest = squares.rest.filter(([code]) => hit(code));
  return (
    <Library name="Designs" find={find} onFind={setFind} hint="Search designs">
      {starts.length > 0 && (
        <Shelf name="Presets" grid>
          {starts.map(([id, patch]) => (
            <Item
              key={id}
              label={titles[id]}
              on={name === id}
              onPick={() => change(patch)}
              art={<Art deps={[...deps, id]} paint={draw(patch!)} />}
            />
          ))}
        </Shelf>
      )}
      {lattices.map(([ring, title, list]) => {
        const kept = list.filter(([, word, norm]) => hit(title, word, norm));
        return (
          kept.length > 0 && (
            <Shelf key={ring} name={title} grid>
              {kept.map(([base, word, norm, patch]) => (
                <Item
                  key={base}
                  label={word}
                  sub={norm}
                  on={value.ring === ring && value.base === base}
                  onPick={() => change(patch)}
                  art={<Art deps={[...deps, ring, base]} paint={draw(patch)} />}
                />
              ))}
            </Shelf>
          )
        );
      })}
      {named.length > 0 && (
        <Shelf name="Named boxes" grid>
          {named.map(square)}
        </Shelf>
      )}
      {rest.length > 0 && (
        <Shelf name="Other boxes" grid>
          {rest.map(square)}
        </Shelf>
      )}
    </Library>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units: given }: Props) {
  const units = useUnits(UNITS, given);
  const live = useRef<Handle<Radix> | null>(null);
  const [facts, setFacts] = useState<Facts | null>(null);
  const dialed = useMemo(
    () => units && dial(units.num, units.math, value),
    [units, value.ring, value.base, value.d, value.t],
  );
  const presets = useMemo(() => (units ? shelf(units) : []), [units]);
  const name = presets.find(([, one]) => wears(value, one))?.[0] ?? "";
  const label = PRESETS.find(([one]) => one === name)?.[1] ?? "Custom";
  const now = useRef<Now | null>(null);
  now.current = { value, onChange, dialed, units, name };
  const change = (patch: Patch | null) => patch && now.current!.onChange(patch);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units, onFacts: setFacts }),
    [units],
  );
  const moves: Moves = {
    flat: () => now.current!.dialed && change(flat(now.current!.dialed)),
    every: () => now.current!.dialed && change(every(now.current!.dialed)),
  };
  const ready = (handle: Handle<Radix> | null) => {
    live.current = handle;
    if (!handle) setFacts(null);
    onReady?.(handle);
  };
  const told: Readout[] = facts ? [["design", label], ...rows(facts)] : [];
  const key = JSON.stringify(told);
  useEffect(() => onStatus?.(told), [key]);
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current!.value), "svg");
  };
  const have = Boolean(units);
  useEffect(() => onExport?.(have ? [["SVG", keep]] : []), [have]);
  const spelled = spell(dialed?.letter ?? "w", ...(pair(value.base) as Pair));
  return (
    <>
      {units && (
        <Scene
          make={scene}
          value={value}
          onReady={ready}
          onClick={() => change({ level: deeper(value) })}
          role="img"
          aria-label={`The level ${value.level} words of the radix design ${label} at base ${spelled}`}
        />
      )}
      {units && <Designs value={value} units={units} dialed={dialed} presets={presets} name={name} change={change} />}
      <Bar side="right">
        <Pane
          value={value}
          units={units}
          dialed={dialed}
          facts={facts}
          label={label}
          onChange={onChange}
          change={change}
          moves={moves}
        />
      </Bar>
    </>
  );
}
