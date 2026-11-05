import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, ME, SLICE, sliced } from "@mrlyprod/os/contract/index.ts";

export type Hit = { path: string; text: string };

export type Found = { words: string[]; hits: Record<string, Hit[]>; at: number };

export type Search = { people: string[]; last: Record<string, Found> };

export const WORD = 24;

const ID = "search";

const HINT = "/more";

const SAID = 10;

const DEEP: Record<string, string> = { wiki: "articles" };

const DEEP_MOST = 300;

const KNOBS = {
  query: {
    line: "Characters one search may hold.",
    schema: { type: "integer", minimum: 10, maximum: 1000 },
    reality: 100,
  },
  hits: { line: "Hits a search keeps per app.", schema: { type: "integer", minimum: 1, maximum: 100 }, reality: 10 },
  snippet: {
    line: "Characters a hit quotes.",
    schema: { type: "integer", minimum: 20, maximum: 2000 },
    reality: 120,
  },
} satisfies Knobs;

export const words = (text: string): string[] =>
  [...new Set(text.toLowerCase().match(/[\p{L}\p{N}][\p{L}\p{N}-]*[\p{L}\p{N}]/gu) ?? [])]
    .map((one) => one.slice(0, WORD))
    .slice(0, 8);

type Leaf = { path: string; text: string };

export function leaves(state: Json, path: string, out: Leaf[] = []): Leaf[] {
  if (typeof state === "string") {
    if (state.trim()) out.push({ path, text: state });
    return out;
  }
  if (state === null || typeof state !== "object") return out;
  if (Array.isArray(state)) {
    for (const [i, one] of state.entries()) leaves(one, `${path}/${i}`, out);
    return out;
  }
  for (const key of Object.keys(state).sort()) leaves(state[key]!, `${path}/${key}`, out);
  return out;
}

const snippet = (text: string, size: number) => (text.length > size ? `${text.slice(0, size - 3)}...` : text);

function visible(ctx: Ctx, id: string, people: readonly string[]): Leaf[] {
  const front = leaves(ctx.see(id), id);
  const have = new Set(front.map((leaf) => leaf.path));
  const own = `${id}/${SLICE}${ME}`;
  const mine = leaves(ctx.see(own), own).filter((leaf) => !have.has(leaf.path.replace(own, id)));
  const others = people
    .filter((person) => person !== ctx.who.id)
    .flatMap((person) => leaves(ctx.see(`${id}/${SLICE}${person}`), `${id}/${SLICE}${person}`));
  const deep = Object.hasOwn(DEEP, id) ? deeper(ctx, `${id}/${DEEP[id]}`) : [];
  const seen = new Set<string>();
  return [...front, ...mine, ...deep, ...others].filter((leaf) => {
    const key = `${leaf.path.replace(/\/@[^/]+/g, "")}\n${leaf.text}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function deeper(ctx: Ctx, path: string): Leaf[] {
  const table = ctx.see(path);
  if (!table || typeof table !== "object" || Array.isArray(table)) return [];
  return Object.keys(table)
    .slice(0, DEEP_MOST)
    .flatMap((key) => leaves(ctx.see(`${path}/${key}`), `${path}/${key}`));
}

function walkable(ctx: Ctx, path: string): string {
  let at = path;
  while (at.includes("/") && ctx.see(at) === null) at = at.slice(0, at.lastIndexOf("/"));
  return at;
}

function find(ctx: Ctx, people: readonly string[], query: string[]): Record<string, Hit[]> {
  const hits: Record<string, Hit[]> = {};
  const size = ctx.knob("snippet");
  for (const id of Object.keys((ctx.read("") ?? {}) as Obj).sort()) {
    if (id === ID) continue;
    const found = visible(ctx, id, people)
      .filter((leaf) => {
        if (leaf.path.endsWith(HINT)) return false;
        const haystack = `${leaf.path} ${leaf.text}`.toLowerCase();
        return query.every((word) => haystack.includes(word));
      })
      .slice(0, ctx.knob("hits"))
      .map((leaf) => ({ path: walkable(ctx, leaf.path), text: snippet(leaf.text, size) }));
    if (found.length) hits[id] = found;
  }
  return hits;
}

const finding = (knob: Knob): Schema => ({
  type: "object",
  properties: { words: { type: "string", minLength: 1, maxLength: knob("query") } },
  required: ["words"],
  additionalProperties: false,
});

const NONE: Found = { words: [], hits: {}, at: 0 };

const mine = (state: Search, id: string) => state.last[id] ?? NONE;

const whole = (state: Search, who: Who | null) => (who ? mine(state, who.id) : NONE);

export default app<Search>({
  id: ID,
  title: "Search",
  line: "Find words and people across everything you may see; the index is rebuilt from your own view of the world on each search.",
  boot: ({ actors }) => ({ people: actors ?? [], last: {} }),
  verbs: {
    find: {
      line: (knob) =>
        `Search what you may see for words, all of them: yours, the public, what others show. Up to ${knob("hits")} hits per app, each with a path to read; kept as your last search.`,
      args: finding,
      run: (state, args, ctx) => {
        const query = words(String(args.words));
        if (!query.length) return ctx.refuse("say a word of two letters or digits at least");
        const hits = find(ctx, state.people, query);
        const flat = Object.values(hits).flat();
        ctx.say({
          note: `${flat.length} hits in ${Object.keys(hits).length} apps${flat.length > SAID ? `; the first ${SAID}, and read search for all` : ""}`,
          hits: flat.slice(0, SAID),
        });
        return { ...state, last: { ...state.last, [ctx.who.id]: { words: query, hits, at: ctx.at } } };
      },
    },
  },
  read: sliced(mine, whole),
  knobs: KNOBS,
});
