import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, named, sliced, stream, walk } from "@mrlyprod/os/contract/index.ts";
import { HUES } from "../shared/hues.ts";
import { alike, clean } from "../shared/names.ts";
import { blockedBy, kindFor, known, notice, palette, reach } from "../shared/people.ts";

export type Person = { name: string; hue: string; glyph: string; kind: string; bio: string; status: string };

export type Role = "owner" | "admin" | "member";

export type Org = {
  n: number;
  name: string;
  line: string;
  kind: string;
  open: boolean;
  by: string;
  founded: number;
  dissolved: number | null;
  members: Record<string, Role>;
  asking: string[];
  invited: string[];
};

export type Links = Record<string, string[]>;

export type People = {
  orgs: Record<string, Org>;
  friends: Links;
  asks: Links;
  blocks: Links;
  mutes: Links;
  [id: string]: Person | Record<string, Org> | Links;
};

const ORG_KINDS = ["company", "club", "school", "charity", "union"];

const KNOBS = {
  friends: {
    line: "Friends one person may keep.",
    schema: { type: "integer", minimum: 1, maximum: 5000 },
    reality: 1000,
  },
  founds: {
    line: "Live organisations one person may have founded.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 1000,
  },
  members: {
    line: "Members one organisation may hold.",
    schema: { type: "integer", minimum: 2, maximum: 100000 },
    reality: 100000,
  },
  name: {
    line: "Longest name of a person.",
    schema: { type: "integer", minimum: 8, maximum: 128 },
    reality: 32,
  },
  orgname: {
    line: "Longest name of an organisation.",
    schema: { type: "integer", minimum: 12, maximum: 128 },
    reality: 40,
  },
  line: {
    line: "Longest status, or line about an organisation.",
    schema: { type: "integer", minimum: 20, maximum: 2000 },
    reality: 140,
  },
  bio: {
    line: "Longest bio.",
    schema: { type: "integer", minimum: 20, maximum: 5000 },
    reality: 280,
  },
  botsfound: {
    line: "Whether bots may found organisations at all.",
    schema: { type: "boolean" },
    reality: true,
  },
  kinds: {
    line: "The kinds of organisation that may be founded; the first is the default.",
    schema: { enum: [ORG_KINDS, ["company"], ["club", "school", "charity"], ["company", "union"]] },
    reality: ORG_KINDS,
  },
  palette: {
    line: "What kind each person is: human, agent, bot or org, or a list the seed draws from for each.",
    schema: {},
    reality: "human",
  },
} satisfies Knobs;

const ID = 40;

const ROLES: Role[] = ["owner", "admin", "member"];

const BOTS = ["bot", "agent"];

const TOP = ["orgs", "friends", "asks", "blocks", "mutes"];

const OWN = ["rename", "about", "look", "friend", "unfriend", "block", "unblock", "mute", "unmute", "found"];

const ORGANISED = [...OWN, "join", "admit", "invite", "remove", "leave", "role"];

const GLYPH = "^[.#]{5}( [.#]{5}){4}$";

export const cards = (state: People): Record<string, Person> =>
  Object.fromEntries(Object.entries(state).filter(([key]) => !TOP.includes(key))) as Record<string, Person>;

export function glyph(bits: number): string {
  const rows = Array.from({ length: 5 }, (_, y) => {
    const half = [0, 1, 2].map((x) => ((bits >>> (y * 3 + x)) & 1 ? "#" : "."));
    return [...half, half[1], half[0]].join("");
  });
  return rows.join(" ").includes("#") ? rows.join(" ") : "..... ..... ..#.. ..... .....";
}

const card = (id: string, kind: string, draw: () => number, hue = HUES[below(draw, HUES.length)]!): Person => ({
  name: named(id),
  hue,
  glyph: glyph(draw()),
  kind,
  bio: "",
  status: "",
});

function shuffled(draw: () => number): string[] {
  const out = [...HUES];
  for (let i = out.length - 1; i > 0; i--) {
    const j = below(draw, i + 1);
    [out[i], out[j]] = [out[j]!, out[i]!];
  }
  return out;
}

const count = (state: People) => Object.keys(state.orgs).length;

function orgOf(state: People, args: Obj, ctx: Ctx): Org {
  const one = known(state.orgs, String(args.org), "org", ctx);
  if (one.dissolved !== null) ctx.refuse(`${one.name} was dissolved`);
  return one;
}

const owners = (one: Org) => Object.keys(one.members).filter((id) => one.members[id] === "owner");

