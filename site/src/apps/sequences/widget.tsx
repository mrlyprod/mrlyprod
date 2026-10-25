import type { CSSProperties, MouseEvent, PointerEvent, ReactNode } from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { text } from "../../lib/app/export.ts";
import { Bar } from "../../lib/app/frame.tsx";
import { useReroll, useUnit } from "../../lib/app/hooks.ts";
import type { Chip } from "../../lib/app/library.tsx";
import { Art, Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Make } from "../../lib/app/run.ts";
import type { Units } from "../../lib/app/spec.ts";
import { paints, veil } from "../../lib/color.ts";
import { useTheme } from "../../lib/designs/pick.tsx";
import { rng } from "../../lib/rand.ts";
import { KINDS } from "../../lib/sequences/chart.ts";
import type { Hit, Row } from "../../lib/sequences/engine.ts";
import {
  applies,
  csv,
  DEEP,
  filter,
  identify,
  json,
  keyed,
  lead,
  ledger,
  MOST,
  measureOf,
  numbers,
  parse,
  picks,
  plotted,
  search,
} from "../../lib/sequences/engine.ts";
import { facts, hit, RECORDS } from "../../lib/sequences/facts.ts";
import { glyph, title } from "../../lib/sequences/paint.ts";
import { Found, Record, shift } from "../../lib/sequences/parts.tsx";
import type { Feed, Held, Knobs, Stage } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

export type Note = readonly [string, ReactNode];

export type Asked = Knobs & { seed: number; q: string; pick: string };

type Contract = WidgetProps<Units, Held>;

export type Props = Pick<Contract, "value" | "onChange"> &
  Partial<Pick<Contract, "onReady" | "onExport">> & { onStatus?: (rows: Note[]) => void };

const SLICE = 8;
const PULSE = 90;
const PEEK = 6;
const SIDE = 3;
const PER = 50;
const HINT = "8, 21, 40 · carpet · A000567 · 23";
const MODES: Chip[] = [
  ["one", "One"],
  ["many", "Compare"],
];

const roll = (seed: number, n: number) => Math.floor(rng(seed >>> 0)() * n);

const terse = (terms: string[], count: number) =>
  terms.slice(0, count).join(", ") + (terms.length > count ? ", …" : "");

export const stamp = (cells: readonly number[]) => (ctx: CanvasRenderingContext2D, w: number, h: number) => {
  const shape = glyph(cells);
  if (!shape) return;
  const { accent } = paints(ctx.canvas);
  const pad = w * 0.08;
  const unit = (w - 2 * pad) / shape.width;
  const size = unit - Math.max(1, unit / 6);
  const y0 = (h - SIDE * unit) / 2;
  cells.forEach((v, i) => {
    ctx.fillStyle = v ? accent : veil(accent, 0.15);
    ctx.fillRect(
      pad + (Math.floor(i / (SIDE * SIDE)) * (SIDE + 1) + (i % SIDE)) * unit,
      y0 + (Math.floor(i / SIDE) % SIDE) * unit,
      size,
      size,
    );
  });
};

const hover = (row: Row, hit: Hit | null | undefined, typed: boolean) =>
  [
    typed ? "typed" : "",
    hit ? `${hit.status} ${hit.id}${shift(hit)} ${hit.name}` : "",
    row.terms.length ? terse(row.terms, PEEK) : "…",
  ]
    .filter(Boolean)
    .join(" · ");

const PAGER: CSSProperties = {
  display: "flex",
  gap: "var(--s2)",
  alignItems: "center",
  padding: "0 var(--s2)",
  fontSize: "var(--t2)",
};

const NOTE: CSSProperties = { padding: "0 var(--s2)", fontSize: "var(--t2)", lineHeight: 1.5 };

