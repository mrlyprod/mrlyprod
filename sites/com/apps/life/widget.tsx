import { paints } from "@mrlyprod/ui/kit/color.ts";
import { Btn, Pick, Segment, Toggle } from "@mrlyprod/ui/kit/controls.tsx";
import { useTheme } from "@mrlyprod/ui/kit/designs/pick.tsx";
import type { Readout } from "@mrlyprod/ui/kit/hooks.ts";
import { useStatus } from "@mrlyprod/ui/kit/hooks.ts";
import type { Chip } from "@mrlyprod/ui/kit/library.tsx";
import { Art, Item, Library, Shelf } from "@mrlyprod/ui/kit/library.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Option, Scalar, Seeded, Units } from "@mrlyprod/ui/kit/spec.ts";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { Start } from "../shared/life/engine.ts";
import { dimOf, maskOf, PATTERNS, roll, rolled, SEEDS, sources, sow, spell, startOf } from "../shared/life/engine.ts";
import { MaskView, Pane, useLife, usePaint } from "../shared/life/parts.tsx";
import { DESIGN, fixed, island, knobsOf, nameOf } from "../shared/life/recipe.ts";
import { SOUP, sprite } from "../shared/life/scene.ts";
import type { Life, Stat } from "./scene.ts";
import { DIMS, make, PRESETS, RULES, SPEC, wears } from "./scene.ts";
import type { Filter, Row } from "./shelves.tsx";
import { ALL, Encyclopedia } from "./shelves.tsx";

type Props = Partial<WidgetProps<Units, Life>> & Pick<WidgetProps<Units, Life>, "onChange"> & { value: Seeded };

type Act = "step" | "doom" | "reset" | "clear";

type Side = "born" | "stay";

const HAND: Option = ["", "By hand"];

const KINDS: Chip[] = [
  ["shelves", "Shelves"],
  ["rows", "Encyclopedia"],
];

const ROWS = "rows";

const THUMB = 40;

const LIBRARY = () => import("./library.gen.json", { with: { type: "json" } });

const listed = (counts: readonly number[] | undefined) => (counts?.length ? counts.join(" ") : "none");

