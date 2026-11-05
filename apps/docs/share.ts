import type { Ctx, Json, Knob, Knobs, Obj, Schema, Verb, Who } from "@mrlyprod/os/contract/index.ts";
import { sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { notice, reach, someone } from "../shared/people.ts";

export type Shared = {
  owner: string;
  title: string;
  editors: string[];
  readers: string[];
  open: boolean;
  by: string;
  told: number | null;
  at: number;
};

export type Base = { people: string[] | null; next: number; listed: number };

export type Role = "owner" | "editor" | "reader" | "public";

export const SHARING = {
  title: { line: "Characters one title holds.", schema: { type: "integer", minimum: 10, maximum: 1000 }, reality: 100 },
  own: { line: "Things one person may own here.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 50 },
  sharers: {
    line: "People one thing may be shared with.",
    schema: { type: "integer", minimum: 1, maximum: 256 },
    reality: 20,
  },
  quiet: {
    line: "World milliseconds one person's changes to one thing stay quiet after a notice.",
    schema: { type: "integer", minimum: 0, maximum: 86400000 },
    reality: 300000,
  },
  listed: {
    line: "Public things one overview lists; deeper paths hold them all.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
} satisfies Knobs;

const ID = 40;

const ROLES = ["editor", "reader"] as const;

export const shape = (properties: Record<string, Schema>, optional: string[] = []): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties).filter((key) => !optional.includes(key)),
  additionalProperties: false,
});

export const roleOf = (one: Shared, me: string | null): Role | null => {
  if (me === one.owner) return "owner";
  if (me && one.editors.includes(me)) return "editor";
  if (me && one.readers.includes(me)) return "reader";
  return one.open ? "public" : null;
};

export const member = (role: Role | null) => role === "owner" || role === "editor" || role === "reader";

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

export const told = (ctx: Ctx, one: Shared, text: string) => {
  const me = ctx.who.id;
  for (const id of new Set([one.owner, ...one.editors, ...one.readers])) if (id !== me) ctx.wish(notice(id, text));
};

type Seen<T> = [string, T, Role];

export type Table<S extends Base, T extends Shared> = {
  id: string;
  noun: string;
  arg: string;
  prefix: string;
  many: string;
  items: (state: S) => Record<string, T>;
  put: (state: S, id: string, one: T | null) => S;
  blank: () => Omit<T, keyof Shared>;
  brief: (one: T) => Obj;
  full: (state: S, id: string, one: T) => Obj;
  more?: (state: S, me: string | null) => Obj;
};

