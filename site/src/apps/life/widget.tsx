import { useCallback, useMemo, useRef, useState } from "react";
import { Btn, Pick, Segment } from "../../lib/app/controls.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useReroll, useStatus } from "../../lib/app/hooks.ts";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Option, Scalar, Units } from "../../lib/app/spec.ts";
import { paints } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import type { Start } from "../../lib/life/engine.ts";
import { dimOf, maskOf, PATTERNS, SEEDS, sources, sow, spell, startOf } from "../../lib/life/engine.ts";
import { MaskView, Pane, useLife, usePaint } from "../../lib/life/parts.tsx";
import { SOUP, sprite } from "../../lib/life/scene.ts";
import { rng } from "../../lib/rand.ts";
import type { Life, Stat } from "./scene.ts";
import { make, PRESETS, RULES, SPEC, wears } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Life>> & Pick<WidgetProps<Units, Life>, "value" | "onChange">;

type Act = "step" | "doom" | "reset" | "clear";

const HAND: Option = ["", "By hand"];

const THUMB = 40;

const listed = (counts: readonly number[] | undefined) => (counts?.length ? counts.join(" ") : "none");

export const status = (stat: Omit<Stat, "mask"> | null): Readout[] =>
  stat
    ? [
        ["gen", stat.gen],
        ["pop", stat.pop],
        ["fate", stat.fate || "-"],
        ["rule", stat.name],
        ["born", listed(stat.counts?.[0])],
        ["stay", listed(stat.counts?.[1])],
        ["mask", stat.label],
        ["cells", stat.cells],
        ["lattice", stat.lattice],
        ...(stat.error ? [["error", stat.error] satisfies Readout] : []),
      ]
    : [];

const clean = (text: Scalar | null | undefined) =>
  String(text ?? "")
    .trim()
    .toLowerCase();

const has = (text: string, find: string) => clean(text).includes(clean(find));

const ink = (ctx: CanvasRenderingContext2D) => paints(ctx.canvas).accent;

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const units = useLife();
  const handle = useRef<Handle<Life> | null>(null);
  const [stat, setStat] = useState<Stat | null>(null);
  const [find, setFind] = useState("");
  const tick = useTheme();
  const paint = usePaint(handle);
  const names = useMemo(() => (units ? sources(units.life) : []), [units]);
  const options = useMemo(() => [HAND, ...names.map((one): Option => [one, one.replaceAll("_", " ")])], [names]);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, ...units!, onStat: setStat }),
    [units],
  );
  const ready = (scene: Handle<Life> | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const act = (name: Act) => () => handle.current?.[name]?.();
  const dim = dimOf(value);
  const chosen = (text: Scalar) => (names.includes(clean(text)) ? clean(text) : "");
  const from = (key: string, i: number, label: string) => (
    <Pick
      key={`${key}-from`}
      label={label}
      value={chosen(value[key])}
      options={options}
      onChange={(one) => onChange({ [key]: one || spell(stat?.counts?.[i] ?? []) })}
    />
  );
  useStatus(onStatus, status(stat));
  useReroll(value.seed as number, () => onChange({ from: "noise" }));
  const folds = [
    {
      name: "Board",
      items: [
        ...(SOUP(value) ? ["density"] : []),
        "speed",
        <Btn key="step" onClick={act("step")}>
          Step
        </Btn>,
        <Btn key="fate" onClick={act("doom")}>
          Run to fate
        </Btn>,
        <Btn key="reset" onClick={act("reset")}>
          Reset
        </Btn>,
        <Btn key="clear" onClick={act("clear")}>
          Clear
        </Btn>,
      ],
    },
    {
      name: "Rule",
      live: stat?.name,
      items: [
        "born",
        ...(units ? [from("born", 0, "Born from")] : []),
        "stay",
        ...(units ? [from("stay", 1, "Stay from")] : []),
        "wrap",
      ],
    },
    {
      name: "Mask",
      live: stat?.label,
      items: [
        <MaskView key="view" mask={stat?.mask} />,
        <Segment
          key="dim"
          label="Dim"
          value={dim}
          options={SPEC[0].options}
          onChange={(v) => onChange({ dim: Number(v), code: Number(v) === 1 ? "1" : "7" })}
        />,
        "number",
        "level",
      ],
    },
  ];
  const start = startOf(value);
  const soup = (id: string) => sow({ ...value, from: id }, rng(1), units!.math, THUMB, THUMB);
  const shapes = (list: readonly Start[]) => list.filter(([, name]) => has(name, find));
  const seeds = shapes(SEEDS.filter(([id]) => dim === 2 || id !== "design"));
  const patterns = dim === 2 ? shapes(Object.entries(PATTERNS).map(([id, { name }]): Start => [id, name])) : [];
  const worlds = PRESETS.filter(([label]) => has(label, find));
  const rules = RULES.filter(([label]) => has(label, find));
  return (
    <>
      {units && <Scene make={made} value={value} onReady={ready} {...paint} />}
      {units && (
        <Library find={find} onFind={setFind}>
          {patterns.length > 0 && (
            <Shelf name="Patterns" grid>
              {patterns.map(([id, name]) => (
                <Item
                  key={id}
                  label={name}
                  on={start === id}
                  onPick={() => onChange({ from: id })}
                  art={<Art paint={(ctx, w) => sprite(ctx, soup(id), THUMB, THUMB, w, ink(ctx))} deps={[id, tick]} />}
                />
              ))}
            </Shelf>
          )}
          {seeds.length > 0 && (
            <Shelf name="Seeds" grid={dim === 2}>
              {seeds.map(([id, name]) => (
                <Item
                  key={id}
                  label={name}
                  on={start === id}
                  onPick={() => onChange({ from: id })}
                  art={
                    dim === 2 ? (
                      <Art
                        paint={(ctx, w) => sprite(ctx, soup(id), THUMB, THUMB, w, ink(ctx))}
                        deps={[id, tick, value.code, value.number, value.density]}
                      />
                    ) : undefined
                  }
                />
              ))}
            </Shelf>
          )}
          {worlds.length > 0 && (
            <Shelf name="Worlds">
              {worlds.map(([label, one]) => (
                <Item
                  key={label}
                  label={label}
                  sub={`B${one.born}/S${one.stay}`}
                  on={wears(value, one)}
                  onPick={() => onChange(one)}
                  art={
                    <Art
                      paint={(ctx, w) => {
                        const { mask } = maskOf(units.life, one);
                        sprite(ctx, mask.data, mask.shape[1], mask.shape[0], w, ink(ctx));
                      }}
                      deps={[label, tick]}
                    />
                  }
                />
              ))}
            </Shelf>
          )}
          {rules.length > 0 && (
            <Shelf name="Rules">
              {rules.map(([label, one]) => (
                <Item
                  key={label}
                  label={label}
                  sub={`B${one.born}/S${one.stay}`}
                  on={wears(value, one)}
                  onPick={() => onChange(one)}
                />
              ))}
            </Shelf>
          )}
        </Library>
      )}
      <Pane spec={SPEC} value={value} onChange={onChange} folds={folds} />
    </>
  );
}