const leaders = (one: Org) => Object.keys(one.members).filter((id) => one.members[id] !== "member");

const leads = (one: Org, id: string | null) => id !== null && leaders(one).includes(id);

const put = (state: People, one: Org): People => ({ ...state, orgs: { ...state.orgs, [one.n]: one } });

const without = (members: Record<string, Role>, id: string) =>
  Object.fromEntries(Object.entries(members).filter(([one]) => one !== id));

const listed = (links: Links, id: string) => (Object.hasOwn(links, id) ? links[id]! : []);

const linked = (links: Links, id: string, other: string): Links => ({ ...links, [id]: [...listed(links, id), other] });

const unlinked = (links: Links, id: string, other: string): Links => ({
  ...links,
  [id]: listed(links, id).filter((one) => one !== other),
});

const askers = (state: People, id: string) => Object.keys(state.asks).filter((one) => state.asks[one]!.includes(id));

const apart = (state: People, a: string, b: string): People => ({
  ...state,
  friends: unlinked(unlinked(state.friends, a, b), b, a),
  asks: unlinked(unlinked(state.asks, a, b), b, a),
});

const enrolled = (state: People, ctx: Ctx): People =>
  Object.hasOwn(cards(state), ctx.who.id)
    ? state
    : { ...state, [ctx.who.id]: card(ctx.who.id, ctx.who.kind, ctx.draw) };

function edited(state: People, args: Obj, keys: string[], ctx: Ctx): People {
  const given = keys.filter((key) => args[key] !== undefined);
  if (!given.length) ctx.refuse(`give a ${keys.join(" or a ")}; nothing changed`);
  const base = enrolled(state, ctx);
  const change = Object.fromEntries(given.map((key) => [key, String(args[key]).trim()]));
  return { ...base, [ctx.who.id]: { ...cards(base)[ctx.who.id]!, ...change } };
}

const tag = (state: People, id: string) => {
  const name = cards(state)[id]?.name;
  return name ? `${name} (${id})` : id;
};

function tell(ctx: Ctx, ids: string[], text: string) {
  for (const id of ids) if (id !== ctx.who.id) ctx.wish(notice(id, text));
}

function other(state: People, args: Obj, ctx: Ctx): string {
  const person = String(args.person);
  if (person === ctx.who.id) return ctx.refuse("that is you; name someone else");
  if (!Object.hasOwn(cards(state), person)) return ctx.refuse(`no person ${person}; read people for everyone here`);
  return person;
}

function welcomed(one: Org, ctx: Ctx) {
  for (const id of leaders(one)) if (blockedBy(ctx, id, ctx.who.id)) ctx.refuse(`${id} is not taking this from you`);
}

function leading(one: Org, ctx: Ctx, doing: string) {
  if (!leads(one, ctx.who.id)) ctx.refuse(`only an owner or an admin of ${one.name} ${doing}`);
}

function befriended(state: People, me: string, person: string, ctx: Ctx): People {
  const most = ctx.knob("friends");
  if (listed(state.friends, person).length >= most)
    ctx.refuse(`${person} is at the limit of ${most} friends; ask again later`);
  tell(ctx, [person], `${tag(state, me)} and you are friends now`);
  const parted = apart(state, me, person);
  return { ...parted, friends: linked(linked(parted.friends, me, person), person, me) };
}

function asked(state: People, me: string, person: string, ctx: Ctx): People {
  tell(ctx, [person], `${tag(state, me)} asks to be your friend; answer with people.friend or people.unfriend`);
  return { ...state, asks: linked(state.asks, me, person) };
}

function parting(state: People, me: string, person: string): string | null {
  if (listed(state.friends, me).includes(person)) return `${tag(state, me)} is no longer your friend`;
  if (listed(state.asks, me).includes(person)) return `${tag(state, me)} took back the ask to be your friend`;
  if (listed(state.asks, person).includes(me)) return `${tag(state, me)} turned down your ask to be friends`;
  return null;
}

function seated(state: People, one: Org, person: string, ctx: Ctx): People {
  const most = ctx.knob("members");
  if (Object.keys(one.members).length >= most) ctx.refuse(`${one.name} is full at ${most} members; try another org`);
  return put(state, {
    ...one,
    members: { ...one.members, [person]: "member" },
    asking: one.asking.filter((id) => id !== person),
    invited: one.invited.filter((id) => id !== person),
  });
}

