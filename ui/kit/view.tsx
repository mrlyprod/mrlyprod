import { rows, site } from "site:routes";
import { appOf, named, SLICE, verbOf } from "@mrlyprod/os/contract/index.ts";
import type {
  ChartNode,
  Field as Column,
  DoNode,
  FormNode,
  Json,
  Manifest,
  Move,
  Obj,
  Rank,
  RowsNode,
  Schema,
  Shown,
  Source,
  ValueNode,
  ViewerNode,
  ViewNode,
} from "@mrlyprod/os/kernel/index.ts";
import { canon, check } from "@mrlyprod/os/kernel/index.ts";
import type { ReactNode } from "react";
import { Fragment, lazy, Suspense, useEffect, useMemo, useRef, useState } from "react";
import type { Look, Model } from "./chart.ts";
import { draw } from "./chart.ts";
import { paints } from "./color.ts";
import { Btn, Fold, Group, Pick, Slider, Text, Toggle } from "./controls.tsx";
import { fed, source as feed } from "./feed.ts";
import { dig, useFile } from "./files.ts";
import { island } from "./island.tsx";
import type { Outcome, Signed } from "./world.ts";
import { useApp, useCall, useMoves, useReads, useView, useWho, useWorld } from "./world.ts";

type FieldProps = { name: string; schema: Schema; value: Json; onChange: (value: Json) => void };

type Choice = [string, string];

type FormProps = {
  move: Move;
  send: (args: Obj) => Outcome;
  choices?: Record<string, Choice[]>;
  rank?: Rank;
  note?: string | null;
};

const SLIDES = 10000;

export function first(schema: Schema): Json {
  if (schema.default !== undefined) return schema.default as Json;
  if (schema.const !== undefined) return schema.const as Json;
  if (schema.enum?.length) return schema.enum[0] as Json;
  if (schema.type === "integer" || schema.type === "number") return schema.minimum ?? 0;
  if (schema.type === "boolean") return false;
  if (schema.type === "string") return "";
  if (schema.type === "array") return [];
  if (schema.type === "object" || schema.properties)
    return Object.fromEntries(
      Object.entries(schema.properties ?? {})
        .filter(([key]) => schema.required?.includes(key))
        .map(([key, one]) => [key, first(one)]),
    );
  return null;
}

const shown = (value: Json) => (typeof value === "string" ? value : canon(value));

const tail = (verb: string) => verb.slice(verb.lastIndexOf(".") + 1);

export function Tree({ value }: { value: Json }): ReactNode {
  if (value === null || typeof value !== "object") return <span className="leaf">{shown(value)}</span>;
  if (Array.isArray(value))
    return value.length ? (
      <ol className="tree">
        {value.map((one, i) => (
          <li key={i}>
            <Tree value={one} />
          </li>
        ))}
      </ol>
    ) : (
      <span className="leaf">none</span>
    );
  const keys = Object.keys(value);
  if (!keys.length) return <span className="leaf">none</span>;
  return (
    <dl className="tree">
      {keys.map((key) => (
        <Fragment key={key}>
          <dt>{key}</dt>
          <dd>
            <Tree value={value[key]!} />
          </dd>
        </Fragment>
      ))}
    </dl>
  );
}

export function Field({ name, schema, value, onChange }: FieldProps) {
  const label = schema.title ?? name;
  const known = { name, "aria-label": label };
  const number = (text: string) => {
    const n = Number(text);
    if (text.trim() && Number.isFinite(n)) onChange(n);
  };
  if (schema.enum) {
    const options = schema.enum.map((one): [string, string] => [canon(one), shown(one as Json)]);
    const pick = (key: string) => onChange(schema.enum!.find((one) => canon(one) === key) as Json);
    return <Pick {...known} label={label} value={canon(value)} options={options} onChange={pick} />;
  }
  if (schema.type === "integer" || schema.type === "number") {
    const { minimum: min, maximum: max } = schema;
    if (min === undefined || max === undefined || max - min > SLIDES)
      return <Text {...known} label={label} value={shown(value)} commit onChange={number} />;
    const step = schema.multipleOf ?? (schema.type === "integer" ? 1 : (max - min) / 100);
    return (
      <Slider {...known} label={label} value={Number(value)} min={min} max={max} step={step} onChange={onChange} />
    );
  }
  if (schema.type === "boolean")
    return <Toggle {...known} label={label} value={value ? 1 : 0} onChange={(on) => onChange(on === 1)} />;
  if (schema.type === "string")
    return (
      <Text {...known} label={label} value={String(value)} wide={(schema.maxLength ?? 0) > 40} onChange={onChange} />
    );
  const parse = (text: string) => {
    try {
      onChange(JSON.parse(text) as Json);
    } catch {}
  };
  return <Text {...known} label={label} value={canon(value)} wide commit onChange={parse} />;
}

