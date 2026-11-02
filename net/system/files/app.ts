import type { Ctx, Json, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Entry = { kind: "file" | "folder"; text: string; at: number };

export type Store = Record<string, Entry>;

export type Files = { stores: Record<string, Store> };

export const PATH = 200;

export const NAME = 40;

export const DEPTH = 8;

export const TEXT = 4000;

export const ENTRIES = 200;

/* PATHS */

const SEGMENT = /^[A-Za-z0-9._ -]+$/;

export const parent = (path: string) => path.slice(0, path.lastIndexOf("/")) || "/";

export function clean(path: string, refuse: Ctx["refuse"]): string {
  if (!path.startsWith("/") || path === "/")
    return refuse(`a path starts with / and names something in your store, like /notes/todo.txt, not ${path}`);
  const parts = path.slice(1).split("/");
  if (parts.some((part) => part === "." || part === ".."))
    return refuse("a path stays inside your store; . and .. are not names");
  if (parts.length > DEPTH) return refuse(`a path is at most ${DEPTH} names deep`);
  for (const part of parts)
    if (!part || part.length > NAME || !SEGMENT.test(part) || part.trim() !== part)
      return refuse(`each name is 1 to ${NAME} letters, digits, spaces, dots, hyphens or underscores, not "${part}"`);
  return path;
}

const under = (path: string, root: string) => path === root || path.startsWith(`${root}/`);

function placed(store: Store, path: string, refuse: Ctx["refuse"]) {
  const up = parent(path);
  if (up !== "/" && store[up]?.kind !== "folder")
    return refuse(store[up] ? `${up} is a file, not a folder` : `no folder ${up}; make it first`);
}

const full = (store: Store, adding: number, refuse: Ctx["refuse"]) => {
  if (Object.keys(store).length + adding > ENTRIES) refuse(`your store holds at most ${ENTRIES} files and folders`);
};

function tree(store: Store): Json {
  const root: Record<string, Json> = {};
  for (const path of Object.keys(store).sort()) {
    const parts = path.slice(1).split("/");
    let at = root;
    for (const part of parts.slice(0, -1)) at = at[part] as Record<string, Json>;
    const entry = store[path]!;
    at[parts.at(-1)!] = entry.kind === "folder" ? {} : entry.text;
  }
  return root;
}

/* SCHEMAS */

const path: Schema = { type: "string", minLength: 1, maxLength: PATH };

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

/* APP */

export default app<Files>({
  id: "files",
  title: "Files",
  line: "A store of small text files and folders per user, sandboxed: a path names your own store, never another's, never the machine.",
  boot: () => ({ stores: {} }),
  verbs: {
    write: {
      line: `Write a text file of up to ${TEXT} characters at a path in your store, replacing what is there; its folder must exist.`,
      args: shape({ path, text: { type: "string", maxLength: TEXT } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx.refuse);
        const store = state.stores[me] ?? {};
        if (store[at]?.kind === "folder") return ctx.refuse(`${at} is a folder`);
        placed(store, at, ctx.refuse);
        full(store, store[at] ? 0 : 1, ctx.refuse);
        return {
          stores: { ...state.stores, [me]: { ...store, [at]: { kind: "file", text: String(args.text), at: ctx.at } } },
        };
      },
    },
    folder: {
      line: "Make a folder at a path in your store; its parent must exist.",
      args: shape({ path }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx.refuse);
        const store = state.stores[me] ?? {};
        if (store[at]) return ctx.refuse(`${at} already exists`);
        placed(store, at, ctx.refuse);
        full(store, 1, ctx.refuse);
        return { stores: { ...state.stores, [me]: { ...store, [at]: { kind: "folder", text: "", at: ctx.at } } } };
      },
    },
    move: {
      line: "Move or rename a file or a folder, with all it holds, within your store.",
      args: shape({ from: path, to: path }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const from = clean(String(args.from), ctx.refuse);
        const to = clean(String(args.to), ctx.refuse);
        const store = state.stores[me] ?? {};
        if (!store[from]) return ctx.refuse(`no ${from} in your store`);
        if (store[to]) return ctx.refuse(`${to} already exists`);
        if (under(to, from)) return ctx.refuse(`${from} cannot move inside itself`);
        placed(store, to, ctx.refuse);
        const moved = Object.entries(store).map(([key, entry]): [string, Entry] =>
          under(key, from) ? [to + key.slice(from.length), entry] : [key, entry],
        );
        const deep = moved.find(([key]) => key.split("/").length - 1 > DEPTH);
        if (deep) return ctx.refuse(`${deep[0]} would be more than ${DEPTH} names deep`);
        return { stores: { ...state.stores, [me]: Object.fromEntries(moved) } };
      },
    },
    delete: {
      line: "Delete a file, or a folder with all it holds, from your store.",
      args: shape({ path }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const at = clean(String(args.path), ctx.refuse);
        const store = state.stores[me] ?? {};
        if (!store[at]) return ctx.refuse(`no ${at} in your store`);
        return {
          stores: {
            ...state.stores,
            [me]: Object.fromEntries(Object.entries(store).filter(([key]) => !under(key, at))),
          },
        };
      },
    },
  },
  read: sliced((state: Files, id: string) => tree(state.stores[id] ?? {})),
  bot,
});
