import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, someone } from "../shared/people.ts";

export type Entry = { kind: "file" | "folder"; text: string; at: number; with?: string[] };

export type Store = Record<string, Entry>;

export type Files = { people: string[] | null; stores: Record<string, Store> };

const ID = 40;

const KNOBS = {
  path: { line: "Characters one path holds.", schema: { type: "integer", minimum: 20, maximum: 2000 }, reality: 200 },
  name: {
    line: "Characters one file or folder name holds.",
    schema: { type: "integer", minimum: 5, maximum: 255 },
    reality: 40,
  },
  depth: { line: "How many names deep a path goes.", schema: { type: "integer", minimum: 1, maximum: 32 }, reality: 8 },
  text: {
    line: "Characters one file holds.",
    schema: { type: "integer", minimum: 100, maximum: 100000 },
    reality: 4000,
  },
  entries: {
    line: "Files and folders one store holds.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 200,
  },
  sharers: {
    line: "People one file may be shared with.",
    schema: { type: "integer", minimum: 1, maximum: 256 },
    reality: 20,
  },
} satisfies Knobs;

const SEGMENT = /^[A-Za-z0-9._ -]+$/;

const RESERVED = "__proto__";

export const parent = (path: string) => path.slice(0, path.lastIndexOf("/")) || "/";

export function clean(path: string, ctx: Ctx): string {
  const { refuse, knob } = ctx;
  const depth = knob("depth");
  const name = knob("name");
  if (!path.startsWith("/") || path === "/")
    return refuse(`a path starts with / and names something in your store, like /notes/todo.txt, not ${path}`);
  const parts = path.slice(1).split("/");
  if (parts.some((part) => part === "." || part === ".."))
    return refuse("a path stays inside your store; . and .. are not names");
  if (parts.length > depth) return refuse(`a path is at most ${depth} names deep`);
  for (const part of parts)
    if (!part || part === RESERVED || part.length > name || !SEGMENT.test(part) || part.trim() !== part)
      return refuse(`each name is 1 to ${name} letters, digits, spaces, dots, hyphens or underscores, not "${part}"`);
  return path;
}

const under = (path: string, root: string) => path === root || path.startsWith(`${root}/`);

function placed(store: Store, path: string, refuse: Ctx["refuse"]) {
  const up = parent(path);
  if (up !== "/" && store[up]?.kind !== "folder")
    return refuse(store[up] ? `${up} is a file, not a folder` : `no folder ${up}; make it first`);
}

const full = (store: Store, adding: number, ctx: Ctx) => {
  const most = ctx.knob("entries");
  if (Object.keys(store).length + adding > most) ctx.refuse(`your store holds at most ${most} files and folders`);
};

function tree(store: Store, leaf: (entry: Entry) => Json): Record<string, Json> {
  const root: Record<string, Json> = {};
  for (const path of Object.keys(store).sort()) {
    const parts = path.slice(1).split("/");
    let at = root;
    for (const part of parts.slice(0, -1)) {
      if (!Object.hasOwn(at, part)) at[part] = {};
      at = at[part] as Record<string, Json>;
    }
    const entry = store[path]!;
    const name = parts.at(-1)!;
    at[name] = entry.kind === "file" ? leaf(entry) : Object.hasOwn(at, name) ? at[name]! : {};
  }
  return root;
}

const sharedWith = (store: Store, me: string): Store =>
  Object.fromEntries(Object.entries(store).filter(([, entry]) => entry.with?.includes(me)));

const given = (state: Files, me: string, leaf: (entry: Entry) => Json) =>
  Object.fromEntries(
    Object.entries(state.stores)
      .filter(([owner, store]) => owner !== me && Object.keys(sharedWith(store, me)).length)
      .map(([owner, store]) => [owner, tree(sharedWith(store, me), leaf)]),
  );

const sharing = (state: Files, me: string) =>
  Object.entries(state.stores[me] ?? {})
    .filter(([, entry]) => entry.with?.length)
    .map(([path, entry]) => ({ path, with: entry.with! }));

const size = (entry: Entry) => entry.text.length;

const words = (entry: Entry) => entry.text;

const view = (state: Files, me: string, leaf: (entry: Entry) => Json): Json => ({
  store: tree(state.stores[me] ?? {}, leaf),
  shared: given(state, me, leaf),
  sharing: sharing(state, me),
});

const count = (state: Files): Json => ({
  people: Object.keys(state.stores).length,
  entries: Object.values(state.stores).reduce((sum, store) => sum + Object.keys(store).length, 0),
});

const deep = sliced<Files>(
  (state, id) => view(state, id, words),
  (state, who) => (who ? view(state, who.id, words) : count(state)),
);

const read = (state: Files, path: string[], who: Who | null, at = 0): Json | undefined => {
  if (path.length) return deep(state, path, who, at);
  return who ? view(state, who.id, size) : count(state);
};

const tell = (ctx: Ctx, entry: Entry | undefined, text: string) => {
  for (const who of entry?.with ?? []) ctx.wish(notice(who, text));
};

const path = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("path") });

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