export function sharing<S extends Base, T extends Shared>(table: Table<S, T>) {
  const { noun, arg, prefix, many, items, put, blank, brief, full, more } = table;
  const pointer: Schema = { type: "string", pattern: `^${prefix}-[0-9]+$`, maxLength: ID };
  const titled = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("title") });
  const opened = (state: S, id: string, ctx: Ctx): T => {
    const one = items(state)[id];
    return one && roleOf(one, ctx.who.id)
      ? one
      : ctx.refuse(`you can see no ${noun} ${id}; read ${many} for yours, or ask its owner to share it`);
  };
  const writable = (state: S, id: string, ctx: Ctx): T => {
    const one = opened(state, id, ctx);
    const role = roleOf(one, ctx.who.id);
    return role === "owner" || role === "editor"
      ? one
      : ctx.refuse(`${id} is ${one.owner}'s and you may only read it; ask them to make you an editor`);
  };
  const owned = (state: S, id: string, ctx: Ctx): T => {
    const one = opened(state, id, ctx);
    return one.owner === ctx.who.id ? one : ctx.refuse(`${id} is ${one.owner}'s; only its owner may do that`);
  };
  const changed = (id: string, was: T, ctx: Ctx, what: string): T => {
    const me = ctx.who.id;
    const quiet = was.by === me && was.told !== null && ctx.at - was.told < ctx.knob("quiet");
    if (!quiet) told(ctx, was, `${me} ${what} ${id}: ${was.title}`);
    return { ...was, at: ctx.at, by: me, told: quiet ? was.told : ctx.at };
  };
  const row = (one: T, role: Role): Obj => ({
    title: one.title,
    owner: one.owner,
    role,
    at: one.at,
    by: one.by,
    ...(member(role) ? { editors: one.editors, readers: one.readers, public: one.open } : {}),
    ...brief(one),
  });
  const visible = (state: S, me: string | null) =>
    Object.entries(items(state)).flatMap(([id, one]): Seen<T>[] => {
      const role = roleOf(one, me);
      return role ? [[id, one, role]] : [];
    });
  const keyed = (list: Seen<T>[], shown: (id: string, one: T, role: Role) => Obj) =>
    Object.fromEntries(list.map(([id, one, role]) => [id, shown(id, one, role)]));
  const held = (list: Seen<T>[]) => list.filter(([, , role]) => role !== "public");
  const open = (list: Seen<T>[]) => list.filter(([, , role]) => role === "public").sort(([, a], [, b]) => b.at - a.at);
  const overview = (state: S, me: string | null): Json => {
    const list = visible(state, me);
    const shown = (_id: string, one: T, role: Role) => row(one, role);
    return { [many]: keyed(held(list), shown), public: keyed(open(list).slice(0, state.listed), shown) };
  };
  const everything = (state: S, me: string | null): Obj => {
    const list = visible(state, me);
    const shown = (id: string, one: T, role: Role) => ({ ...row(one, role), ...full(state, id, one) });
    return { [many]: keyed(held(list), shown), public: keyed(open(list), shown) };
  };
  const deep = sliced<S>(
    (state, id) => everything(state, id),
    (state, who) => ({ ...everything(state, who?.id ?? null), ...more?.(state, who?.id ?? null) }),
  );
  const oneAt = (state: S, path: string[], me: string | null): Json | undefined => {
    const id = path[1]!;
    const one = Object.hasOwn(items(state), id) ? items(state)[id] : undefined;
    const role = one ? roleOf(one, me) : null;
    return one && role ? walk({ ...row(one, role), ...full(state, id, one) }, path.slice(2)) : undefined;
  };
  const read = (state: S, path: string[], who: Who | null, at = 0): Json | undefined => {
    const me = who?.id ?? null;
    if (!path.length) return overview(state, me);
    return path[0] === many && path[1] !== undefined ? oneAt(state, path, me) : deep(state, path, who, at);
  };
  const verbs: Record<string, Verb<S>> = {
    create: {
      line: (knob) =>
        `Start a ${noun} of yours, private until you share it, titled 1 to ${knob("title")} characters; you hold at most ${knob("own")}.`,
      args: (knob) => shape({ title: titled(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.title).trim();
        const most = ctx.knob("own");
        if (!name) return ctx.refuse("a title needs more than spaces");
        if (Object.values(items(state)).filter((one) => one.owner === me).length >= most)
          return ctx.refuse(`you hold ${most} ${many} already; delete one first`);
        const one = {
          owner: me,
          title: name,
          editors: [],
          readers: [],
          open: false,
          by: me,
          told: null,
          at: ctx.at,
          ...blank(),
        } as unknown as T;
        const id = `${prefix}-${state.next}`;
        ctx.say({ note: `Started ${id}: ${name}`, id });
        return put({ ...state, next: state.next + 1 }, id, one);
      },
    },
    rename: {
      line: `Change the title of a ${noun} you own or edit.`,
      args: (knob) => shape({ [arg]: pointer, title: titled(knob) }),
      run: (state, args, ctx) => {
        const id = String(args[arg]);
        const was = writable(state, id, ctx);
        const name = String(args.title).trim();
        if (!name) return ctx.refuse("a title needs more than spaces");
        ctx.say({ note: `Renamed ${id}: ${name}`, id });
        return put(state, id, { ...changed(id, was, ctx, "renamed"), title: name });
      },
    },
    share: {
      line: (knob) =>
        `Share a ${noun} of yours with someone and tell them: role editor (the default) may change it, reader only reads; at most ${knob("sharers")} people. Share again to change a role.`,
      args: shape({ [arg]: pointer, with: person, role: { enum: [...ROLES] } }, ["role"]),
      run: (state, args, ctx) => {
        const id = String(args[arg]);
        const me = ctx.who.id;
        const was = owned(state, id, ctx);
        const who = someone(state.people, String(args.with), ctx);
        const role = args.role === "reader" ? "reader" : "editor";
        const most = ctx.knob("sharers");
        if (who === me) return ctx.refuse("you own it already; share with someone else");
        reach(ctx, who);
        const fresh = !was.editors.includes(who) && !was.readers.includes(who);
        if (roleOf(was, who) === role) return ctx.refuse(`${who} is already a ${role} of ${id}`);
        if (fresh && was.editors.length + was.readers.length >= most)
          return ctx.refuse(`${id} is shared with ${most} people already; unshare someone first`);
        ctx.wish(notice(who, `${me} shared ${id} with you as ${role}: ${was.title}`));
        ctx.say({ note: `Shared ${id} with ${who} as ${role}`, id, with: who, role });
        return put(state, id, {
          ...was,
          told: null,
          editors: role === "editor" ? [...was.editors, who] : was.editors.filter((one) => one !== who),
          readers: role === "reader" ? [...was.readers, who] : was.readers.filter((one) => one !== who),
        });
      },
    },
    unshare: {
      line: `Stop sharing a ${noun} of yours with someone, or leave one shared with you by naming yourself.`,
      args: shape({ [arg]: pointer, with: person }),
      run: (state, args, ctx) => {
        const id = String(args[arg]);
        const me = ctx.who.id;
        const who = String(args.with);
        const was = opened(state, id, ctx);
        if (was.owner !== me && who !== me)
          return ctx.refuse(`${id} is ${was.owner}'s; you may only leave it yourself`);
        if (!was.editors.includes(who) && !was.readers.includes(who))
          return ctx.refuse(`${id} is not shared with ${who}`);
        const to = who === me ? was.owner : who;
        ctx.wish(notice(to, who === me ? `${me} left ${id}: ${was.title}` : `${me} unshared ${id}: ${was.title}`));
        ctx.say({ note: who === me ? `Left ${id}` : `Stopped sharing ${id} with ${who}`, id });
        return put(state, id, {
          ...was,
          editors: was.editors.filter((one) => one !== who),
          readers: was.readers.filter((one) => one !== who),
        });
      },
    },
    publish: {
      line: `Open a ${noun} of yours for everyone to read, or close it again with on: false.`,
      args: () => shape({ [arg]: pointer, on: { type: "boolean" } }),
      run: (state, args, ctx) => {
        const id = String(args[arg]);
        const on = Boolean(args.on);
        const was = owned(state, id, ctx);
        if (was.open === on) return ctx.refuse(on ? `${id} is public already` : `${id} is not public`);
        told(ctx, was, `${ctx.who.id} made ${id} ${on ? "public" : "private"}: ${was.title}`);
        ctx.say({ note: `${id} is ${on ? "public" : "private"}`, id, public: on });
        return put(state, id, { ...was, open: on });
      },
    },
    delete: {
      line: `Delete a ${noun} of yours; whoever it is shared with is told.`,
      args: () => shape({ [arg]: pointer }),
      run: (state, args, ctx) => {
        const id = String(args[arg]);
        const was = owned(state, id, ctx);
        told(ctx, was, `${ctx.who.id} deleted ${id}: ${was.title}`);
        ctx.say({ note: `Deleted ${id}`, id });
        return put(state, id, null);
      },
    },
  };
  return { pointer, writable, changed, verbs, read };
}