export function Form({ move, send, choices = {}, rank, note }: FormProps) {
  const props = move.args.properties ?? {};
  const keys = Object.keys(props);
  const need = new Set(move.args.required ?? []);
  const [draft, setDraft] = useState<Obj>({});
  const value = (key: string) => {
    const one = props[key]!;
    if (choices[key]?.length) return key in draft ? draft[key]! : choices[key][0]![0];
    return key in draft && check(one, draft[key]) === null ? draft[key]! : first(one);
  };
  const press = () => {
    const out = send(
      Object.fromEntries(keys.filter((key) => need.has(key) || key in draft).map((key) => [key, value(key)])),
    );
    if (out.ok) setDraft(Object.fromEntries(Object.entries(draft).filter(([key]) => props[key]?.type !== "string")));
  };
  const set = (key: string) => (one: Json) => setDraft({ ...draft, [key]: one });
  const body = (
    <>
      <p className="sub">{move.line}</p>
      {keys.map((key) =>
        choices[key]?.length ? (
          <Pick
            key={key}
            name={key}
            aria-label={props[key]!.title ?? key}
            label={props[key]!.title ?? key}
            value={String(value(key))}
            options={choices[key]}
            onChange={set(key)}
          />
        ) : (
          <Field key={key} name={key} schema={props[key]!} value={value(key)} onChange={set(key)} />
        ),
      )}
      <Btn primary={!rank || rank === "primary"} onClick={press}>
        {tail(move.verb)}
      </Btn>
      {rank && (
        <p className="note" role="status">
          {note}
        </p>
      )}
    </>
  );
  const name = rank ? named(verbOf(move.verb)) : move.verb;
  return rank === "rare" ? (
    <Fold name={name} data-verb={move.verb}>
      {body}
    </Fold>
  ) : (
    <Group name={name} data-verb={move.verb}>
      {body}
    </Group>
  );
}

const spelled = (value: Json): string => {
  if (value === null) return "";
  if (Array.isArray(value)) return value.map(spelled).join(", ");
  if (typeof value === "object")
    return Object.entries(value)
      .map(([key, one]) => `${key} ${spelled(one)}`)
      .join(", ");
  return String(value);
};

const DATED = { dateStyle: "medium", timeStyle: "short" } as const;

const UNITS = ["B", "kB", "MB", "GB", "TB"];

const sized = (n: number) => {
  const at = Math.min(Math.floor(Math.log10(Math.max(n, 1)) / 3), UNITS.length - 1);
  return `${(n / 1000 ** at).toLocaleString(undefined, { maximumFractionDigits: 1 })} ${UNITS[at]}`;
};

const when = (value: Json, unit?: string) => {
  if (typeof value === "string") {
    const at = Date.parse(value);
    return Number.isNaN(at) ? value : new Date(at).toLocaleString(undefined, DATED);
  }
  if (typeof value !== "number") return spelled(value);
  if (!value) return "start";
  return new Date(unit === "second" ? value * 1000 : value).toLocaleString(undefined, DATED);
};

const SHOWN: Record<Exclude<Shown, "time">, (value: Json) => string> = {
  text: spelled,
  number: (value) =>
    typeof value === "number" ? value.toLocaleString(undefined, { maximumFractionDigits: 1 }) : spelled(value),
  person: (value) => (typeof value === "string" ? named(value) : spelled(value)),
};

const unitOf = (value: Json, unit?: string) => (unit ? ` ${unit}${value === 1 ? "" : "s"}` : "");

const told = (as: Shown, value: Json, unit?: string) => {
  if (as === "time") return when(value, unit);
  if (unit === "byte" && typeof value === "number") return sized(value);
  return `${SHOWN[as](value)}${unitOf(value, unit)}`;
};

const nameOf = (source: Source) => ("file" in source ? `${source.file}#${source.path ?? ""}` : source.world);

const useSource = (source: Source): Json | null => {
  const world = useReads("world" in source ? [source.world] : [])[0] ?? null;
  const file = useFile("file" in source ? source.file : null);
  if ("file" in source) return dig(file, (source.path ?? "").split("/").filter(Boolean));
  return world;
};

const called = (one: Json) =>
  one && typeof one === "object" && !Array.isArray(one) && typeof one.name === "string" ? one.name : spelled(one);

function ValuePart({ node }: { node: ValueNode }) {
  const value = useSource(node.from);
  if (value === null) return null;
  return (
    <p className="value" data-from={nameOf(node.from)}>
      <span>{node.title}</span>
      <strong>{told(node.as, value, node.unit)}</strong>
    </p>
  );
}

