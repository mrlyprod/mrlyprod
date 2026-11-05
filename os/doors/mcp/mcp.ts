import { createInterface } from "node:readline";
import type { Envelope, Json, Obj, Who } from "../../kernel/index.ts";
import { Fault } from "../../kernel/index.ts";

export type World = {
  list: () => {
    moves: { verb: string; line: string; args: object }[];
    apps: { id: string; title: string; line: string }[];
  };
  call: (act: { verb: string; args?: Obj; who: Who; at: number }) => Envelope;
  read: (path?: string, shape?: Json, who?: Who | null) => Envelope;
  at: () => number;
};

export type Options = { name: string; version: string; who: Who; note: string; front?: () => string[] };

type Ask = { jsonrpc: "2.0"; id?: number | string | null; method: string; params?: Record<string, unknown> };

type Out = (message: object) => void;

const VERSIONS = ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

const READ = "read";

const URI = "world:///";

const SHORT = 2000;

export const tool = (verb: string) => verb.replaceAll(".", "_");

const asText = (value: unknown) => JSON.stringify(value);

function tools(world: World) {
  const { moves, apps } = world.list();
  const title = (verb: string) => apps.find((one) => verb.slice(0, verb.indexOf(".")) === one.id)?.title;
  return [
    ...moves.map((move) => ({
      name: tool(move.verb),
      title: move.verb,
      description: [title(move.verb), move.line].filter(Boolean).join(": "),
      inputSchema: move.args,
    })),
    {
      name: READ,
      title: "read",
      description:
        "Read the world at a slash path, as you see it: '' for your front page (who you are, the time, every app); '<app>' for your view of one app; '<app>/<key>' for one leaf of that view; '<app>/@<id>' for what others may see of someone, '<app>/@me' for all of yours. shape keeps only the keys it names, in every entry of a list or record of records; its key '*' applies to every value of a record.",
      inputSchema: {
        type: "object",
        properties: { path: { type: "string" }, shape: { type: ["object", "array"] } },
        additionalProperties: false,
      },
    },
  ];
}

const told = (value: unknown) => ({ content: [{ type: "text", text: asText(value) }] });

function failed(note: string) {
  return { content: [{ type: "text", text: note }], isError: true };
}

function front(world: World, { who, front: shown }: Options) {
  const at = world.at();
  return {
    me: who.id,
    kind: who.kind,
    at,
    now: new Date(at).toISOString(),
    ...Object.fromEntries((shown?.() ?? []).map((path) => [path, world.read(path, undefined, who).state])),
    apps: Object.fromEntries(world.list().apps.map((one) => [one.id, one.line])),
  };
}

const keysOf = (value: unknown) => (value && typeof value === "object" ? Object.keys(value) : []);

const ROWS = 20;

function hollow(value: unknown): boolean {
  if (Array.isArray(value)) return value.length > 0 && value.every(hollow);
  if (value && typeof value === "object") return Object.values(value).every(hollow);
  return false;
}

function looked(world: World, opts: Options, path: string, shape: Json | undefined) {
  const { who } = opts;
  if (!path) {
    const page = front(world, opts);
    const keep = Array.isArray(shape) ? shape.map(String) : keysOf(shape);
    return told(keep.length ? Object.fromEntries(Object.entries(page).filter(([key]) => keep.includes(key))) : page);
  }
  const { tick, state } = world.read(path, shape, who);
  if (state === null) {
    const up = path.split("/").slice(0, -1).join("/");
    const here = up ? keysOf(world.read(up, undefined, who).state) : world.list().apps.map((one) => one.id);
    return failed(`nothing at ${path}; ${up || "the world"} holds: ${here.join(", ") || "nothing"}`);
  }
  if (shape !== undefined && hollow(state)) {
    const raw = world.read(path, undefined, who).state;
    const rows = Array.isArray(raw) ? raw : raw && typeof raw === "object" ? Object.values(raw) : [];
    const inner = [...new Set(rows.slice(0, ROWS).flatMap(keysOf))];
    const here = Array.isArray(raw) || keysOf(state).length ? inner : keysOf(raw);
    const each = Array.isArray(raw) || keysOf(state).length ? "each entry of " : "";
    if (here.length) return failed(`shape named none of what ${each}${path} holds: ${here.join(", ")}`);
  }
  return told({ tick, state });
}