function admitted(state: People, one: Org, person: string, ctx: Ctx): People {
  const next = seated(state, one, person, ctx);
  ctx.say({ note: `${person} is a member of ${one.name}`, org: one.n, person });
  tell(ctx, [person], `You are a member of ${one.name}`);
  return next;
}

function joined(state: People, one: Org, ctx: Ctx): People {
  const next = seated(state, one, ctx.who.id, ctx);
  ctx.say({ note: `You are a member of ${one.name}`, org: one.n, member: true });
  tell(ctx, leaders(one), `${tag(state, ctx.who.id)} joined ${one.name}`);
  return next;
}

function departed(state: People, one: Org, ctx: Ctx): People {
  const me = ctx.who.id;
  const members = without(one.members, me);
  if (Object.keys(members).length) {
    tell(ctx, leaders({ ...one, members }), `${tag(state, me)} left ${one.name}`);
    return put(state, { ...one, members });
  }
  tell(ctx, [...one.asking, ...one.invited], `${one.name} was dissolved; its last member left`);
  return put(state, { ...one, members, asking: [], invited: [], dissolved: ctx.at });
}

const shownOrg = (one: Org, id: string | null) => ({
  name: one.name,
  line: one.line,
  kind: one.kind,
  open: one.open,
  by: one.by,
  founded: one.founded,
  members: one.members,
  ...(leads(one, id) ? { asking: one.asking, invited: one.invited } : {}),
});

const live = (state: People) => Object.values(state.orgs).filter((one) => one.dissolved === null);

const orgsFor = (state: People, id: string | null) =>
  Object.fromEntries(live(state).map((one) => [one.n, shownOrg(one, id)]));

const placed = (state: People, id: string) =>
  Object.fromEntries(
    live(state)
      .filter((one) => Object.hasOwn(one.members, id))
      .map((one) => [one.n, { name: one.name, kind: one.kind, role: one.members[id]! }]),
  );

const numbers = (state: People, hit: (one: Org) => boolean) =>
  live(state)
    .filter(hit)
    .map((one) => one.n);

const mine = (state: People, id: string) => ({
  me: id,
  friends: listed(state.friends, id),
  asks: { from: askers(state, id), to: listed(state.asks, id) },
  blocks: listed(state.blocks, id),
  mutes: listed(state.mutes, id),
  joined: placed(state, id),
  asking: numbers(state, (one) => one.asking.includes(id)),
  invited: numbers(state, (one) => one.invited.includes(id)),
  orgs: orgsFor(state, id),
});

const everyone = (state: People, who: Who | null) => {
  const all = cards(state);
  return who && all[who.id] ? { ...all, [who.id]: { ...all[who.id]!, kind: who.kind } } : all;
};

const whole = (state: People, who: Who | null) =>
  (who ? { ...everyone(state, who), ...mine(state, who.id) } : everyone(state, null)) as Json;

const publicly = (state: People, id: string): Json | undefined => {
  const one = Object.hasOwn(cards(state), id) ? cards(state)[id] : undefined;
  return one && { ...one, friends: listed(state.friends, id), orgs: placed(state, id) };
};

const slice = (state: People, id: string, who: Who | null) => {
  if (who?.id !== id) return publicly(state, id);
  const { orgs: _all, me: _me, ...rest } = mine(state, id);
  return { ...(publicly(state, id) as Obj | undefined), ...rest, kind: who.kind };
};

const seen = sliced(slice, whole, true);

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const choosing: Schema = {
  type: "object",
  properties: { person },
  required: ["person"],
  additionalProperties: false,
};

const renaming = (knob: Knob): Schema => ({
  type: "object",
  properties: { name: { type: "string", minLength: 1, maxLength: knob("name") } },
  required: ["name"],
  additionalProperties: false,
});

const describing = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    bio: { type: "string", maxLength: knob("bio") },
    status: { type: "string", maxLength: knob("line") },
  },
  additionalProperties: false,
});

const looking: Schema = {
  type: "object",
  properties: { hue: { enum: HUES }, glyph: { type: "string", pattern: GLYPH } },
  additionalProperties: false,
};

const naming = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { org: { type: "integer", minimum: 1 }, ...more },
  required: ["org", ...Object.keys(more)],
  additionalProperties: false,
});

const founding = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    name: { type: "string", minLength: 1, maxLength: knob("orgname") },
    line: { type: "string", maxLength: knob("line") },
    kind: { enum: knob<string[]>("kinds") },
    open: { type: "boolean" },
  },
  required: ["name", "open"],
  additionalProperties: false,
});

