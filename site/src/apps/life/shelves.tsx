import { useMemo } from "react";
import type { Chip } from "../../lib/app/library.tsx";
import { Art, Chips, Item, Shelf } from "../../lib/app/library.tsx";
import { paints } from "../../lib/color.ts";
import type { Door } from "../../lib/life/engine.ts";
import { grammar } from "../../lib/life/engine.ts";
import { thumb } from "../../lib/life/recipe.ts";

/* TYPES */

export type Row = {
  key: string;
  fate: string;
  class: string;
  family: string;
  gen: number;
  period: number;
  pop: { last: number; max: number[] };
};

export type Library = { shelves: number; per_cell: number; rows: Row[] };

export type Filter = { cls: string; family: string; stop: string };

export type Props = {
  rows: readonly Row[] | null;
  life: Door;
  find: string;
  filter: Filter;
  onFilter: (filter: Filter) => void;
  current: string;
  onPick: (key: string) => void;
  tick?: number;
};

export const ALL = "all";

export const STOPS = ["dead", "still", "loop", "mover", "wall", "limit"];

const THUMB = 40;

const word = (token: string) => token[0].toUpperCase() + token.slice(1);

const has = (text: string, find: string) => text.toLowerCase().includes(find.trim().toLowerCase());

const first = <T,>(list: readonly T[]): T[] => [...new Set(list)];

export function classes(rows: readonly Row[]): string[] {
  return first(rows.map((row) => row.class));
}

export function families(rows: readonly Row[], shapes: readonly string[]): string[] {
  return first(rows.map((row) => row.family)).sort((a, b) => shapes.indexOf(a) - shapes.indexOf(b));
}

export function shelved(rows: readonly Row[], filter: Filter, find: string): [string, Row[]][] {
  const cls = filter.cls || classes(rows)[0];
  const kept = rows.filter(
    (row) =>
      row.class === cls &&
      (filter.family === ALL || row.family === filter.family) &&
      (filter.stop === ALL || row.fate === filter.stop) &&
      has(row.key, find),
  );
  return STOPS.map((stop): [string, Row[]] => [stop, kept.filter((row) => row.fate === stop)]).filter(
    ([, list]) => list.length > 0,
  );
}

export function Encyclopedia({ rows, life, find, filter, onFilter, current, onPick, tick = 0 }: Props) {
  const shapes = useMemo(() => grammar(life).shapes, [life]);
  const seeds = useMemo(() => new Map((rows ?? []).map((row) => [row.key, life.recipe.parse(row.key)])), [rows, life]);
  if (!rows) return <Shelf name="Encyclopedia" />;
  const kinds = classes(rows);
  const cls = filter.cls || kinds[0];
  const chip = (list: readonly string[], all: boolean): Chip[] => [
    ...(all ? [[ALL, "All"] as Chip] : []),
    ...list.map((one): Chip => [one, word(one)]),
  ];
  return (
    <>
      <Chips
        label="Class"
        chips={chip(kinds, false)}
        chip={cls}
        onChip={(v) => onFilter({ ...filter, cls: String(v) })}
      />
      <Chips
        label="Family"
        chips={chip(families(rows, shapes), true)}
        chip={filter.family}
        onChip={(v) => onFilter({ ...filter, family: String(v) })}
      />
      <Chips
        label="Stop"
        chips={chip(
          STOPS.filter((stop) => rows.some((row) => row.fate === stop)),
          true,
        )}
        chip={filter.stop}
        onChip={(v) => onFilter({ ...filter, stop: String(v) })}
      />
      {shelved(rows, { ...filter, cls }, find).map(([stop, list]) => (
        <Shelf key={stop} name={`${word(stop)} (${list.length})`}>
          {list.map((row) => {
            const seed = seeds.get(row.key);
            return (
              <Item
                key={row.key}
                label={seed ? `${seed.seed} ${seed.side}.${seed.level} · ${seed.mask}` : row.key}
                sub={`${row.gen}, p${row.period}, ${row.pop.last}`}
                title={row.key}
                on={current === row.key}
                onPick={() => onPick(row.key)}
                art={
                  <Art
                    px={THUMB}
                    paint={(ctx, w) => {
                      const { accent, paper } = paints(ctx.canvas);
                      thumb(ctx, life, row.key, w, accent, paper);
                    }}
                    deps={[row.key, tick]}
                  />
                }
              />
            );
          })}
        </Shelf>
      ))}
    </>
  );
}
