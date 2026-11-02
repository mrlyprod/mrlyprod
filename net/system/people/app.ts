import { kindFor, palette } from "../../sdk/kinds.ts";
import type { Ctx, Json, Kind, Obj, Schema } from "../../sdk/system.ts";
import { app, below, named, notice, SLICE, stream, walk } from "../../sdk/system.ts";
import { HUES } from "../../sdk/theme/hues.ts";
import bot from "./bot.ts";

/* TYPES */

export type Person = { name: string; hue: string; glyph: string; kind: Kind };

export type Role = "owner" | "admin" | "member";

export type Org = {
  n: number;
  name: string;
  line: string;
  open: boolean;
  by: string;
  founded: number;
  dissolved: number | null;
  members: Record<string, Role>;
  asking: string[];
};

export type People = { orgs: Record<string, Org>; [id: string]: Person | Record<string, Org> };

export const NAME = 32;

export const ORG = 40;

export const LINE = 140;

const ID = 40;

const ROLES: Role[] = ["owner", "admin", "member"];

/* CARDS */

export const cards = (state: People): Record<string, Person> =>
  Object.fromEntries(Object.entries(state).filter(([key]) => key !== "orgs")) as Record<string, Person>;

export function glyph(bits: number): string {
  const rows = Array.from({ length: 5 }, (_, y) => {
    const half = [0, 1, 2].map((x) => ((bits >>> (y * 3 + x)) & 1 ? "#" : "."));
    return [...half, half[1], half[0]].join("");
  });
  return rows.join(" ").includes("#") ? rows.join(" ") : "..... ..... ..#.. ..... .....";
}

const card = (id: string, kind: Kind, draw: () => number, hue = HUES[below(draw, HUES.length)]!): Person => ({
  name: named(id),
  hue,
  glyph: glyph(draw()),
  kind,
});

function shuffled(draw: () => number): string[] {
  const out = [...HUES];
  for (let i = out.length - 1; i > 0; i--) {
    const j = below(draw, i + 1);
    [out[i], out[j]] = [out[j]!, out[i]!];
  }
  return out;
}

/* ORGS */

const count = (state: People) => Object.keys(state.orgs).length;

function orgOf(state: People, args: Obj, ctx: Ctx): Org {
  const one = state.orgs[String(args.org)]!;
  if (one.dissolved !== null) ctx.refuse(`${one.name} was dissolved`);
  return one;
}

const owners = (one: Org) => Object.keys(one.members).filter((id) => one.members[id] === "owner");

const put = (state: People, one: Org): People => ({ ...state, orgs: { ...state.orgs, [one.n]: one } });

const without = (members: Record<string, Role>, id: string) =>
  Object.fromEntries(Object.entries(members).filter(([one]) => one !== id));

const mine = (state: People, id: string) => ({
  orgs: Object.fromEntries(
    Object.values(state.orgs)
      .filter((one) => id in one.members)
      .map((one) => [one.n, one.members[id]!]),
  ),
  asking: Object.values(state.orgs)
    .filter((one) => one.asking.includes(id))
    .map((one) => one.n),
});

/* SCHEMAS */

const renaming: Schema = {
  type: "object",
  properties: { name: { type: "string", minLength: 1, maxLength: NAME } },
  required: ["name"],
  additionalProperties: false,
};

const someone = (state: People): Schema => {
  const ids = Object.keys(cards(state));
  return ids.length ? { enum: ids } : { type: "string", minLength: 1, maxLength: ID };
};

const naming = (state: People, more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { org: { type: "integer", minimum: 1, maximum: Math.max(1, count(state)) }, ...more },
  required: ["org", ...Object.keys(more)],
  additionalProperties: false,
});

const founding: Schema = {
  type: "object",
  properties: {
    name: { type: "string", minLength: 1, maxLength: ORG },
    line: { type: "string", maxLength: LINE },
    open: { type: "boolean" },
  },
  required: ["name", "open"],
  additionalProperties: false,
};

/* APP */

