import type { Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, canon, sliced } from "@mrlyprod/os/contract/index.ts";

export type Page = { id: number; by: string; path: string; at: number; page: Json };

export type Archive = { next: number; pages: Record<string, Page> };

const ID = "archive";

const KNOBS = {
  path: {
    line: "Characters a saved path may hold.",
    schema: { type: "integer", minimum: 20, maximum: 2000 },
    reality: 200,
  },
  size: {
    line: "Characters of JSON one saved page may hold.",
    schema: { type: "integer", minimum: 100, maximum: 100000 },
    reality: 4000,
  },
  keep: { line: "Pages one person keeps.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 20 },
} satisfies Knobs;

export const pathOf = (raw: string) => raw.split("/").filter(Boolean).join("/");

const mine = (state: Archive, id: string) => ({
  pages: Object.values(state.pages)
    .filter((one) => one.by === id)
    .sort((a, b) => b.id - a.id),
});

const brief = ({ id, path, at, page }: Page) => ({ id, path, at, size: canon(page).length });

const whole = (state: Archive, who: Who | null): Json =>
  who
    ? { pages: mine(state, who.id).pages.map(brief), more: "archive/@me/pages holds each page in full" }
    : { saved: Object.keys(state.pages).length };

const saving = (knob: Knob): Schema => ({
  type: "object",
  properties: { path: { type: "string", minLength: 1, maxLength: knob("path") } },
  required: ["path"],
  additionalProperties: false,
});

const naming: Schema = {
  type: "object",
  properties: { id: { type: "integer", minimum: 1 } },
  required: ["id"],
  additionalProperties: false,
};

export default app<Archive>({
  id: ID,
  title: "Archive",
  line: "Your private shelf of pages saved from inside the world: what you could see at a tick, kept as it was.",
  boot: () => ({ next: 1, pages: {} }),
  verbs: {
    save: {
      line: (knob) =>
        `Save a page: the path of a read, as people/@someone, kept as it answers you now, at most ${knob("size")} characters as JSON; you keep at most ${knob("keep")}.`,
      args: saving,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const path = pathOf(String(args.path));
        const size = ctx.knob("size");
        const keep = ctx.knob("keep");
        if (!path) return ctx.refuse("a path names an app, then a key inside it: people/@test-1");
        if (path.split("/")[0] === ID) return ctx.refuse("the archive does not save itself");
        const page = ctx.see(path);
        if (page === null) return ctx.refuse(`nothing you may see at ${path}; read an app first, then a key inside it`);
        const chars = canon(page).length;
        if (chars > size)
          return ctx.refuse(`${path} is ${chars} characters as JSON, over ${size}; save a smaller path`);
        if (mine(state, me).pages.length >= keep)
          return ctx.refuse(`your shelf is full at ${keep}; delete a page first`);
        const id = state.next;
        ctx.say({ note: `Saved ${path} as page ${id}; archive/@me/pages holds it`, id, path });
        return {
          next: id + 1,
          pages: { ...state.pages, [id]: { id, by: me, path, at: ctx.at, page } },
        };
      },
    },
    delete: {
      line: "Delete a saved page of yours.",
      args: naming,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const one = state.pages[id];
        if (!one || one.by !== ctx.who.id) return ctx.refuse(`no page ${id} of yours; read archive to see yours`);
        const pages = { ...state.pages };
        delete pages[id];
        ctx.say({ note: `Deleted page ${id}`, id });
        return { ...state, pages };
      },
    },
  },
  read: sliced(mine, whole),
  knobs: KNOBS,
});
