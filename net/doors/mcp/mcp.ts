import { createInterface } from "node:readline";
import type { Envelope, Json, Obj, Who } from "../../kernel/index.ts";
import { Fault } from "../../kernel/index.ts";

/* TYPES */

export type World = {
  list: () => { moves: { verb: string; line: string; args: object }[]; apps: { id: string; title: string }[] };
  call: (act: { verb: string; args?: Obj; who: Who; at: number }) => Envelope;
  read: (path?: string, shape?: Json, who?: Who | null) => Envelope;
  at: () => number;
};

export type Options = { name: string; version: string; who: Who; note: string };

type Ask = { jsonrpc: "2.0"; id?: number | string | null; method: string; params?: Record<string, unknown> };

type Out = (message: object) => void;

const VERSIONS = ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

const READ = "read";

const URI = "mrlynet://world/";

/* NAMES */

export const tool = (verb: string) => verb.replaceAll(".", "_");

const asText = (value: unknown) => JSON.stringify(value, null, 2);

/* TOOLS */

function tools(world: World) {
  const { moves, apps } = world.list();
  const title = (verb: string) => apps.find((one) => verb.split(".").includes(one.id))?.title;
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
        "Read the world at a slash path: '' for every app, 'snake' for one app, 'snake/seed' for one leaf. shape keeps only the keys it names.",
      inputSchema: {
        type: "object",
        properties: { path: { type: "string" }, shape: { type: ["object", "array"] } },
        additionalProperties: false,
      },
    },
  ];
}

function answer(envelope: Envelope) {
  return {
    content: [{ type: "text", text: asText(envelope) }],
    structuredContent: envelope,
    isError: !envelope.last.ok,
  };
}

function failed(note: string) {
  return { content: [{ type: "text", text: note }], isError: true };
}

function called(world: World, opts: Options, name: string, args: Obj, say: Out) {
  if (name === READ) return answer(world.read(String(args.path ?? ""), args.shape, opts.who));
  const before = world.list().moves.map((one) => one.verb);
  const verb = before.find((one) => tool(one) === name);
  if (!verb) return failed(`no tool ${name} now; the list shifts as the world does: ask tools/list again`);
  const envelope = world.call({ verb, args, who: opts.who, at: world.at() });
  const after = envelope.moves;
  if (after.join() !== before.join()) say({ jsonrpc: "2.0", method: "notifications/tools/list_changed" });
  return answer(envelope);
}

function resources(world: World) {
  return [
    { uri: URI, name: "world", title: "The whole world", mimeType: "application/json" },
    ...world
      .list()
      .apps.map((one) => ({ uri: `${URI}${one.id}`, name: one.id, title: one.title, mimeType: "application/json" })),
  ];
}

/* SERVE */

export function handle(world: World, opts: Options, ask: Ask, say: Out): object | null {
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
      return called(world, opts, String(p.name), (p.arguments ?? {}) as Obj, say);
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
  for await (const raw of createInterface({ input, crlfDelay: Number.POSITIVE_INFINITY })) {
    if (!raw.trim()) continue;
    let ask: Ask;
    try {
      ask = JSON.parse(raw);
    } catch {
      say({ jsonrpc: "2.0", id: null, error: { code: -32700, message: "parse error: send one JSON object a line" } });
      continue;
    }
    try {
      const result = handle(world, opts, ask, say);
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