export function Widget({ value, onChange, onReady, onExport, onStatus }: Props) {
  const math = useUnit("math");
  const [tick, setTick] = useState(0);
  const [page, setPage] = useState(0);
  const [many, setMany] = useState(false);
  const theme = useTheme();
  const live = useRef<Feed>({ rows: [], version: 0 });
  const stage = useRef<Handle<Stage> | null>(null);
  const now = useRef(value);
  const tell = useRef(onChange);
  const listed = useRef<Row[]>([]);
  const chosen = useRef<Row[]>([]);
  now.current = value;
  tell.current = onChange;
  const book = useMemo(() => math && ledger(math), [math]);
  const { dim, base, measure, axis, chart, digits, terms, seed, q, pick } = value as Asked;
  const space = useMemo(
    () => (book ? filter(book.keys(dim, base), measure, axis) : []),
    [book, dim, base, measure, axis],
  );
  const homes = useMemo(() => new Set(space.map((row) => row.key)), [space]);
  const named = (needle: string) => {
    if (!book) return [];
    const found = [book.file(needle), ...keyed(RECORDS, needle).map((record) => book.file(record.key))].filter(
      Boolean,
    ) as Row[];
    return filter([...new Map(found.map((row) => [row.key, row])).values()], measure, axis);
  };
  const list = useMemo(
    () =>
      book
        ? search(space, q, { found: hit, typed: (code) => filter(book.typed(dim, base, code), measure, axis), named })
        : [],
    [book, space, q, tick],
  );
  listed.current = list;
  const listedKeys = useMemo(() => new Set(list.map((row) => row.key)), [list]);
  const keys = useMemo(() => picks(pick), [pick]);
  const rows = useMemo(() => {
    if (!book) return [];
    const own = keys.map((key) => book.row(parse(key)!));
    if (own.length) return own;
    return space.length ? [lead(book, space, roll(seed, space.length))!] : [];
  }, [book, keys, space, seed]);
  chosen.current = rows;
  const chosenKey = rows.map((row) => row.key).join(".");
  const write = (next: string[]) => tell.current?.({ pick: next.slice(0, MOST).join(".") });
  useReroll(seed, (next) => {
    if (!book) return;
    const from = listed.current.length ? listed.current : space;
    const row = from.length ? lead(book, from, roll(next, from.length), chosen.current[0]?.key) : null;
    if (row) write([row.key]);
  });
  const typed = numbers(q);
  const asked = typed && typed.length > 1;
  const first = list[0]?.key ?? "";
  useEffect(() => {
    if (!asked || !first || chosen.current.some((row) => listed.current.includes(row))) return;
    write([first]);
  }, [q, first]);
  useEffect(() => setPage(0), [dim, base, measure, axis, q]);
  useEffect(() => {
    const at = listed.current.findIndex((row) => row.key === chosen.current[0]?.key);
    if (at >= 0) setPage(Math.floor(at / PER));
  }, [chosenKey]);
  useEffect(() => {
    rows.forEach(hit);
    live.current = {
      rows: rows.map((row) => ({ ...row, terms: row.terms.slice(0, terms) })),
      version: live.current.version + 1,
    };
    stage.current?.draw?.();
  }, [rows, terms, tick]);
  useEffect(() => {
    if (!book) return undefined;
    let timer: ReturnType<typeof setTimeout> | 0 = 0;
    let gone = false;
    let last = 0;
    const work = () => {
      const end = performance.now() + SLICE;
      let more = true;
      let did = false;
      while (more && performance.now() < end) {
        more = chosen.current.some((row) => book.step(row, terms, DEEP.cells)) || book.sweep(dim, base);
        did ||= more;
      }
      if (did && (!more || performance.now() - last > PULSE)) {
        last = performance.now();
        setTick((n) => n + 1);
      }
      if (more && !gone) timer = setTimeout(work, 0);
    };
    work();
    return () => {
      gone = true;
      clearTimeout(timer);
    };
  }, [book, dim, base, terms, chosenKey]);
  const choose = (row: Row, e?: MouseEvent<HTMLButtonElement>) => {
    if (many || e?.shiftKey || e?.metaKey || e?.ctrlKey) return toggle(row);
    write([row.key]);
  };
  const toggle = (row: Row) => {
    const had = chosen.current.map((one) => one.key);
    if (had.includes(row.key) && had.length === 1) return;
    write(had.includes(row.key) ? had.filter((one) => one !== row.key) : [...had, row.key]);
  };
  const browse = (by: number) => {
    const at = listed.current.findIndex((row) => row.key === chosen.current[0]?.key);
    const next = listed.current[Math.max(0, Math.min(listed.current.length - 1, at + by))];
    if (next) write([next.key]);
  };
  const find = () => document.querySelector<HTMLInputElement>(".library .find input")?.focus();
  const cycle = () => {
    const at = KINDS.findIndex(([slug]) => slug === now.current.chart);
    tell.current?.({ chart: KINDS[(at + 1) % KINDS.length][0] });
  };
  const plot = () => {
    const row = chosen.current[0];
    if (row) location.assign(plotted(row, now.current.terms as number));
  };
  const ready = (handle: Handle<Stage> | null) => {
    stage.current = handle;
    onReady?.(handle && { ...handle, browse, find, cycle, plot });
  };
  const scene = useCallback<Make<Stage>>((canvas, view, opts) => make(canvas, view, { ...opts, live, math }), [math]);
  const tap = (e: PointerEvent<HTMLCanvasElement>) => {
    const handle = stage.current;
    if (!handle?.tap) return;
    const box = e.currentTarget.getBoundingClientRect();
    const k = e.currentTarget.width / (box.width || 1);
    const got = handle.tap((e.clientX - box.left) * k, (e.clientY - box.top) * k);
    if (got?.row && got.row > 0) {
      const had = chosen.current.map((one) => one.key);
      write([had[got.row], ...had.filter((_, i) => i !== got.row)]);
    }
  };
  const sheet = (kind: string) => {
    const data = chosen.current.map((row) => ({ ...row, terms: row.terms.slice(0, now.current.terms as number) }));
    if (data.length) text(kind === "csv" ? csv(data) : json(data), String(named(String(now.current))), kind);
  };
  const have = rows.length > 0;
  useEffect(
    () =>
      onExport?.(
        have
          ? [
              ["CSV", () => sheet("csv")],
              ["JSON", () => sheet("json")],
            ]
          : [],
      ),
    [have],
  );
  const head = rows[0];
  const told = book && head ? facts(book, head, terms) : null;
  const toldKey = told
    ? `${head.key}|${told.record?.id}|${told.record?.shift}|${told.status}|${told.terms}|${told.closed}|${told.found.map(({ record }) => record.id).join()}|${terms}`
    : "";
  useEffect(() => {
    if (!told) return onStatus?.([]);
    onStatus?.([
      ["record", told.record ? <Record key="record" hit={told.record} /> : "none"],
      ["status", told.status],
      ["closed form", told.closed],
      ["terms", told.terms],
      [
        "plot",
        <a key="plot" href={plotted(head, terms)}>
          draw this row
        </a>,
      ],
      ["space", told.space],
      ["axis", told.axis],
      ["name", head.file],
      ["found", <Found key="found" found={told.found} />],
    ]);
  }, [toldKey]);
  const done = book ? book.progress(dim, base) : { done: 0, total: 0 };
  const picked = measureOf(measure);
  const shown = [...rows.filter((row) => !listedKeys.has(row.key)), ...list];
  const pages = Math.max(1, Math.ceil(shown.length / PER));
  const at = Math.min(page, pages - 1);
  const slice = shown.slice(at * PER, (at + 1) * PER);
  const holders = asked ? identify(RECORDS, typed).slice(0, 3) : [];
  const label = head
    ? `The terms of ${title(head)}, charted as ${KINDS.find(([one]) => one === chart)?.[1].toLowerCase()} beside the design`
    : "Sequences";
  const go = () => listed.current[0] && write([listed.current[0].key]);
  const heading =
    picked && !applies(picked, dim, base)
      ? `no ${picked.label.toLowerCase()} in this space`
      : `${list.length} rows${done.done < done.total ? ` · reading ${done.done} of ${done.total}` : ""}`;
  return (
    <>
      {book && (
        <>
          <Scene
            make={scene}
            value={{ chart, digits }}
            onReady={ready}
            onPointerUp={tap}
            role="img"
            aria-label={label}
          />
          <Library
            name="Ledger"
            find={q}
            onFind={(next) => tell.current?.({ q: next })}
            onGo={go}
            hint={HINT}
            chips={MODES}
            chip={many ? "many" : "one"}
            onChip={(mode) => setMany(mode === "many")}
          >
            {asked && (
              <p className="dim" style={NOTE}>
                <Found found={holders} />
              </p>
            )}
            {pages > 1 && (
              <div style={PAGER}>
                <button type="button" disabled={at === 0} onClick={() => setPage(at - 1)}>
                  prev
                </button>
                <span className="mono">{`${at * PER + 1}-${Math.min(shown.length, (at + 1) * PER)} of ${shown.length}`}</span>
                <button type="button" disabled={at >= pages - 1} onClick={() => setPage(at + 1)}>
                  next
                </button>
              </div>
            )}
            <Shelf name={heading}>
              {slice.map((row) => {
                const mark = hit(row);
                const own = listedKeys.has(row.key) && !homes.has(row.key);
                return (
                  <Item
                    key={row.key}
                    label={`${row.label} · ${measureOf(row.measure)?.label} by ${row.axis}`}
                    sub={mark ? `${mark.id}${shift(mark)}` : ""}
                    title={hover(row, mark, own)}
                    on={rows.some((one) => one.key === row.key)}
                    onPick={(e) => choose(row, e)}
                    art={<Art paint={stamp(row.cells)} deps={[row.key, theme]} />}
                  />
                );
              })}
            </Shelf>
          </Library>
        </>
      )}
      {!book && (
        <Bar side="status">
          <span>loading</span>
        </Bar>
      )}
    </>
  );
}
