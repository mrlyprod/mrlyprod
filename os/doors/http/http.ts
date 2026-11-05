import type { Code, Config, Envelope, Json, Manifest, Move, Obj, Who } from "../../kernel/index.ts";
import { Fault } from "../../kernel/index.ts";

export type World = {
  list: () => { moves: Move[]; apps: Manifest[] };
  call: (act: { verb: string; args?: Obj; who: Who; at: number }) => Envelope;
  read: (path?: string, shape?: Json, who?: Who | null) => Envelope;
  at: () => number;
};

export type Worlds = {
  boot: (config: Config) => Promise<{ id: string; world: World }>;
  open: (id: string) => Promise<World | null>;
};

const STATUS: Record<Code, number> = {
  "bad-act": 400,
  "bad-config": 400,
  "unknown-verb": 404,
  "unknown-world": 404,
  refused: 403,
  "spec-mismatch": 409,
  "too-deep": 508,
};

const ROOT = "/api/1/worlds";

const json = (body: unknown, status = 200) =>
  new Response(`${JSON.stringify(body)}\n`, { status, headers: { "content-type": "application/json" } });

const fault = (code: Code, note: string) => json({ error: { code, note } }, STATUS[code]);

async function body(req: Request): Promise<Obj> {
  try {
    const one = await req.json();
    if (one && typeof one === "object" && !Array.isArray(one)) return one as Obj;
  } catch {}
  throw new Fault("bad-act", "the body must be one JSON object");
}

function shapeOf(url: URL): Json | undefined {
  const raw = url.searchParams.get("shape");
  if (raw === null) return undefined;
  try {
    return JSON.parse(raw);
  } catch {
    throw new Fault("bad-act", "shape must be JSON");
  }
}

function decoded(part: string) {
  try {
    return decodeURIComponent(part);
  } catch {
    throw new Fault("bad-act", `the path has a bad escape: ${part}`);
  }
}

export function handler(worlds: Worlds, who: Who) {
  return async (req: Request): Promise<Response> => {
    const url = new URL(req.url);
    try {
      const parts = url.pathname.slice(ROOT.length).split("/").filter(Boolean).map(decoded);
      if (!url.pathname.startsWith(ROOT)) return fault("unknown-verb", `the four calls live under ${ROOT}`);
      if (!parts.length) {
        if (req.method !== "POST") return fault("bad-act", `POST ${ROOT} with a config boots a world`);
        const { id, world } = await worlds.boot((await body(req)) as unknown as Config);
        return json({ id, envelope: world.read(world.list().apps[0]?.id ?? "") });
      }
      const [id, call, ...rest] = parts;
      const world = await worlds.open(id);
      if (!world) return fault("unknown-world", `no world ${id}`);
      if (call === "list" && req.method === "GET") return json(world.list());
      if (call === "read" && req.method === "GET") return json(world.read(rest.join("/"), shapeOf(url), who));
      if (call === "call" && req.method === "POST") {
        const act = await body(req);
        if (typeof act.verb !== "string") throw new Fault("bad-act", "an act is { verb, args }");
        return json(world.call({ verb: act.verb, args: (act.args ?? {}) as Obj, who, at: world.at() }));
      }
      return fault(
        "unknown-verb",
        `${req.method} ${ROOT}/${id}/${call ?? ""} is none of GET list, POST call, GET read`,
      );
    } catch (thrown) {
      if (thrown instanceof Fault) return fault(thrown.code, thrown.note);
      return json({ error: { code: "crash", note: String((thrown as Error)?.message ?? thrown) } }, 500);
    }
  };
}