function acted(envelope: Envelope, verb: string) {
  const { tick, state, last, said } = envelope;
  if (!last.ok) return { ...told({ ok: false, tick, note: last.note }), isError: true };
  if (said !== undefined) return told({ ok: true, tick, said });
  const app = verb.slice(0, verb.indexOf("."));
  const long = asText(state ?? null).length > SHORT;
  return told({ ok: true, tick, ...(long ? { more: `read ${app} for your view` } : { state }) });
}

function called(world: World, opts: Options, name: string, args: Obj) {
  if (name === READ) return looked(world, opts, String(args.path ?? ""), args.shape);
  const verb = world.list().moves.find((one) => tool(one.verb) === name)?.verb;
  if (!verb) return failed(`no tool ${name} now; the list shifts as the world does: ask tools/list again`);
  return acted(world.call({ verb, args, who: opts.who, at: world.at() }), verb);
}

const verbsNow = (world: World) =>
  world
    .list()
    .moves.map((one) => one.verb)
    .join();

function resources(world: World) {
  return [
    { uri: URI, name: "world", title: "The whole world", mimeType: "application/json" },
    ...world
      .list()
      .apps.map((one) => ({ uri: `${URI}${one.id}`, name: one.id, title: one.title, mimeType: "application/json" })),
  ];
}

export function handle(world: World, opts: Options, ask: Ask): object | null {
  const p = ask.params ?? {};
  switch (ask.method) {
    case "initialize": {
      const asked = String(p.protocolVersion ?? "");
      return {
        protocolVersion: VERSIONS.includes(asked) ? asked : VERSIONS[1],
        capabilities: { tools: { listChanged: true }, resources: {} },
        serverInfo: { name: opts.name, version: opts.version },
        instructions: opts.note,
      };
    }
    case "ping":
      return {};
    case "tools/list":
      return { tools: tools(world) };
    case "tools/call":
      return called(world, opts, String(p.name), (p.arguments ?? {}) as Obj);
    case "resources/list":
      return { resources: resources(world) };
    case "resources/templates/list":
      return {
        resourceTemplates: [
          {
            uriTemplate: `${URI}{+path}`,
            name: "path",
            title: "A slash path in the world",
            mimeType: "application/json",
          },
        ],
      };
    case "resources/read": {
      const uri = String(p.uri ?? "");
      if (!uri.startsWith(URI)) throw new Fault("bad-act", `a resource uri starts with ${URI}`);
      const envelope = world.read(decodeURIComponent(uri.slice(URI.length)), undefined, opts.who);
      return { contents: [{ uri, mimeType: "application/json", text: asText(envelope.state) }] };
    }
    default:
      if (ask.method.startsWith("notifications/")) return null;
      throw Object.assign(new Error(`no method ${ask.method}`), { rpc: -32601 });
  }
}

export async function serve(
  world: World,
  opts: Options,
  input = process.stdin,
  write = (text: string) => process.stdout.write(text),
) {
  const say: Out = (message) => write(`${JSON.stringify(message)}\n`);
  let told = verbsNow(world);
  const tell = () => {
    const now = verbsNow(world);
    if (now === told) return;
    told = now;
    say({ jsonrpc: "2.0", method: "notifications/tools/list_changed" });
  };
  for await (const raw of createInterface({ input, crlfDelay: Number.POSITIVE_INFINITY })) {
    if (!raw.trim()) continue;
    let ask: Ask;
    try {
      ask = JSON.parse(raw);
    } catch {
      say({ jsonrpc: "2.0", id: null, error: { code: -32700, message: "parse error: send one JSON object a line" } });
      continue;
    }
    if (!ask || typeof ask !== "object" || Array.isArray(ask)) {
      say({ jsonrpc: "2.0", id: null, error: { code: -32600, message: "send one JSON object a line" } });
      continue;
    }
    try {
      const result = handle(world, opts, ask);
      if (ask.method === "tools/list") told = verbsNow(world);
      if (ask.method === "tools/call") tell();
      if (ask.id !== undefined && result !== null) say({ jsonrpc: "2.0", id: ask.id, result });
    } catch (thrown) {
      if (ask.id === undefined) continue;
      if (thrown instanceof Fault && ask.method === "tools/call") {
        say({ jsonrpc: "2.0", id: ask.id, result: failed(`${thrown.code}: ${thrown.note}`) });
        continue;
      }
      const code = (thrown as { rpc?: number }).rpc ?? (thrown instanceof Fault ? -32602 : -32603);
      say({ jsonrpc: "2.0", id: ask.id, error: { code, message: String((thrown as Error).message ?? thrown) } });
    }
  }
}
