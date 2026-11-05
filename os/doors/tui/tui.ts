import { createInterface } from "node:readline";
import type { Json, Obj, World as Reality, Schema, Who } from "../../kernel/index.ts";
import { canon, check, Fault, kernel, nearest, SPEC } from "../../kernel/index.ts";
import type { World } from "../mcp/mcp.ts";
import { handle } from "../mcp/mcp.ts";

export type Door = { name: string; title: string; who: Who; front?: () => string[] };

type Base = ReturnType<World["list"]>;

type Listing = { moves: Base["moves"]; apps: (Base["apps"][number] & { verbs?: string[] })[] };

type Place = Omit<World, "list"> & { list: () => Listing };

type Move = Listing["moves"][number];

export type Word = { from: number; to: number; raw: string; key: string | null; value: string; open: boolean };

export type Asked =
  | { kind: "own"; name: string; rest: string }
  | { kind: "act"; move: Move; args: Obj }
  | { kind: "no"; note: string };

export type Menu = Listing & { keys: (path: string) => string[] };

export type Options = { from: number; options: string[] };

export type Key = { name: string; text?: string };

type Tone = "bad" | "good" | "dim";

type Completer = (text: string) => Options;

type Io = {
  print: (text: string, tone?: Tone) => void;
  ask: (label: () => string, complete: Completer, main: boolean) => Promise<string | null>;
  status: (text: string) => void;
  close: () => void;
};

const OWN = ["read", "help", "apps", "quit"];

const WIDE = 200;

const ITEMS = 50;

const DEPTH = 6;

const LINES = 400;

const POLL = 2000;

const NUMBER = /^-?\d+(\.\d+)?([eE][+-]?\d+)?$/;