export default app<People>({
  id: "people",
  title: "People",
  line: "Who lives in this world: a card each with a name, hue, glyph, kind, bio and status; friends and blocks; and the organisations they found and join. read people/@id for anyone's public card, friends and orgs; people/@me is your short card with your own friends, asks, blocks and orgs, and people/@me/directory is everyone.",
  boot: ({ seed, actors, knob }) => {
    const { draw } = stream(seed);
    const hues = shuffled(draw);
    const kinds = palette(knob<Json>("palette"));
    return {
      ...Object.fromEntries(
        (actors ?? []).map((id, i) => [id, card(id, kindFor(seed, id, kinds), draw, hues[i % hues.length])]),
      ),
      orgs: {},
      friends: {},
      asks: {},
      blocks: {},
      mutes: {},
    };
  },
  verbs: {
    rename: {
      line: (knob) => `Rename yourself: 1 to ${knob("name")} characters, not like anyone else's name or id.`,
      args: renaming,
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        const me = ctx.who.id;
        const all = cards(state);
        clean(name, "a name", ctx);
        const held = Object.keys(all).find((id) => id !== me && (alike(name, all[id]!.name) || alike(name, id)));
        if (held) return ctx.refuse(`${name} looks too like ${tag(state, held)}; pick another name`);
        ctx.say({ note: `You are now called ${name}`, name });
        return edited(state, { name }, ["name"], ctx);
      },
    },
    about: {
      line: (knob) =>
        `Say who you are: a bio up to ${knob("bio")} characters, a status up to ${knob("line")}; give either or both, and an empty one clears it. Anyone can read them.`,
      args: describing,
      run: (state, args, ctx) => {
        const next = edited(state, args, ["bio", "status"], ctx);
        ctx.say({ note: `Set your ${["bio", "status"].filter((key) => args[key] !== undefined).join(" and ")}` });
        return next;
      },
    },
    look: {
      line: "Choose how you look: a hue, a glyph of 5 rows of 5, . or #, rows split by spaces, or both. Anyone can read them.",
      args: looking,
      run: (state, args, ctx) => {
        if (args.glyph !== undefined && !String(args.glyph).includes("#"))
          return ctx.refuse("a glyph needs at least one #");
        const next = edited(state, args, ["hue", "glyph"], ctx);
        ctx.say({ note: `Set your ${["hue", "glyph"].filter((key) => args[key] !== undefined).join(" and ")}` });
        return next;
      },
    },
    friend: {
      line: (knob) =>
        `Ask someone to be your friend; if they already asked you, you are friends now. Limit: ${knob("friends")} friends.`,
      args: choosing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = other(state, args, ctx);
        const most = ctx.knob("friends");
        reach(ctx, person);
        if (listed(state.friends, me).includes(person)) return ctx.refuse(`${person} is already your friend`);
        if (listed(state.asks, me).includes(person))
          return ctx.refuse(`you already asked ${person}; wait for them, or take it back with people.unfriend`);
        if (listed(state.friends, me).length >= most)
          return ctx.refuse(`you are at the limit of ${most} friends; use people.unfriend first`);
        const base = enrolled(state, ctx);
        const both = listed(base.asks, person).includes(me);
        ctx.say({
          note: both ? `You and ${person} are friends` : `Asked ${person} to be friends`,
          person,
          friends: both,
        });
        return both ? befriended(base, me, person, ctx) : asked(base, me, person, ctx);
      },
    },
    unfriend: {
      line: "End a friendship, take back your ask, or turn down someone's ask.",
      args: choosing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = other(state, args, ctx);
        const text = parting(state, me, person);
        if (!text) return ctx.refuse(`${person} is not your friend and no ask stands between you`);
        tell(ctx, [person], text);
        ctx.say({ note: `Nothing stands between you and ${person} now`, person });
        return apart(state, me, person);
      },
    },
    block: {
      line: "Block someone: it ends any friendship or ask between you, and they cannot reach you from any app. They are not told.",
      args: choosing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = other(state, args, ctx);
        if (listed(state.blocks, me).includes(person)) return ctx.refuse(`you already blocked ${person}`);
        ctx.say({ note: `Blocked ${person}`, person });
        return { ...apart(state, me, person), blocks: linked(state.blocks, me, person) };
      },
    },
    unblock: {
      line: "Stop blocking someone.",
      args: choosing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = other(state, args, ctx);
        if (!listed(state.blocks, me).includes(person)) return ctx.refuse(`you have not blocked ${person}`);
        ctx.say({ note: `Unblocked ${person}`, person });
        return { ...state, blocks: unlinked(state.blocks, me, person) };
      },
    },
    mute: {
      line: "Mute someone: they can still write to you, but raise no notices for you. They are not told.",
      args: choosing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = other(state, args, ctx);
        if (listed(state.mutes, me).includes(person)) return ctx.refuse(`you already muted ${person}`);
        ctx.say({ note: `Muted ${person}`, person });
        return { ...state, mutes: linked(state.mutes, me, person) };
      },
    },
    unmute: {
      line: "Stop muting someone.",
      args: choosing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = other(state, args, ctx);
        if (!listed(state.mutes, me).includes(person)) return ctx.refuse(`you have not muted ${person}`);
        ctx.say({ note: `Unmuted ${person}`, person });
        return { ...state, mutes: unlinked(state.mutes, me, person) };
      },
    },
    found: {
      line: (knob) =>
        `Found an organisation and own it: a name of 1 to ${knob("orgname")} characters nobody else holds, a kind (default ${knob<string[]>("kinds")[0]}); open lets anyone join.`,
      args: founding,
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        if (!ctx.knob<boolean>("botsfound") && BOTS.includes(ctx.who.kind))
          return ctx.refuse("in this world only humans found organisations; join one instead");
        const most = ctx.knob("founds");
        if (live(state).filter((one) => one.by === ctx.who.id && ctx.who.id in one.members).length >= most)
          return ctx.refuse(`you are at the limit of ${most} organisations founded; leave one or join another`);
        clean(name, "an organisation name", ctx);
        const held = live(state).find((one) => alike(name, one.name));
        if (held) return ctx.refuse(`org ${held.n} is already called ${held.name}; pick another name`);
        const n = count(state) + 1;
        const org: Org = {
          n,
          name,
          line: String(args.line ?? "").trim(),
          kind: String(args.kind ?? ctx.knob<string[]>("kinds")[0]),
          open: args.open === true,
          by: ctx.who.id,
          founded: ctx.at,
          dissolved: null,
          members: { [ctx.who.id]: "owner" },
          asking: [],
          invited: [],
        };
        ctx.say({ note: `Founded ${name} as org ${n}`, org: n });
        return put(state, org);
      },
    },
    join: {
      line: "Join an open organisation, accept an invite, or ask to join a closed one.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = orgOf(state, args, ctx);
        if (Object.hasOwn(one.members, me)) return ctx.refuse(`you are already a member of ${one.name}`);
        if (one.invited.includes(me)) return joined(state, one, ctx);
        welcomed(one, ctx);
        if (one.open) return joined(state, one, ctx);
        ctx.say({ note: `Asked to join ${one.name}; an owner or admin admits you`, org: one.n, member: false });
        if (one.asking.includes(me))
          return ctx.refuse(`you already asked to join ${one.name}; an owner or admin admits you`);
        tell(
          ctx,
          leaders(one),
          `${tag(state, me)} asks to join ${one.name} (org ${one.n}); answer with people.admit or people.remove`,
        );
        return put(state, { ...one, asking: [...one.asking, me] });
      },
    },
    admit: {
      line: "As an owner or admin, admit someone who asked to join.",
      args: naming({ person: person }),
      run: (state, args, ctx) => {
        const one = orgOf(state, args, ctx);
        const person = String(args.person);
        leading(one, ctx, "admits members");
        if (!one.asking.includes(person)) return ctx.refuse(`${person} has not asked to join ${one.name}`);
        reach(ctx, person);
        return admitted(state, one, person, ctx);
      },
    },
    invite: {
      line: "As an owner or admin, invite someone in; they accept with people.join or turn it down with people.leave. If they already asked, they are in.",
      args: naming({ person: person }),
      run: (state, args, ctx) => {
        const one = orgOf(state, args, ctx);
        const person = other(state, args, ctx);
        leading(one, ctx, "invites people");
        if (Object.hasOwn(one.members, person)) return ctx.refuse(`${person} is already a member of ${one.name}`);
        if (one.invited.includes(person)) return ctx.refuse(`${person} is already invited to ${one.name}`);
        reach(ctx, person);
        if (one.asking.includes(person)) return admitted(state, one, person, ctx);
        tell(
          ctx,
          [person],
          `${tag(state, ctx.who.id)} invites you to ${one.name} (org ${one.n}); accept with people.join or people.leave`,
        );
        ctx.say({ note: `Invited ${person} to ${one.name}`, org: one.n, person });
        return put(state, { ...one, invited: [...one.invited, person] });
      },
    },
    remove: {
      line: "As an owner or admin, remove a member, turn down an ask or withdraw an invite. An admin removes only plain members.",
      args: naming({ person: person }),
      run: (state, args, ctx) => {
        const one = orgOf(state, args, ctx);
        const person = other(state, args, ctx);
        leading(one, ctx, "removes people");
        if (one.asking.includes(person)) {
          tell(ctx, [person], `Your ask to join ${one.name} was turned down`);
          ctx.say({ note: `Turned down ${person}'s ask to join ${one.name}`, org: one.n, person });
          return put(state, { ...one, asking: one.asking.filter((id) => id !== person) });
        }
        if (one.invited.includes(person)) {
          tell(ctx, [person], `Your invite to ${one.name} was withdrawn`);
          ctx.say({ note: `Withdrew ${person}'s invite to ${one.name}`, org: one.n, person });
          return put(state, { ...one, invited: one.invited.filter((id) => id !== person) });
        }
        if (!Object.hasOwn(one.members, person))
          return ctx.refuse(`${person} is not in ${one.name} and did not ask to be`);
        if (one.members[ctx.who.id] === "admin" && one.members[person] !== "member")
          return ctx.refuse(`an admin removes only plain members; ask an owner of ${one.name}`);
        tell(ctx, [person], `You were removed from ${one.name}`);
        ctx.say({ note: `Removed ${person} from ${one.name}`, org: one.n, person });
        return put(state, { ...one, members: without(one.members, person) });
      },
    },
    leave: {
      line: "Leave an organisation, take back your ask or turn down an invite; the last member out dissolves it.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = orgOf(state, args, ctx);
        if (one.asking.includes(me)) {
          ctx.say({ note: `Took back your ask to join ${one.name}`, org: one.n });
          return put(state, { ...one, asking: one.asking.filter((id) => id !== me) });
        }
        if (one.invited.includes(me)) {
          tell(ctx, leaders(one), `${tag(state, me)} turned down the invite to ${one.name}`);
          ctx.say({ note: `Turned down the invite to ${one.name}`, org: one.n });
          return put(state, { ...one, invited: one.invited.filter((id) => id !== me) });
        }
        if (!(me in one.members)) return ctx.refuse(`you are not a member of ${one.name}`);
        const alone = owners(one).length === 1 && one.members[me] === "owner";
        if (alone && Object.keys(one.members).length > 1)
          return ctx.refuse(`you are the only owner of ${one.name}; make another member owner first`);
        ctx.say({ note: `You left ${one.name}`, org: one.n });
        return departed(state, one, ctx);
      },
    },
    role: {
      line: "As an owner, make a member an owner, an admin or a member.",
      args: naming({ person: person, role: { enum: ROLES } }),
      run: (state, args, ctx) => {
        const one = orgOf(state, args, ctx);
        const person = String(args.person);
        const role = args.role as Role;
        if (one.members[ctx.who.id] !== "owner") return ctx.refuse(`only an owner of ${one.name} sets roles`);
        if (!Object.hasOwn(one.members, person)) return ctx.refuse(`${person} is not a member of ${one.name}`);
        if (one.members[person] === role) return ctx.refuse(`${person} is already ${role} of ${one.name}`);
        if (role !== "owner" && owners(one).length === 1 && one.members[person] === "owner")
          return ctx.refuse(`${one.name} needs an owner; make another member owner first`);
        if (role !== "member") reach(ctx, person);
        tell(ctx, [person], `You are now ${role} of ${one.name}`);
        ctx.say({ note: `${person} is now ${role} of ${one.name}`, org: one.n, person, role });
        return put(state, { ...one, members: { ...one.members, [person]: role } });
      },
    },
  },
  moves: (state) => (count(state) ? ORGANISED : OWN),
  read: (state, path, who, at) => {
    if (!who && path[0] === "orgs") return walk(orgsFor(state, null) as Json, path.slice(1));
    const [head = "", next] = path;
    if (who && next === "directory" && head.startsWith("@") && [`@${who.id}`, "@me"].includes(head))
      return walk(everyone(state, who) as Json, path.slice(2));
    return seen(state, path, who, at);
  },
  knobs: KNOBS,
});