export const status = (stat: Omit<Stat, "mask"> | null): Readout[] => {
  if (!stat) return [];
  const error = stat.error ? [["error", stat.error] satisfies Readout] : [];
  if ("recipe" in stat)
    return [
      ["gen", stat.gen],
      ["pop", stat.pop],
      ["stop", stat.fate || "-"],
      ["recipe", stat.recipe || "-"],
      ["born", `${stat.sides?.[0] || "-"}: ${listed(stat.counts?.[0])}`],
      ["stay", `${stat.sides?.[1] || "-"}: ${listed(stat.counts?.[1])}`],
      ["mask", stat.label || "-"],
      ["budget", stat.budget ?? 0],
      ["reach", stat.reach ?? 0],
      ...(stat.flips ? [["background", stat.background ? "full, flips" : "empty, flips"] satisfies Readout] : []),
      ...error,
    ];
  return [
    ["gen", stat.gen],
    ["pop", stat.pop],
    ["fate", stat.fate || "-"],
    ["rule", stat.name],
    ["born", listed(stat.counts?.[0])],
    ["stay", listed(stat.counts?.[1])],
    ["mask", stat.label],
    ["cells", stat.cells],
    ["lattice", stat.lattice],
    ...error,
  ];
};

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
  const [kind, setKind] = useState<Scalar>(KINDS[0][0]);
  const [rows, setRows] = useState<Row[] | null>(null);
  const [filter, setFilter] = useState<Filter>({ cls: "", family: ALL, stop: ALL });
  const tick = useTheme();
  const paint = usePaint(handle);
  const names = useMemo(() => (units ? sources(units.life) : []), [units]);
  const options = useMemo(() => [HAND, ...names.map((one): Option => [one, one.replaceAll("_", " ")])], [names]);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, life: units!.life, mrly: units!, onStat: setStat }),
    [units],
  );
  useEffect(() => {
    if (kind !== ROWS || rows) return undefined;
    let on = true;
    LIBRARY().then((mod) => on && setRows(mod.default.rows));
    return () => {
      on = false;
    };
  }, [kind, rows]);
  const ready = (scene: Handle<Life> | null) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const act = (name: Act) => () => handle.current?.[name]?.();
  const board = fixed(value);
  const alone = units ? island(units.life, value) : false;
  const dim = dimOf(value);
  const drawn = (key: Side) => (units ? rolled(units.life, value[key]) : null);
  const from = (key: Side, i: number, label: string) => {
    const side = drawn(key);
    return (
      <Pick
        key={`${key}-from`}
        label={label}
        value={side && names.includes(side.name) ? side.name : ""}
        options={options}
        onChange={(one) =>
          onChange({
            [key]: one
              ? roll(units!.life, { name: one, zeros: side?.zeros ?? false, ones: side?.ones ?? false })
              : spell(stat?.counts?.[i] ?? []),
          })
        }
      />
    );
  };
  const rolls = (key: Side) => {
    const side = drawn(key);
    if (!side) return [];
    return (["zeros", "ones"] as const).map((bit) => (
      <Toggle
        key={`${key}-${bit}`}
        label={bit[0].toUpperCase() + bit.slice(1)}
        value={side[bit] ? 1 : 0}
        onChange={(v) => onChange({ [key]: roll(units!.life, { ...side, [bit]: v === 1 }) })}
      />
    ));
  };
  useStatus(onStatus, status(stat));
  const buttons = [
    <Btn key="step" onClick={act("step")}>
      Step
    </Btn>,
    <Btn key="fate" onClick={act("doom")}>
      {board ? "Run to fate" : "Run to stop"}
    </Btn>,
    <Btn key="reset" onClick={act("reset")}>
      Reset
    </Btn>,
    ...(board
      ? [
          <Btn key="clear" onClick={act("clear")}>
            Clear
          </Btn>,
        ]
      : []),
  ];
  const folds = [
    { name: "Mode", items: ["mode"] },
    board
      ? { name: "Board", items: [...(SOUP(value) ? ["density"] : []), "speed", ...buttons] }
      : { name: "Run", live: stat?.recipe, items: ["speed", "x", ...(alone ? ["cap"] : []), ...buttons] },
    !board && { name: "Seed", items: ["shape", "side", "depth", ...(alone ? ["tiling"] : [])] },
    {
      name: "Rule",
      live: stat?.name,
      items: [
        "born",
        ...(units ? [from("born", 0, "Born from"), ...rolls("born")] : []),
        "stay",
        ...(units ? [from("stay", 1, "Stay from"), ...rolls("stay")] : []),
        ...(board ? ["wrap"] : []),
      ],
    },
    {
      name: "Mask",
      live: stat?.label,
      items: [
        <MaskView key="view" mask={stat?.mask} />,
        ...(board
          ? [
              <Segment
                key="dim"
                label="Dim"
                value={dim}
                options={DIMS}
                onChange={(v) => onChange({ dim: Number(v), code: Number(v) === 1 ? "1" : "7" })}
              />,
            ]
          : ["mask"]),
        ...(board || value.mask === DESIGN ? ["number", "level"] : []),
      ],
    },
  ];
  const start = startOf(value);
  const soup = (id: string) => sow({ ...value, from: id }, rng(1), units!, THUMB, THUMB);
  const shapes = (list: readonly Start[]) => list.filter(([, name]) => has(name, find));
  const seeds = shapes(SEEDS.filter(([id]) => dim === 2 || id !== "design"));
  const patterns = dim === 2 ? shapes(Object.entries(PATTERNS).map(([id, { name }]): Start => [id, name])) : [];
  const worlds = PRESETS.filter(([label, one]) => has(label, find) && (board || Number(one.dim) !== 1));
  const rules = RULES.filter(([label]) => has(label, find));
  const current = units ? nameOf(units.life, units, value) : "";
  const pickRow = (key: string) => onChange(knobsOf(units!.life, units!, units!.life.recipe.parse(key)));
  return (
    <>
      {units && <Scene make={made} value={value} onReady={ready} {...paint} />}
      {units && (
        <Library find={find} onFind={setFind} chips={KINDS} chip={kind} onChip={setKind}>
          {kind === ROWS && (
            <Encyclopedia
              rows={rows}
              life={units.life}
              find={find}
              filter={filter}
              onFilter={setFilter}
              current={current}
              onPick={pickRow}
              tick={tick}
            />
          )}
          {kind !== ROWS && board && patterns.length > 0 && (
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
          {kind !== ROWS && board && seeds.length > 0 && (
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
          {kind !== ROWS && worlds.length > 0 && (
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
          {kind !== ROWS && rules.length > 0 && (
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
      <Pane spec={SPEC} value={value} units={units ?? undefined} onChange={onChange} folds={folds} />
    </>
  );
}