export default app<Files>({
  id: "files",
  title: "Files",
  line: "A store of small text files and folders per user, sandboxed: a path names your own store, never another's, never the machine. The overview shows sizes: read files/store/<path> for the text, and files/shared/<person>/<path> for what someone shared with you.",
  boot: ({ actors }) => ({ people: actors, stores: {} }),
  verbs: {
    write: {
      line: (knob) =>
        `Write a text file of up to ${knob("text")} characters at a path in your store, replacing what is there; its folder must exist, and whoever it is shared with hears.`,
      args: (knob) => shape({ path: path(knob), text: { type: "string", maxLength: knob("text") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx);
        const store = state.stores[me] ?? {};
        if (store[at]?.kind === "folder") return ctx.refuse(`${at} is a folder`);
        placed(store, at, ctx.refuse);
        full(store, store[at] ? 0 : 1, ctx);
        tell(ctx, store[at], `${me} changed ${at}, shared with you`);
        const entry: Entry = { ...store[at], kind: "file", text: String(args.text), at: ctx.at };
        ctx.say({ note: `Wrote ${at}, ${entry.text.length} characters; read files/store${at}`, path: at });
        return { ...state, stores: { ...state.stores, [me]: { ...store, [at]: entry } } };
      },
    },
    folder: {
      line: "Make a folder at a path in your store; its parent must exist.",
      args: (knob) => shape({ path: path(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx);
        const store = state.stores[me] ?? {};
        if (store[at]) return ctx.refuse(`${at} already exists`);
        placed(store, at, ctx.refuse);
        full(store, 1, ctx);
        ctx.say({ note: `Made folder ${at}`, path: at });
        return {
          ...state,
          stores: { ...state.stores, [me]: { ...store, [at]: { kind: "folder", text: "", at: ctx.at } } },
        };
      },
    },
    move: {
      line: "Move or rename a file or a folder, with all it holds, within your store.",
      args: (knob) => shape({ from: path(knob), to: path(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const depth = ctx.knob("depth");
        const from = clean(String(args.from), ctx);
        const to = clean(String(args.to), ctx);
        const store = state.stores[me] ?? {};
        if (!store[from]) return ctx.refuse(`no ${from} in your store`);
        if (store[to]) return ctx.refuse(`${to} already exists`);
        if (under(to, from)) return ctx.refuse(`${from} cannot move inside itself`);
        placed(store, to, ctx.refuse);
        const moved = Object.entries(store).map(([key, entry]): [string, Entry] =>
          under(key, from) ? [to + key.slice(from.length), entry] : [key, entry],
        );
        const deep = moved.find(([key]) => key.split("/").length - 1 > depth);
        if (deep) return ctx.refuse(`${deep[0]} would be more than ${depth} names deep`);
        ctx.say({ note: `Moved ${from} to ${to}`, path: to });
        return { ...state, stores: { ...state.stores, [me]: Object.fromEntries(moved) } };
      },
    },
    delete: {
      line: "Delete a file, or a folder with all it holds, from your store; whoever a file was shared with hears.",
      args: (knob) => shape({ path: path(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx);
        const store = state.stores[me] ?? {};
        if (!store[at]) return ctx.refuse(`no ${at} in your store`);
        const doomed = Object.entries(store).filter(([key]) => under(key, at));
        for (const [key, entry] of doomed) tell(ctx, entry, `${me} deleted ${key}, shared with you`);
        ctx.say({ note: `Deleted ${at}${doomed.length > 1 ? ` and ${doomed.length - 1} under it` : ""}`, path: at });
        return {
          ...state,
          stores: {
            ...state.stores,
            [me]: Object.fromEntries(Object.entries(store).filter(([key]) => !under(key, at))),
          },
        };
      },
    },
    share: {
      line: (knob) =>
        `Let someone read a file of your store, and tell them; it shows in their files/shared/<you>. They see every later change. At most ${knob("sharers")} people per file.`,
      args: (knob) => shape({ path: path(knob), with: person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx);
        const who = someone(state.people, String(args.with), ctx);
        const store = state.stores[me] ?? {};
        const entry = store[at];
        const most = ctx.knob("sharers");
        if (entry?.kind !== "file") return ctx.refuse(`no file ${at} in your store; only a file can be shared`);
        if (who === me) return ctx.refuse("it is yours already; share it with someone else");
        if (blockedBy(ctx, who, me)) return ctx.refuse(`${who} is not taking this from you`);
        const list = entry.with ?? [];
        if (list.includes(who)) return ctx.refuse(`${at} is shared with ${who} already`);
        if (list.length >= most) return ctx.refuse(`${at} is shared with ${most} people; unshare someone first`);
        ctx.wish(notice(who, `${me} shared a file with you: ${at}. Read files/shared/${me}${at}`));
        ctx.say({ note: `Shared ${at} with ${who}`, path: at, with: who });
        return {
          ...state,
          stores: { ...state.stores, [me]: { ...store, [at]: { ...entry, with: [...list, who] } } },
        };
      },
    },
    unshare: {
      line: "Stop sharing a file of your store with someone; they are told.",
      args: (knob) => shape({ path: path(knob), with: person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx);
        const who = String(args.with);
        const store = state.stores[me] ?? {};
        const entry = store[at];
        if (!entry?.with?.includes(who)) return ctx.refuse(`${at} is not shared with ${who}`);
        ctx.wish(notice(who, `${me} stopped sharing ${at} with you`));
        ctx.say({ note: `Stopped sharing ${at} with ${who}`, path: at, with: who });
        const list = entry.with.filter((one) => one !== who);
        return { ...state, stores: { ...state.stores, [me]: { ...store, [at]: { ...entry, with: list } } } };
      },
    },
  },
  read,
  knobs: KNOBS,
});
