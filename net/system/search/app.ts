import type { Json, Obj, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Hit = { path: string; text: string };

export type Found = { words: string[]; hits: Record<string, Hit[]>; at: number };

export type Search = { last: Record<string, Found> };

export const QUERY = 100;

export const WORD = 24;

export const HITS = 10;

export const SNIPPET = 120;

/* WORDS */

export const words = (text: string): string[] =>
  [...new Set(text.toLowerCase().match(/[\p{L}\p{N}]{2,}/gu) ?? [])].map((one) => one.slice(0, WORD)).slice(0, 8);

/* INDEX */

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

const snippet = (text: string) => (text.length > SNIPPET ? `${text.slice(0, SNIPPET - 3)}...` : text);

export function find(world: Obj, query: string[], skip: readonly string[] = []): Record<string, Hit[]> {
  const hits: Record<string, Hit[]> = {};
  for (const id of Object.keys(world).sort()) {
    if (skip.includes(id)) continue;
    const found = leaves(world[id]!, id)
      .filter((leaf) => {
        const haystack = `${leaf.path} ${leaf.text}`.toLowerCase();
        return query.every((word) => haystack.includes(word));
      })
      .slice(0, HITS)
      .map((leaf) => ({ path: leaf.path, text: snippet(leaf.text) }));
    if (found.length) hits[id] = found;
  }
  return hits;
}

/* SCHEMAS */

const finding: Schema = {
  type: "object",
  properties: { words: { type: "string", minLength: 1, maxLength: QUERY } },
  required: ["words"],
  additionalProperties: false,
};

/* APP */

export default app<Search>({
  id: "search",
  title: "Search",
  line: "Find words across every app's reads; the index is rebuilt from the world on each search.",
  boot: () => ({ last: {} }),
  verbs: {
    find: {
      line: `Search the world for words, all of them: up to ${HITS} hits per app, kept as your last search.`,
      args: finding,
      run: (state, args, ctx) => {
        const query = words(String(args.words));
        if (!query.length) return ctx.refuse("say a word of two letters or digits at least");
        const world = (ctx.read("") ?? {}) as Obj;
        const hits = find(world, query, ["search"]);
        return { last: { ...state.last, [ctx.who.id]: { words: query, hits, at: ctx.at } } };
      },
    },
  },
  read: sliced((state: Search, me: string) => state.last[me] ?? { words: [], hits: {}, at: 0 }),
  bot,
});
