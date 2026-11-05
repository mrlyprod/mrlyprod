import { alike, hidden } from "@mrlyprod/apps/shared/names.ts";
import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, SLICE, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";
import type { Lookup, Thing } from "../../canon.ts";
import { absent, barred, existing, found, kinds, known, ruling, slug } from "../../canon.ts";

export type Canon = { rules: Obj; added: Record<string, Thing> };

const KNOBS = {
  open: { line: "People can bring new things into being.", schema: { type: "boolean" }, reality: true },
  name: {
    line: "Longest name of a new thing, in characters.",
    schema: { type: "integer", minimum: 8, maximum: 200 },
    reality: 80,
  },
  about: {
    line: "Longest description of a new thing, in characters.",
    schema: { type: "integer", minimum: 20, maximum: 2000 },
    reality: 280,
  },
  each: {
    line: "New things one person may bring into being.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 10,
  },
  keep: {
    line: "Percent of each kind of thing a generated world keeps.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 100,
  },
  not: { line: "Ids of things that do not exist in this world; null means none.", schema: {}, reality: null },
  only: { line: "The only kinds of thing this world holds; null means all.", schema: {}, reality: null },
} satisfies Knobs;

const ruled = (knob: Knob): Obj =>
  Object.fromEntries(
    (["not", "only"] as const).flatMap((name) => {
      const value = knob<Json>(name);
      return value === null ? [] : [[`canon.${name}`, value]];
    }),
  );

const MAKERS = ["company", "author", "artist"];

type Org = { name: string; dissolved: number | null; members: Obj };

const lookup = (state: Canon): Lookup => ({ knob: reader(state.rules, "canon"), read: () => state.added as Json });

const newest = (state: Canon) => Object.values(state.added).sort((a, b) => a.at! - b.at!);

const addedBy = (state: Canon, id: string) => newest(state).filter((one) => one.added === id);

const short = ({ id, name, kind, added }: Thing): Obj => (added ? { id, name, kind, added } : { id, name });

const tally = (state: Canon) => {
  const counts: Record<string, number> = Object.fromEntries(kinds(state.rules).map((kind) => [kind, 0]));
  for (const one of existing(lookup(state))) counts[one.kind]! += 1;
  return counts;
};

const front = (state: Canon, who: Who | null): Obj => ({
  ...tally(state),
  added: Object.keys(state.added).length,
  ...(who ? { mine: addedBy(state, who.id).map(short) } : {}),
});

const shelf = (state: Canon, id: string): Obj => ({
  mine: addedBy(state, id).map(short),
  ...Object.fromEntries(kinds(state.rules).map((kind) => [kind, existing(lookup(state), kind).map((one) => one.name)])),
});

const detail = (state: Canon, id: string) =>
  found(lookup(state), id) ?? { exists: false, note: absent(null, id, lookup(state)) };

const deep = (state: Canon, head: string, rest: string[], who: Who | null): Json | undefined => {
  if (head === "thing") return rest[0] ? walk(detail(state, rest[0]), rest.slice(1)) : undefined;
  if (head === "added") return walk(newest(state).map(short), rest);
  if (head === "mine") return who ? walk(addedBy(state, who.id).map(short), rest) : undefined;
  return kinds(state.rules).includes(head) ? walk(existing(lookup(state), head).map(short), rest) : undefined;
};

const brand = (state: Canon, base: string, kind: string, ctx: Ctx) => {
  const me = ctx.who.id;
  const maker = MAKERS.map((one) => known(`${base}-${one}`) ?? known(base)).find(
    (one) => one && MAKERS.includes(one.kind),
  );
  if (maker && maker.kind !== kind) return true;
  const orgs = Object.values((ctx.read("people/orgs") ?? {}) as Record<string, Org>);
  return (
    orgs.some((org) => org.dissolved === null && slug(org.name) === base && !Object.hasOwn(org.members, me)) ||
    Object.values(state.added).some((one) => one.added !== me && slug(one.name) === base && one.kind !== kind)
  );
};

const claim = (state: Canon, kind: string, name: string, ctx: Ctx): string => {
  const base = slug(name);
  if (!base) return ctx.refuse(`${name} has no letter or digit; name it with words`);
  if (brand(state, base, kind, ctx)) return ctx.refuse(`${name} is someone else's name; name it something else`);
  for (const id of [base, `${base}-${kind}`]) {
    if (barred(state.rules, id)) return ctx.refuse(`${name} cannot exist in this world; name something else`);
    const one = known(id) ?? (Object.hasOwn(state.added, id) ? state.added[id]! : null);
    if (!one) return id;
    if (one.kind === kind) return ctx.refuse(`${one.name} already exists here; read canon/thing/${id}`);
  }
  return ctx.refuse(`${name} is taken; name it something else`);
};

const adding = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    kind: { enum: kinds(ruled(knob)) },
    name: { type: "string", minLength: 1, maxLength: knob("name") },
    about: { type: "string", minLength: 1, maxLength: knob("about") },
  },
  required: ["kind", "name", "about"],
  additionalProperties: false,
});

const shown = sliced(shelf, undefined, true);

export default app<Canon>({
  id: "canon",
  title: "Canon",
  line: "What exists in this world: read canon for kinds, canon/<kind> for a list, canon/thing/<id> for one; add a new thing.",
  boot: ({ knob }) => ({ rules: ruling(ruled(knob)), added: {} }),
  verbs: {
    add: {
      line: (knob) =>
        knob<boolean>("open")
          ? `Bring a new thing into being, marked as yours: a kind, a name of up to ${knob("name")} characters, and what it is in up to ${knob("about")}; up to ${knob("each")} each.`
          : "Nothing new can come into being in this world; pick from what exists.",
      args: adding,
      run: (state, args, ctx) => {
        if (!ctx.knob<boolean>("open"))
          return ctx.refuse("nothing new can come into being in this world; pick from what exists: read canon");
        const me = ctx.who.id;
        const kind = String(args.kind);
        const name = String(args.name).trim();
        const about = String(args.about).trim();
        if (!name) return ctx.refuse("the name is only spaces; name the thing");
        if (!about) return ctx.refuse("the about is only spaces; say what it is");
        const most = ctx.knob("each");
        if (addedBy(state, me).length >= most)
          return ctx.refuse(`you brought ${most} things into being, the most one person may; use what exists`);
        if (hidden(name) || hidden(about.replace(/\s/g, " ")))
          return ctx.refuse("text cannot hold control, zero-width or direction characters; use plain words");
        const twin = existing(lookup(state), kind).find((one) => alike(name, one.name));
        if (twin) return ctx.refuse(`${twin.name} already exists here as ${twin.id}; read canon/thing/${twin.id}`);
        const id = claim(state, kind, name, ctx);
        ctx.say({ note: `${name} now exists as ${id}`, id, kind });
        return { ...state, added: { ...state.added, [id]: { id, name, kind, about, added: me, at: ctx.at } } };
      },
    },
  },
  read: (state, path, who, at) => {
    const [head, ...rest] = path;
    if (!head) return front(state, who);
    return head.startsWith(SLICE) ? shown(state, path, who, at) : deep(state, head, rest, who);
  },
  knobs: KNOBS,
});