function RowsPart({ node }: { node: RowsNode }) {
  const value = useSource(node.from);
  const list = Array.isArray(value) ? value : [];
  const shown = node.reverse ? [...list].reverse() : list;
  const fields = node.fields;
  const mapped = fields.filter((one) => one.names);
  const maps = useReads(mapped.map((one) => one.names!.world));
  const cell = (row: Obj, one: Column) => {
    const value = row[one.key] ?? null;
    const map = maps[mapped.indexOf(one)];
    if (typeof value === "string" && map && typeof map === "object" && !Array.isArray(map) && Object.hasOwn(map, value))
      return called(map[value]!);
    return told(one.as, value, one.unit);
  };
  return (
    <>
      {node.title && <h2>{node.title}</h2>}
      {shown.length ? (
        <table className="rows" data-from={nameOf(node.from)}>
          <thead>
            <tr>
              {fields.map((one) => (
                <th key={one.key} className={one.as}>
                  {one.title ?? one.key}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {shown.map((row, i) => (
              <tr key={i}>
                {fields.map((one) => (
                  <td key={one.key} className={one.as}>
                    {cell(row as Obj, one)}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <p className="reads" data-from={nameOf(node.from)}>
          none
        </p>
      )}
    </>
  );
}

const picks = (value: Json | null): Choice[] => {
  if (Array.isArray(value)) return value.map((one) => [spelled(one), spelled(one)]);
  if (value && typeof value === "object") return Object.entries(value).map(([id, one]) => [id, called(one)]);
  return [];
};

const joined = (reads: (Json | null)[], me: string | undefined): Choice[] => {
  if (reads.includes(null)) return [];
  const all = new Map(reads.flatMap(picks));
  if (me) all.delete(me);
  return [...all];
};

function FormPart({ node }: { node: FormNode }) {
  const move = useMoves(appOf(node.verb)).find((one) => one.verb === node.verb);
  const given = Object.entries(node.choices ?? {}).map(([key, one]) => [key, [one].flat()] as const);
  const read = useReads(given.flatMap(([, list]) => list.map((source) => source.world)));
  const signed = useWho();
  const call = useCall();
  const [note, setNote] = useState<string | null>(null);
  if (!move) return null;
  const send = (args: Obj) => {
    const out = call(node.verb, args);
    const said = out.envelope?.said;
    const line = said && typeof said === "object" && !Array.isArray(said) ? said.note : said;
    setNote(out.note ?? (line === undefined ? null : spelled(line)));
    return out;
  };
  let at = 0;
  const choices = Object.fromEntries(
    given.map(([key, list]) => {
      const reads = read.slice(at, at + list.length);
      at += list.length;
      return [key, joined(reads, signed?.who.id)];
    }),
  );
  return <Form move={move} send={send} choices={choices} rank={node.rank} note={note} />;
}

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

function Plot({ model }: { model: Model }) {
  const at = useRef<HTMLCanvasElement>(null);
  const latest = useRef(model);
  latest.current = model;
  const paint = useRef(() => {});
  useEffect(() => {
    const canvas = at.current!;
    const drop = feed(canvas);
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
  return <canvas ref={at} className="plot" role="img" aria-label={model.label ?? model.text} />;
}

const modelsOf = (node: ChartNode, value: Json | null): Model[] => {
  const list = (Array.isArray(value) ? value : []) as Obj[];
  const label = node.title ?? node.y;
  if (node.shape === "bars") {
    const bars = list
      .filter((row) => !node.skip?.includes(String(row[node.x])))
      .map((row) => ({ label: String(row[node.x]), value: Number(row[node.y]) }))
      .sort((a, b) => b.value - a.value);
    return bars.length ? [{ kind: "bars", label, bars }] : [];
  }
  const keys = [...new Set(list.map((row) => (typeof row.key === "string" ? row.key : "")))];
  return keys.flatMap((key): Model[] => {
    const rows = list.filter((row) => (typeof row.key === "string" ? row.key : "") === key);
    const points = rows.map((row) => {
      const x = Number(row[node.x]);
      return { at: node.xunit === "second" ? x : x / 1000, value: Number(row[node.y]) };
    });
    return points.length ? [{ kind: "line", label: key ? `${key} ${label}` : label, points }] : [];
  });
};

function ChartPart({ node }: { node: ChartNode }) {
  const value = useSource(node.from);
  const models = useMemo(() => modelsOf(node, value), [node, value]);
  return (
    <>
      {node.title && <h2>{node.title}</h2>}
      {models.map((model) => (
        <Plot key={model.label} model={model} />
      ))}
    </>
  );
}

const Savers = lazy(() => import("../shell/gallery.tsx").then(({ Screensavers }) => ({ default: Screensavers })));

const Code = lazy(() => import("../shell/code.tsx").then(({ Code }) => ({ default: Code })));

const Settings = lazy(() => import("../shell/settings/index.tsx").then(({ Settings }) => ({ default: Settings })));

function ViewerPart({ node, id }: { node: ViewerNode; id: string }) {
  const row = rows.find((one) => one.meta?.id === id)!;
  return (
    <Suspense fallback={null}>
      {node.name === "savers" && <Savers row={row} rows={rows} site={site} from={location.pathname} live />}
      {node.name === "code" && <Code site={site} />}
    </Suspense>
  );
}

const ACTIONS: Record<string, (node: DoNode) => void> = {
  open: (node) => void open(node.to, "_blank", "noopener"),
};

function DoPart({ node }: { node: DoNode }) {
  return (
    <Btn data-do={node.action} onClick={() => ACTIONS[node.action]?.(node)}>
      {node.title}
    </Btn>
  );
}

function Part({ node, actor, id }: { node: ViewNode; actor: boolean; id: string }) {
  if (node.guest === false && !actor) return null;
  if (node.kind === "value") return <ValuePart node={node} />;
  if (node.kind === "rows") return <RowsPart node={node} />;
  if (node.kind === "form") return <FormPart node={node} />;
  if (node.kind === "chart") return <ChartPart node={node} />;
  if (node.kind === "do") return <DoPart node={node} />;
  if (node.kind === "viewer") return <ViewerPart node={node} id={id} />;
  if (node.kind !== "group") return null;
  return (
    <>
      {node.title && <h2>{node.title}</h2>}
      {node.nodes.map((one, i) => (
        <Part key={i} node={one} actor={actor} id={id} />
      ))}
    </>
  );
}

function Head({ app, signed }: { app: Manifest; signed: Signed }) {
  return (
    <>
      <h1>{app.title}</h1>
      <p className="sub">{app.line}</p>
      <p className="reads">{signed.actor ? `Signed in as ${named(signed.who.id)}` : "Watching as Guest"}</p>
    </>
  );
}

function Drawn({ app, signed, nodes }: { app: Manifest; signed: Signed; nodes: ViewNode[] }) {
  const panes = nodes.filter((one) => one.kind === "pane");
  if (panes.length) {
    const row = rows.find((one) => one.meta?.id === app.id)!;
    return (
      <Suspense fallback={null}>
        <Settings row={row} site={site} panes={panes} start={new URL(location.href)} />
      </Suspense>
    );
  }
  const whole = nodes.find((one) => one.kind === "viewer");
  if (whole) return <Part node={whole} actor={signed.actor} id={app.id} />;
  return (
    <section className="world">
      <Head app={app} signed={signed} />
      <div className="arena">
        {nodes
          .filter((one) => one.guest !== false || signed.actor)
          .map((one, i) => (
            <div key={i} className="panel">
              <Part node={one} actor={signed.actor} id={app.id} />
            </div>
          ))}
      </div>
    </section>
  );
}

function Plain({ id, app, signed }: { id: string; app: Manifest; signed: Signed }) {
  const actor = signed.actor;
  const slice = useWorld(actor ? `${id}/${SLICE}${signed.who.id}` : id);
  const whole = useWorld(id);
  const moves = useMoves(id);
  const call = useCall();
  const [note, setNote] = useState<string | null>(null);
  const send = (verb: string) => (args: Obj) => {
    const out = call(verb, args);
    setNote(out.note ?? (out.envelope?.said === undefined ? null : JSON.stringify(out.envelope.said)));
    return out;
  };
  return (
    <section className="world">
      <Head app={app} signed={signed} />
      <div className="arena">
        <div className="panel">
          <Tree value={actor && slice !== null ? slice : whole} />
        </div>
        {app.verbs.length > 0 && (
          <div className="panel">
            {moves.map((move) => (
              <Form key={move.verb} move={move} send={send(move.verb)} />
            ))}
            <p className="note" role="status">
              {note}
            </p>
          </div>
        )}
      </div>
    </section>
  );
}

export function View({ id }: { id: string }) {
  const app = useApp(id);
  const signed = useWho();
  const nodes = useView(id);
  if (!signed) return null;
  if (!app) return <p className="sub">This world holds no {id}.</p>;
  return nodes ? <Drawn app={app} signed={signed} nodes={nodes} /> : <Plain id={id} app={app} signed={signed} />;
}

export const view = (id: string) =>
  island((host) => {
    const still = host.nextElementSibling;
    still?.remove();
    return { node: <View id={id} />, close: () => still && host.after(still) };
  });
