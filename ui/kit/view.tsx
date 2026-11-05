import { named, SLICE } from "@mrlyprod/os/contract/index.ts";
import type { Json, Move, Obj, Schema } from "@mrlyprod/os/kernel/index.ts";
import { canon, check } from "@mrlyprod/os/kernel/index.ts";
import type { ReactNode } from "react";
import { Fragment, useState } from "react";
import { Btn, Group, Pick, Slider, Text, Toggle } from "./controls.tsx";
import { island } from "./island.tsx";
import type { Outcome } from "./world.ts";
import { useApp, useCall, useMoves, useWho, useWorld } from "./world.ts";

type FieldProps = { name: string; schema: Schema; value: Json; onChange: (value: Json) => void };

type FormProps = { move: Move; send: (args: Obj) => Outcome };

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
  const number = (text: string) => {
    const n = Number(text);
    if (text.trim() && Number.isFinite(n)) onChange(n);
  };
  if (schema.enum) {
    const options = schema.enum.map((one): [string, string] => [canon(one), shown(one as Json)]);
    const pick = (key: string) => onChange(schema.enum!.find((one) => canon(one) === key) as Json);
    return <Pick label={label} value={canon(value)} options={options} onChange={pick} />;
  }
  if (schema.type === "integer" || schema.type === "number") {
    const { minimum: min, maximum: max } = schema;
    if (min === undefined || max === undefined)
      return <Text label={label} value={shown(value)} commit onChange={number} />;
    const step = schema.multipleOf ?? (schema.type === "integer" ? 1 : (max - min) / 100);
    return <Slider label={label} value={Number(value)} min={min} max={max} step={step} onChange={onChange} />;
  }
  if (schema.type === "boolean")
    return <Toggle label={label} value={value ? 1 : 0} onChange={(on) => onChange(on === 1)} />;
  if (schema.type === "string")
    return <Text label={label} value={String(value)} wide={(schema.maxLength ?? 0) > 40} onChange={onChange} />;
  const parse = (text: string) => {
    try {
      onChange(JSON.parse(text) as Json);
    } catch {}
  };
  return <Text label={label} value={canon(value)} wide commit onChange={parse} />;
}

export function Form({ move, send }: FormProps) {
  const props = move.args.properties ?? {};
  const keys = Object.keys(props);
  const need = new Set(move.args.required ?? []);
  const [draft, setDraft] = useState<Obj>({});
  const value = (key: string) => {
    const one = props[key]!;
    return key in draft && check(one, draft[key]) === null ? draft[key]! : first(one);
  };
  const press = () => {
    const out = send(
      Object.fromEntries(keys.filter((key) => need.has(key) || key in draft).map((key) => [key, value(key)])),
    );
    if (out.ok) setDraft(Object.fromEntries(Object.entries(draft).filter(([key]) => props[key]?.type !== "string")));
  };
  return (
    <Group name={move.verb}>
      <p className="sub">{move.line}</p>
      {keys.map((key) => (
        <Field
          key={key}
          name={key}
          schema={props[key]!}
          value={value(key)}
          onChange={(one) => setDraft({ ...draft, [key]: one })}
        />
      ))}
      <Btn primary onClick={press}>
        {tail(move.verb)}
      </Btn>
    </Group>
  );
}

export function View({ id }: { id: string }) {
  const app = useApp(id);
  const signed = useWho();
  const actor = signed?.actor === true;
  const slice = useWorld(actor ? `${id}/${SLICE}${signed.who.id}` : id);
  const whole = useWorld(id);
  const moves = useMoves(id);
  const call = useCall();
  const [note, setNote] = useState<string | null>(null);
  if (!signed) return null;
  if (!app) return <p className="sub">This world holds no {id}.</p>;
  const send = (verb: string) => (args: Obj) => {
    const out = call(verb, args);
    setNote(out.note ?? (out.envelope?.said === undefined ? null : JSON.stringify(out.envelope.said)));
    return out;
  };
  return (
    <section className="world">
      <h1>{app.title}</h1>
      <p className="sub">{app.line}</p>
      <p className="reads">{actor ? `Signed in as ${named(signed.who.id)}` : "Watching as Guest"}</p>
      <div className="arena">
        <div className="panel">
          <Tree value={actor && slice !== null ? slice : whole} />
        </div>
        <div className="panel">
          {moves.map((move) => (
            <Form key={move.verb} move={move} send={send(move.verb)} />
          ))}
          <p className="note" role="status">
            {note}
          </p>
        </div>
      </div>
    </section>
  );
}

export const view = (id: string) =>
  island((host) => {
    const still = host.nextElementSibling;
    still?.remove();
    return { node: <View id={id} />, close: () => still && host.after(still) };
  });