export default app<People>({
  id: "people",
  title: "People",
  line: "Who lives in this world, a name, a hue, a glyph and a kind each, and the organisations they found and join.",
  boot: ({ seed, actors, rules }) => {
    const { draw } = stream(seed);
    const hues = shuffled(draw);
    const kinds = palette(rules.kinds);
    return {
      ...Object.fromEntries(
        (actors ?? []).map((id, i) => [id, card(id, kindFor(seed, id, kinds), draw, hues[i % hues.length])]),
      ),
      orgs: {},
    };
  },
  verbs: {
    rename: {
      line: `Rename yourself: 1 to ${NAME} characters, a name nobody else holds.`,
      args: renaming,
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        const me = ctx.who.id;
        const all = cards(state);
        const held = Object.keys(all).find((id) => id !== me && all[id]!.name.toLowerCase() === name.toLowerCase());
        if (held) return ctx.refuse(`${held} is already called ${all[held]!.name}; pick another name`);
        return { ...state, [me]: { ...(all[me] ?? card(me, ctx.who.kind, ctx.draw)), name } };
      },
    },
    found: {
      line: `Found an organisation and own it: a name of 1 to ${ORG} characters nobody else holds; open lets anyone join.`,
      args: founding,
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        const held = Object.values(state.orgs).find(
          (one) => one.dissolved === null && one.name.toLowerCase() === name.toLowerCase(),
        );
        if (held) return ctx.refuse(`org ${held.n} is already called ${held.name}; pick another name`);
        const n = count(state) + 1;
        const org: Org = {
          n,
          name,
          line: String(args.line ?? "").trim(),
          open: args.open === true,
          by: ctx.who.id,
          founded: ctx.at,
          dissolved: null,
          members: { [ctx.who.id]: "owner" },
          asking: [],
        };
        return put(state, org);
      },
    },
    join: {
      line: "Join an open organisation, or ask to join a closed one.",
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = orgOf(state, args, ctx);
        if (me in one.members) return ctx.refuse(`you are already a member of ${one.name}`);
        if (one.open) return put(state, { ...one, members: { ...one.members, [me]: "member" } });
        if (one.asking.includes(me))
          return ctx.refuse(`you already asked to join ${one.name}; an owner or admin admits you`);
        return put(state, { ...one, asking: [...one.asking, me] });
      },
    },
    admit: {
      line: "As an owner or admin, admit someone who asked to join.",
      args: (state) => naming(state, { person: someone(state) }),
      run: (state, args, ctx) => {
        const one = orgOf(state, args, ctx);
        const person = String(args.person);
        const role = one.members[ctx.who.id];
        if (role !== "owner" && role !== "admin")
          return ctx.refuse(`only an owner or an admin of ${one.name} admits members`);
        if (!one.asking.includes(person)) return ctx.refuse(`${person} has not asked to join ${one.name}`);
        ctx.wish(notice(person, `You are a member of ${one.name}`));
        return put(state, {
          ...one,
          members: { ...one.members, [person]: "member" },
          asking: one.asking.filter((id) => id !== person),
        });
      },
    },
    leave: {
      line: "Leave an organisation, or take back your ask to join; the last member out dissolves it.",
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = orgOf(state, args, ctx);
        if (one.asking.includes(me)) return put(state, { ...one, asking: one.asking.filter((id) => id !== me) });
        if (!(me in one.members)) return ctx.refuse(`you are not a member of ${one.name}`);
        const members = without(one.members, me);
        const alone = owners(one).length === 1 && one.members[me] === "owner";
        if (alone && Object.keys(members).length)
          return ctx.refuse(`you are the only owner of ${one.name}; make another member owner first`);
        if (Object.keys(members).length) return put(state, { ...one, members });
        return put(state, { ...one, members, asking: [], dissolved: ctx.at });
      },
    },
    role: {
      line: "As an owner, make a member an owner, an admin or a member.",
      args: (state) => naming(state, { person: someone(state), role: { enum: ROLES } }),
      run: (state, args, ctx) => {
        const one = orgOf(state, args, ctx);
        const person = String(args.person);
        const role = args.role as Role;
        if (one.members[ctx.who.id] !== "owner") return ctx.refuse(`only an owner of ${one.name} sets roles`);
        if (!(person in one.members)) return ctx.refuse(`${person} is not a member of ${one.name}`);
        if (role !== "owner" && owners(one).length === 1 && one.members[person] === "owner")
          return ctx.refuse(`${one.name} needs an owner; make another member owner first`);
        return put(state, { ...one, members: { ...one.members, [person]: role } });
      },
    },
  },
  moves: (state) => (count(state) ? ["rename", "found", "join", "admit", "leave", "role"] : ["rename", "found"]),
  read: (state, path) => {
    const [head = "", ...rest] = path;
    if (!path.length) return cards(state) as Json;
    if (head === "orgs") return walk(state.orgs as Json, rest);
    if (head.startsWith(SLICE)) return walk(mine(state, head.slice(SLICE.length)), rest);
    return walk(state as Json, path);
  },
  bot,
});
