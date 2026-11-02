import type { Ctx, Obj, Schema } from "../../sdk/system.ts";
import { act, app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Cursor = { line: number; col: number };

export type Session = { open: string | null; buffer: string[]; cursor: Cursor; dirty: boolean };

export type Code = { sessions: Record<string, Session> };

type Entry = { kind: "file" | "folder"; text: string };

export const FILES = "files";

export const PATH = 200;

export const LINE = 200;

export const LINES = 500;

export const TEXT = 4000;

const SEGMENT = /^[A-Za-z0-9._ -]+$/;

const NONE: Session = { open: null, buffer: [], cursor: { line: 0, col: 0 }, dirty: false };

/* STORES */

const store = (ctx: Ctx): Record<string, Entry> => {
  const got = ctx.read(`${FILES}/stores/${ctx.who.id}`);
  if (got === null) {
    if (ctx.read(FILES) === null) ctx.refuse(`this world holds no ${FILES} app; code edits the files app's store`);
    return {};
  }
  return got as Record<string, Entry>;
};

const parent = (path: string) => path.slice(0, path.lastIndexOf("/")) || "/";

function clean(path: string, refuse: Ctx["refuse"]): string {
  if (!path.startsWith("/") || path === "/")
    return refuse(`a path starts with / and names a file in your store, like /src/main.ts, not ${path}`);
  const parts = path.slice(1).split("/");
  for (const part of parts)
    if (!part || part.length > 40 || !SEGMENT.test(part) || part.trim() !== part)
      return refuse(`each name is 1 to 40 letters, digits, spaces, dots, hyphens or underscores, not "${part}"`);
  return path;
}

const session = (state: Code, ctx: Ctx): Session => {
  const one = state.sessions[ctx.who.id];
  if (!one?.open) return ctx.refuse("open a file first");
  return one;
};

const put = (state: Code, me: string, one: Session): Code => ({ sessions: { ...state.sessions, [me]: one } });

/* SCHEMAS */

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const path: Schema = { type: "string", minLength: 1, maxLength: PATH };

const line: Schema = { type: "integer", minimum: 0, maximum: LINES - 1 };

/* APP */

export default app<Code>({
  id: "code",
  title: "Code",
  line: "The editor: open a file of your store, edit it line by line with a cursor, save it back.",
  boot: () => ({ sessions: {} }),
  verbs: {
    open: {
      line: "Open a file of your store into your buffer; the cursor goes to its first line.",
      args: shape({ path }),
      run: (state, args, ctx) => {
        const at = clean(String(args.path), ctx.refuse);
        const entry = store(ctx)[at];
        if (entry?.kind !== "file") return ctx.refuse(`no file ${at} in your store; code.new starts one`);
        return put(state, ctx.who.id, {
          open: at,
          buffer: entry.text.split("\n"),
          cursor: { line: 0, col: 0 },
          dirty: false,
        });
      },
    },
    new: {
      line: "Start a new file at a path of your store, in a folder that exists; it lands in the store on save.",
      args: shape({ path }),
      run: (state, args, ctx) => {
        const at = clean(String(args.path), ctx.refuse);
        const files = store(ctx);
        if (files[at]) return ctx.refuse(`${at} exists already; code.open opens it`);
        const up = parent(at);
        if (up !== "/" && files[up]?.kind !== "folder") return ctx.refuse(`no folder ${up}; make it in ${FILES} first`);
        return put(state, ctx.who.id, { open: at, buffer: [""], cursor: { line: 0, col: 0 }, dirty: true });
      },
    },
    edit: {
      line: `Replace one line of your buffer, or append one at the line after the last; a line is at most ${LINE} characters.`,
      args: shape({ line, text: { type: "string", maxLength: LINE } }),
      run: (state, args, ctx) => {
        const was = session(state, ctx);
        const n = Number(args.line);
        const text = String(args.text);
        if (n > was.buffer.length)
          return ctx.refuse(
            `the buffer has ${was.buffer.length} lines; edit one of them or append at ${was.buffer.length}`,
          );
        if (text.includes("\n")) return ctx.refuse("a line holds no line break; edit the next line instead");
        const buffer =
          n === was.buffer.length ? [...was.buffer, text] : was.buffer.map((one, i) => (i === n ? text : one));
        if (buffer.join("\n").length > TEXT) return ctx.refuse(`a file holds at most ${TEXT} characters`);
        return put(state, ctx.who.id, { ...was, buffer, cursor: { line: n, col: text.length }, dirty: true });
      },
    },
    cursor: {
      line: "Move your cursor to a line and a column of the buffer.",
      args: shape({ line, col: { type: "integer", minimum: 0, maximum: LINE } }),
      run: (state, args, ctx) => {
        const was = session(state, ctx);
        const n = Number(args.line);
        const col = Number(args.col);
        if (n >= was.buffer.length) return ctx.refuse(`the buffer has ${was.buffer.length} lines, numbered from 0`);
        if (col > was.buffer[n]!.length) return ctx.refuse(`line ${n} has ${was.buffer[n]!.length} characters`);
        return put(state, ctx.who.id, { ...was, cursor: { line: n, col } });
      },
    },
    save: {
      line: `Save your buffer to its file in the ${FILES} app.`,
      args: shape({}),
      run: (state, _args, ctx) => {
        const was = session(state, ctx);
        store(ctx);
        ctx.wish(act(`${FILES}.write`, { path: was.open!, text: was.buffer.join("\n") } as Obj));
        return put(state, ctx.who.id, { ...was, dirty: false });
      },
    },
    close: {
      line: "Close your buffer; unsaved lines are lost.",
      args: shape({}),
      run: (state, _args, ctx) => {
        session(state, ctx);
        return put(state, ctx.who.id, NONE);
      },
    },
  },
  moves: (state) =>
    Object.values(state.sessions).some((one) => one.open)
      ? ["open", "new", "edit", "cursor", "save", "close"]
      : ["open", "new"],
  read: sliced((state: Code, me: string) => state.sessions[me] ?? NONE),
  bot,
});