const WORD = /(?:([^\s="']+)=)?(?:"((?:\\.|[^"\\])*)("?)|'([^']*)('?)|(\S*))/y;

const ESC = "\x1b";

const WIPE = `${ESC}[2K`;

const PASTE_ON = `${ESC}[200~`;

const PASTE_OFF = `${ESC}[201~`;

const TONES: Record<Tone, string> = { bad: "31", good: "32", dim: "2" };

const CONTROLS: Record<string, string> = {
  "\x01": "home",
  "\x03": "cancel",
  "\x04": "eof",
  "\x05": "end",
  "\x08": "backspace",
  "\x09": "tab",
  "\x0a": "enter",
  "\x0b": "kill",
  "\x0d": "enter",
  "\x15": "wipe",
  "\x17": "rub",
  "\x7f": "backspace",
};

const SEQUENCES: Record<string, string> = {
  "[A": "up",
  "[B": "down",
  "[C": "right",
  "[D": "left",
  "[F": "end",
  "[H": "home",
  "[1~": "home",
  "[3~": "delete",
  "[4~": "end",
};

const graphemes = new Intl.Segmenter();

const parts = (text: string) => Array.from(graphemes.segment(text), (one) => one.segment);

const wide = (text: string) => Bun.stringWidth(text);

export const clean = (text: string) =>
  text.replace(/[\p{Cc}\p{Zl}\p{Zp}\p{Bidi_C}]/gu, (one) => (one === "\n" ? one : one === "\t" ? " " : ""));

export function fit(text: string, width: number) {
  if (wide(text) <= width) return text;
  const room = Math.max(width, 8) - 3;
  let out = "";
  let used = 0;
  for (const { segment } of graphemes.segment(text)) {
    used += wide(segment);
    if (used > room) break;
    out += segment;
  }
  return `${out}...`;
}

const branch = (value: unknown): value is object =>
  !!value && typeof value === "object" && Object.keys(value).length > 0;

function leaf(value: unknown) {
  if (typeof value === "string") return value || '""';
  if (value && typeof value === "object") return Array.isArray(value) ? "[]" : "{}";
  return String(value);
}

const flat = (text: string) => clean(text).replaceAll("\n", " ↵ ");

function rows(value: object, width: number, depth: number): string[] {
  const all = Object.entries(value);
  if (depth >= DEPTH) return [`... ${all.length} more inside`];
  const list = Array.isArray(value);
  const out = all.slice(0, ITEMS).flatMap(([key, one]) => {
    const label = list ? "- " : `${flat(key)}: `;
    if (!branch(one)) return [`${label}${fit(flat(leaf(one)), width - wide(label))}`];
    const inner = rows(one, width - 2, depth + 1);
    if (list) return inner.map((line, i) => `${i ? "  " : "- "}${line}`);
    return [`${flat(key)}:`, ...inner.map((line) => `  ${line}`)];
  });
  if (all.length > ITEMS) out.push(`... ${all.length - ITEMS} more`);
  return out;
}

export function tree(value: unknown, width = WIDE): string[] {
  if (!branch(value)) return clean(leaf(value)).split("\n");
  const out = rows(value, width, 0);
  return out.length > LINES
    ? [...out.slice(0, LINES), `... ${out.length - LINES} more lines; read a deeper path`]
    : out;
}

const unescaped = (text: string) => text.replace(/\\(.)/g, (_, one) => (one === "n" ? "\n" : one));

export function split(line: string): Word[] {
  const out: Word[] = [];
  let at = 0;
  while (at < line.length) {
    if (/\s/.test(line[at])) {
      at++;
      continue;
    }
    WORD.lastIndex = at;
    const [raw, key, double, shut, single, close, bare] = WORD.exec(line) as RegExpExecArray;
    const open = (double !== undefined && !shut) || (single !== undefined && !close);
    const value = double !== undefined ? unescaped(double) : (single ?? bare ?? "");
    out.push({ from: at, to: at + raw.length, raw, key: key ?? null, value, open });
    at += raw.length;
  }
  return out;
}

const appOf = (verb: string) => verb.split(".").at(-2) ?? "";

const nameOf = (verb: string) => verb.split(".").at(-1) ?? "";

const idsOf = ({ moves, apps }: Listing) => [
  ...new Set([...apps.map((one) => one.id), ...moves.map((one) => appOf(one.verb))]),
];

const prop = (schema: Schema, key: string): Schema => {
  const all = schema.properties ?? {};
  return Object.hasOwn(all, key) ? all[key] : {};
};

const shown = (one: unknown) => (typeof one === "string" ? one : canon(one));

const listed = (schema: Schema) => (schema.const !== undefined ? [schema.const] : schema.enum);

function parsed(raw: string): Json {
  try {
    return JSON.parse(raw);
  } catch {
    return raw;
  }
}

export function coerce(raw: string, schema: Schema): Json {
  const hit = listed(schema)?.find((one) => shown(one) === raw);
  if (hit !== undefined) return hit as Json;
  switch (schema.type) {
    case "string":
      return raw;
    case "number":
    case "integer":
      return NUMBER.test(raw) ? Number(raw) : raw;
    case "boolean":
      return raw === "true" ? true : raw === "false" ? false : raw;
    case "null":
      return raw === "null" ? null : raw;
    case "object":
      return parsed(raw);
    case "array":
      if (raw.trimStart().startsWith("[")) return parsed(raw);
      return raw ? raw.split(",").map((one) => coerce(one.trim(), schema.items ?? {})) : [];
    default:
      if (listed(schema)) return raw;
      if (raw === "true" || raw === "false") return raw === "true";
      return NUMBER.test(raw) ? Number(raw) : raw;
  }
}

const range = (low: number | undefined, high: number | undefined, unit = "") => {
  if (low !== undefined && high !== undefined) return `${low} to ${high}${unit}`;
  if (low !== undefined) return `at least ${low}${unit}`;
  return high === undefined ? null : `at most ${high}${unit}`;
};

export function limits(schema: Schema): string {
  const choices = listed(schema);
  const items = schema.items && listed(schema.items);
  const kind =
    schema.type === "array" ? "list, split by commas or JSON" : schema.type === "object" ? "JSON" : schema.type;
  return [
    choices ? `one of ${choices.map(shown).join(", ") || "nothing now"}` : (kind ?? "anything"),
    items && `of ${items.map(shown).join(", ")}`,
    range(schema.minimum, schema.maximum),
    range(schema.minLength, schema.maxLength, " characters"),
    range(schema.minItems, schema.maxItems, " items"),
    schema.pattern && `matching ${schema.pattern}`,
    schema.multipleOf !== undefined && `a multiple of ${schema.multipleOf}`,
    schema.line,
  ]
    .filter(Boolean)
    .join(", ");
}

const absent = (app: string, verb: string, { moves, apps }: Listing) => {
  const legal = moves.filter((one) => appOf(one.verb) === app).map((one) => nameOf(one.verb));
  const all = apps.find((one) => one.id === app)?.verbs?.map(nameOf) ?? [];
  if (all.includes(verb)) return `${app}.${verb} is not legal now; legal now: ${legal.join(", ") || "nothing"}`;
  const near = nearest(verb, legal);
  return `${app} has no ${verb}${near ? `; nearest: ${near}` : ""}`;
};

export function parse(line: string, list: Listing): Asked {
  if (!line.startsWith("/")) return { kind: "no", note: "commands start with /; try /help" };
  const words = split(line);
  const shut = words.find((one) => one.open);
  if (shut) return { kind: "no", note: `unclosed quote in ${shut.raw}` };
  const [head, second] = words;
  const name = head.raw.slice(1);
  if (OWN.includes(name))
    return { kind: "own", name, rest: second ? (second.key === null ? second.value : second.raw) : "" };
  const dotted = name.includes(".");
  const app = dotted ? appOf(name) : name;
  if (!idsOf(list).includes(app)) {
    const near = nearest(app, idsOf(list));
    return { kind: "no", note: `no app ${app}${near ? `; nearest: ${near}` : ""}` };
  }
  if (!dotted && (!second || second.key !== null)) return { kind: "own", name: "help", rest: app };
  const verb = dotted ? nameOf(name) : second.value;
  const move = list.moves.find((one) => appOf(one.verb) === app && nameOf(one.verb) === verb);
  if (!move) return { kind: "no", note: absent(app, verb, list) };
  const given = words.slice(dotted ? 1 : 2);
  const stray = given.find((one) => one.key === null);
  if (stray)
    return { kind: "no", note: `${stray.raw}: arguments are name=value; quote spaces, like text="hello there"` };
  const schema = move.args as Schema;
  const args = Object.fromEntries(given.map((one) => [one.key, coerce(one.value, prop(schema, one.key as string))]));
  return { kind: "act", move, args };
}

export function values(text: string, schema: Schema): string[] {
  if (schema.type === "array") {
    const cut = text.lastIndexOf(",") + 1;
    return values(text.slice(cut), schema.items ?? {}).map((one) => `${text.slice(0, cut)}${one}`);
  }
  const choices = listed(schema) ?? (schema.type === "boolean" ? [true, false] : []);
  return choices.map(shown).filter((one) => one.startsWith(text));
}

const quoted = (value: string) => (/^$|[\s"']/.test(value) ? JSON.stringify(value) : value);

const closed = (options: string[]) =>
  options.length === 1 && !/[=/,]$/.test(options[0]) ? [`${options[0]} `] : options;

export function complete(text: string, menu: Menu): Options {
  if (!text.startsWith("/")) return { from: text.length, options: [] };
  const words = split(text);
  const last = words.at(-1) as Word;
  const typing = last.to === text.length;
  const done = typing ? words.slice(0, -1) : words;
  const from = typing ? last.from : text.length;
  const word = typing ? last.raw : "";
  const pick = (options: string[]) => ({ from, options: closed(options.filter((one) => one.startsWith(word))) });
  const none = { from, options: [] };
  if (!done.length) {
    if (word.includes(".")) return pick(menu.moves.map((one) => `/${one.verb}`));
    return pick([...new Set([...OWN, ...idsOf(menu)])].map((one) => `/${one}`));
  }
  const head = done[0].raw.slice(1);
  if (OWN.includes(head)) {
    if (done.length > 1) return none;
    if (head === "help") return pick(idsOf(menu));
    if (head !== "read") return none;
    const cut = word.lastIndexOf("/") + 1;
    return {
      from,
      options: menu
        .keys(word.slice(0, Math.max(cut - 1, 0)))
        .map((one) => `${word.slice(0, cut)}${one}/`)
        .filter((one) => one.startsWith(word)),
    };
  }
  const dotted = head.includes(".");
  if (!dotted && done.length === 1)
    return pick(menu.moves.filter((one) => appOf(one.verb) === head).map((one) => nameOf(one.verb)));
  const move = dotted
    ? menu.moves.find((one) => one.verb === head)
    : menu.moves.find((one) => appOf(one.verb) === head && nameOf(one.verb) === done[1].raw);
  if (!move) return none;
  const schema = move.args as Schema;
  if (typing && last.key !== null) {
    const options = values(last.value, prop(schema, last.key)).map((one) => `${last.key}=${quoted(one)}`);
    return { from, options: closed(options) };
  }
  const given = new Set(done.map((one) => one.key));
  const required = schema.required ?? [];
  const names = Object.keys(schema.properties ?? {}).sort((a, b) => +required.includes(b) - +required.includes(a));
  return pick(names.filter((one) => !given.has(one)).map((one) => `${one}=`));
}

const common = (list: string[]) =>
  list.reduce((a, b) => {
    let i = 0;
    while (i < a.length && a[i] === b[i]) i++;
    return a.slice(0, i);
  });

export function keysOf(chunk: string): { keys: Key[]; rest: string } {
  const keys: Key[] = [];
  let at = 0;
  while (at < chunk.length) {
    const rest = chunk.slice(at);
    if (rest.startsWith(PASTE_ON)) {
      const end = rest.indexOf(PASTE_OFF);
      if (end < 0) return { keys, rest };
      keys.push({ name: "text", text: rest.slice(PASTE_ON.length, end).replace(/\r\n?|\n/g, " ") });
      at += end + PASTE_OFF.length;
    } else if (rest[0] === ESC) {
      const body = rest.slice(1);
      if (!body) keys.push({ name: "escape" });
      else if (/^(?:\[[0-9;?]*[ -/]*|O)$/.test(body)) return { keys, rest };
      const seq = /^(?:\[[0-9;?]*[ -/]*[@-~]|O[A-Z]|.)/su.exec(body)?.[0] ?? "";
      const name = SEQUENCES[seq.replace(/^O/, "[")];
      if (name) keys.push({ name });
      at += 1 + seq.length;
    } else if (CONTROLS[rest[0]]) {
      keys.push({ name: CONTROLS[rest[0]] });
      at++;
    } else {
      const run = /^[^\p{Cc}]+/u.exec(rest)?.[0];
      if (run) keys.push({ name: "text", text: run });
      at += run?.length ?? 1;
    }
  }
  return { keys, rest: "" };
}

export function viewport(line: string, cursor: number, room: number): { shown: string; at: number } {
  const head = parts(line.slice(0, cursor));
  let used = head.reduce((sum, one) => sum + wide(one), 0);
  let start = 0;
  if (used > room - 1) {
    used += 1;
    while (used > room - 1 && start < head.length) used -= wide(head[start++]);
  }
  let tail = "";
  for (const one of parts(line.slice(cursor))) {
    if (used + wide(tail) + wide(one) > room - 2) {
      tail += "…";
      break;
    }
    tail += one;
  }
  return { shown: `${start ? "…" : ""}${head.slice(start).join("")}${tail}`, at: used };
}

const previous = (text: string, at: number) => parts(text.slice(0, at)).slice(0, -1).join("").length;

const following = (text: string, at: number) => at + (parts(text.slice(at))[0]?.length ?? 0);

const wordLeft = (text: string, at: number) => text.slice(0, at).trimEnd().lastIndexOf(" ") + 1;

function terminal(stdin: NodeJS.ReadStream, stdout: NodeJS.WriteStream): Io {
  type Waiter = {
    label: () => string;
    complete: Completer;
    main: boolean;
    done: (line: string | null) => void;
  };
  const color = !process.env.NO_COLOR;
  const paint = (text: string, code?: string) => (code && color ? `${ESC}[${code}m${text}${ESC}[0m` : text);
  const history: string[] = [];
  const queue: Key[] = [];
  let waiter: Waiter | null = null;
  let line = "";
  let cursor = 0;
  let carry = "";
  let back = 0;
  let draft = "";
  let ended = false;
  let closed = false;
  const write = (text: string) => stdout.write(text);
  const draw = () => {
    if (!waiter) return;
    const cols = stdout.columns || 80;
    const label = fit(clean(waiter.label()), Math.max(8, cols - 12));
    const inset = wide(label);
    const view = viewport(line, cursor, Math.max(4, cols - inset));
    write(`\r${WIPE}${paint(label, "1")}${view.shown}${ESC}[${inset + view.at + 1}G`);
  };
  const print = (text: string, tone?: Tone) => {
    const body = paint(clean(text), tone && TONES[tone]).replaceAll("\n", "\r\n");
    write(`\r${WIPE}${body}\r\n`);
    draw();
  };
  const set = (text: string) => {
    line = text;
    cursor = text.length;
  };
  const insert = (text: string) => {
    const safe = clean(text).replaceAll("\n", " ");
    line = `${line.slice(0, cursor)}${safe}${line.slice(cursor)}`;
    cursor += safe.length;
  };
  const cut = (from: number, to: number) => {
    line = `${line.slice(0, from)}${line.slice(to)}`;
    cursor = from;
  };
  const finish = (value: string | null) => {
    const done = waiter?.done;
    waiter = null;
    done?.(value);
  };
  const tab = (now: Waiter) => {
    const before = line.slice(0, cursor);
    const { from, options } = now.complete(before);
    if (!options.length) return;
    const shared = common(options);
    if (shared.length > before.length - from) {
      cut(from, cursor);
      insert(shared);
    } else if (options.length > 1) print(options.join("  "), "dim");
  };
  const leave = () => {
    write(`\r${WIPE}`);
    finish(null);
  };
  const enter = (now: Waiter) => {
    const label = now.label();
    waiter = null;
    print(`${label}${line}`);
    if (now.main && line.trim() && history.at(-1) !== line) history.push(line);
    back = 0;
    now.done(line);
  };
  const recall = (step: number) => {
    const next = back + step;
    if (next < 0 || next > history.length) return;
    if (!back) draft = line;
    back = next;
    set(back ? (history.at(-back) ?? "") : draft);
  };
  const moved: Record<string, () => number> = {
    left: () => previous(line, cursor),
    right: () => following(line, cursor),
    home: () => 0,
    end: () => line.length,
  };
  const cuts: Record<string, () => [number, number]> = {
    backspace: () => [previous(line, cursor), cursor],
    delete: () => [cursor, following(line, cursor)],
    kill: () => [cursor, line.length],
    wipe: () => [0, cursor],
    rub: () => [wordLeft(line, cursor), cursor],
  };
  const press = ({ name, text }: Key, now: Waiter) => {
    if (moved[name]) cursor = moved[name]();
    else if (cuts[name]) cut(...cuts[name]());
    else if (name === "text") insert(text ?? "");
    else if (name === "tab") tab(now);
    else if (name === "enter") enter(now);
    else if (name === "eof") line ? cut(cursor, following(line, cursor)) : leave();
    else if (name === "up" || name === "down") now.main && recall(name === "up" ? 1 : -1);
    else if (line && (now.main || name === "escape")) set("");
    else if (!now.main || name === "cancel") leave();
  };
  const drain = () => {
    while (waiter && queue.length) press(queue.shift() as Key, waiter);
    draw();
  };
  const take = (chunk: Buffer | string) => {
    const got = keysOf(`${carry}${chunk}`);
    carry = got.rest;
    queue.push(...got.keys);
    drain();
  };
  const hangup = () => {
    ended = true;
    finish(null);
  };
  const redraw = () => draw();
  const close = () => {
    if (closed) return;
    closed = true;
    write(`\r${WIPE}${ESC}[?2004l`);
    stdin.setRawMode(false);
    stdin.removeListener("data", take);
    stdin.removeListener("end", hangup);
    stdout.removeListener("resize", redraw);
    stdin.pause();
  };
  process.once("exit", close);
  for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"] as const) process.once(signal, () => process.exit(1));
  write(`${"\n".repeat(Math.max((stdout.rows || 1) - 1, 0))}${ESC}[?2004h`);
  stdin.setRawMode(true);
  stdin.setEncoding("utf8");
  stdin.on("data", take);
  stdin.on("end", hangup);
  stdout.on("resize", redraw);
  stdin.resume();
  return {
    print,
    status: redraw,
    close,
    ask: (label, complete, main) =>
      new Promise((done) => {
        if (ended) return done(null);
        waiter = { label, complete, main, done };
        set("");
        drain();
      }),
  };
}

function plain(stdin: NodeJS.ReadStream, stdout: NodeJS.WriteStream): Io {
  const reader = createInterface({ input: stdin, crlfDelay: Number.POSITIVE_INFINITY });
  const lines = reader[Symbol.asyncIterator]();
  const print = (text: string) => stdout.write(`${clean(text)}\n`);
  let held: string | null = null;
  stdout.on("error", () => process.exit(0));
  return {
    print,
    status: print,
    close: () => reader.close(),
    ask: async (_, __, main) => {
      const line: string | null = held ?? (await lines.next()).value ?? null;
      held = null;
      if (line === null) return null;
      if (!main && line.startsWith("/")) {
        held = line;
        return null;
      }
      print(`> ${line}`);
      return line;
    },
  };
}

export function nowhere(): Place {
  const k = kernel([]);
  const world: Reality = {
    spec: SPEC,
    name: "nowhere",
    seed: 0,
    tick: 0,
    at: 0,
    rand: 0,
    actors: null,
    rules: {},
    apps: {},
    last: { ok: true, note: null },
  };
  return {
    list: () => k.list(world),
    call: (act) => k.call(world, act).envelope,
    read: (path, shape, who) => k.read(world, path, shape, who),
    at: () => Date.now(),
  };
}

function page(world: Place, { who, name, front }: Door, shape?: Json): Obj {
  const read = {
    jsonrpc: "2.0" as const,
    method: "tools/call",
    params: { name: "read", arguments: shape ? { shape } : {} },
  };
  const told = handle(world, { name, version: "", note: "", who, front }, read) as { content: { text: string }[] };
  return JSON.parse(told.content[0].text);
}

const failure = (thrown: unknown) =>
  thrown instanceof Fault ? `${thrown.code}: ${thrown.note}` : `crash: ${(thrown as Error)?.message ?? thrown}`;

const slash = (verb: string) => (OWN.includes(appOf(verb)) ? `/${verb}` : `/${appOf(verb)} ${nameOf(verb)}`);

function helpOf(id: string, list: Listing): string[] | null {
  const one = list.apps.find((app) => app.id === id);
  const moves = list.moves.filter((move) => appOf(move.verb) === id);
  if (!one && !moves.length) return null;
  const legal = moves.map((move) => nameOf(move.verb));
  const later = (one?.verbs ?? []).map(nameOf).filter((name) => !legal.includes(name));
  const usage = (move: Move) => {
    const schema = move.args as Schema;
    const required = schema.required ?? [];
    return [
      `${slash(move.verb)}: ${move.line}`,
      ...Object.keys(schema.properties ?? {}).map(
        (key) => `  ${key}${required.includes(key) ? " (required)" : ""}: ${limits(prop(schema, key))}`,
      ),
    ];
  };
  return [
    ...(one ? [`${one.title}: ${one.line}`] : []),
    ...(moves.length ? moves.flatMap(usage) : ["no verb is legal now"]),
    ...(later.length ? [`not legal now: ${later.join(", ")}`] : []),
    ...(one ? [`/read ${id} shows it as you may see it`] : []),
  ];
}

function helpAll(door: Door, list: Listing): string[] {
  return [
    `${door.title} (${door.name}), as ${door.who.id}:`,
    "/<app> <verb> name=value ...   act; a missing required argument is asked for",
    "/<app>.<verb> name=value ...   the same, and it reaches an app named like a command below",
    "/read [path]                   look at the world; no path is your front page",
    "/help [app]                    this, or one app's verbs and their arguments",
    "/apps                          every app",
    "/quit                          leave",
    'Tab completes. Quote spaces: text="hello there". Lists take commas or JSON. Objects take JSON.',
    list.apps.length
      ? `${list.apps.length} apps: /apps lists them`
      : "No app is installed, so there is nothing to act on.",
  ];
}

export async function tui(
  world: Place,
  door: Door,
  stdin: NodeJS.ReadStream = process.stdin,
  stdout: NodeJS.WriteStream = process.stdout,
) {
  const tty = !!stdin.isTTY && !!stdout.isTTY;
  const io = tty ? terminal(stdin, stdout) : plain(stdin, stdout);
  const width = () => (tty ? stdout.columns || 80 : WIDE);
  const say = (lines: string[], tone?: Tone) => io.print(lines.join("\n"), tone);
  let unread = 0;
  const refresh = () => {
    try {
      const now = Number(page(world, door, ["unread"]).unread) || 0;
      if (now === unread) return;
      unread = now;
      io.status(`unread: ${unread}`);
    } catch {}
  };
  const label = () => `${door.who.id}${unread ? ` (${unread} unread)` : ""}> `;
  const guarded = (run: () => Options, from: number) => {
    try {
      return run();
    } catch {
      return { from, options: [] };
    }
  };
  const keys = (path: string) => {
    if (!path) return world.list().apps.map((one) => one.id);
    const state = world.read(path, undefined, door.who).state;
    return state && typeof state === "object" ? Object.keys(state) : [];
  };
  const completer: Completer = (text) => guarded(() => complete(text, { ...world.list(), keys }), text.length);
  const look = (path: string) => {
    if (!path) return say(tree(page(world, door), width()));
    const { state } = world.read(path, undefined, door.who);
    if (state !== null) return say(tree(state, width()));
    const up = path.split("/").filter(Boolean).slice(0, -1).join("/");
    const here = keys(up);
    io.print(`nothing at ${path}; ${up || "the world"} holds: ${here.join(", ") || "nothing"}`, "bad");
  };
  const fill = async (schema: Schema, args: Obj) => {
    for (const key of schema.required ?? []) {
      if (Object.hasOwn(args, key)) continue;
      const one = prop(schema, key);
      io.print(`${key}: ${limits(one)}`, "dim");
      for (;;) {
        const raw = await io.ask(
          () => `${key}> `,
          (text) => ({ from: 0, options: values(text, one) }),
          false,
        );
        if (raw === null) return false;
        const value = coerce(raw, one);
        const note = check(one, value, key);
        if (!note) {
          args[key] = value;
          break;
        }
        io.print(note, "bad");
      }
    }
    return true;
  };
  const act = async (move: Move, args: Obj) => {
    const schema = move.args as Schema;
    const stuck = [...(schema.required ?? []), ...Object.keys(args)].find(
      (key) => prop(schema, key).enum?.length === 0,
    );
    if (stuck) return io.print(`${move.verb}: nothing to pick for ${stuck} now`, "bad");
    const note = check({ ...schema, required: [] }, args);
    if (note) return io.print(`bad-act: ${move.verb}: ${note}`, "bad");
    if (!(await fill(schema, args))) return io.print("cancelled", "dim");
    const { last, said } = world.call({ verb: move.verb, args, who: door.who, at: world.at() });
    if (said !== undefined) say(tree(said, width()));
    else if (last.ok) io.print("ok", "good");
    else io.print(`refused: ${last.note}`, "bad");
  };
  const run = async (line: string) => {
    const text = line.trim();
    if (!text) return true;
    const asked = parse(text, world.list());
    if (asked.kind === "no") io.print(asked.note, "bad");
    if (asked.kind === "act") await act(asked.move, asked.args);
    if (asked.kind !== "own") return true;
    if (asked.name === "quit") return false;
    if (asked.name === "read") look(asked.rest);
    if (asked.name === "apps") {
      const { apps } = world.list();
      say(apps.length ? apps.map((one) => fit(`${one.id}: ${one.line}`, width())) : ["no apps"]);
    }
    if (asked.name === "help") {
      const list = world.list();
      if (!asked.rest) say(helpAll(door, list));
      else say(helpOf(asked.rest, list) ?? [`no app ${asked.rest}`]);
    }
    return true;
  };
  const attempt = async (work: () => unknown) => {
    try {
      await work();
    } catch (thrown) {
      io.print(failure(thrown), "bad");
    }
  };
  await attempt(() => {
    io.print(`${door.title}: /help lists commands, Tab completes, /quit leaves.`, "dim");
    look("");
    unread = Number(page(world, door, ["unread"]).unread) || 0;
  });
  const timer = tty ? setInterval(refresh, POLL) : null;
  for (;;) {
    const line = await io.ask(label, completer, true);
    if (line === null) break;
    let going = true;
    await attempt(async () => {
      going = await run(line);
    });
    if (!going) break;
    refresh();
  }
  if (timer) clearInterval(timer);
  io.close();
}
