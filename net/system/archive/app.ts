import type { Json, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Page = { id: number; by: string; path: string; tick: number; at: number; page: Json };

export type Archive = { next: number; pages: Record<string, Page> };

export const PATH = 200;

export const SIZE = 4000;

export const KEEP = 20;

/* PAGES */

export const pathOf = (raw: string) => raw.split("/").filter(Boolean).join("/");

const mine = (state: Archive, id: string) => ({
  pages: Object.values(state.pages)
    .filter((one) => one.by === id)
    .sort((a, b) => b.id - a.id),
});

/* SCHEMAS */

const saving: Schema = {
  type: "object",
  properties: { path: { type: "string", minLength: 1, maxLength: PATH } },
  required: ["path"],
  additionalProperties: false,
};

const naming = (state: Archive): Schema => ({
  type: "object",
  properties: { id: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) } },
  required: ["id"],
  additionalProperties: false,
});

/* APP */

export default app<Archive>({
  id: "archive",
  title: "Archive",
  line: "Pages saved from inside the world: what another app answered at a tick, kept as it was.",
  boot: () => ({ next: 1, pages: {} }),
  verbs: {
    save: {
      line: `Save a page: the path of a read, as people/test-1, kept as it answers now, at most ${SIZE} characters as JSON; you keep at most ${KEEP}.`,
      args: saving,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const path = pathOf(String(args.path));
        if (!path) return ctx.refuse("a path names an app, then a key inside it: people/test-1");
        if (path.split("/")[0] === "archive") return ctx.refuse("the archive does not save itself");
        const page = ctx.read(path);
        if (page === null) return ctx.refuse(`nothing answers at ${path}`);
        const size = JSON.stringify(page).length;
        if (size > SIZE) return ctx.refuse(`${path} is ${size} characters as JSON, over ${SIZE}; read a smaller path`);
        if (mine(state, me).pages.length >= KEEP) return ctx.refuse(`you keep ${KEEP} pages; delete one first`);
        const id = state.next;
        return {
          next: id + 1,
          pages: { ...state.pages, [id]: { id, by: me, path, tick: ctx.tick, at: ctx.at, page } },
        };
      },
    },
    delete: {
      line: "Delete a saved page of yours.",
      args: naming,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const one = state.pages[id];
        if (!one) return ctx.refuse(`no page ${id}; it was deleted`);
        if (one.by !== ctx.who.id) return ctx.refuse(`page ${id} is ${one.by}'s; only they delete it`);
        const pages = { ...state.pages };
        delete pages[id];
        return { ...state, pages };
      },
    },
  },
  moves: (state) => (Object.keys(state.pages).length ? ["save", "delete"] : ["save"]),
  read: sliced(mine),
  bot,
});
