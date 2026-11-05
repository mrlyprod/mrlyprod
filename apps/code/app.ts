import type { Ctx, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { act, app, sliced } from "@mrlyprod/os/contract/index.ts";

export type Cursor = { line: number; col: number };

export type Session = { open: string | null; buffer: string[]; cursor: Cursor; dirty: boolean };

export type Code = { sessions: Record<string, Session> };

type Entry = { kind: "file" | "folder"; text: string };

export const FILES = "files";

const KNOBS = {
  path: { line: "Characters one path holds.", schema: { type: "integer", minimum: 20, maximum: 2000 }, reality: 200 },
  line: { line: "Characters one line holds.", schema: { type: "integer", minimum: 20, maximum: 2000 }, reality: 200 },
  lines: { line: "Lines one buffer holds.", schema: { type: "integer", minimum: 10, maximum: 10000 }, reality: 500 },
  text: {
    line: "Characters one buffer holds.",
    schema: { type: "integer", minimum: 100, maximum: 100000 },
    reality: 4000,
  },
} satisfies Knobs;

const SEGMENT = /^[A-Za-z0-9._ -]+$/;

const NONE: Session = { open: null, buffer: [], cursor: { line: 0, col: 0 }, dirty: false };

const filing = (knob: Knob, name: string, reality: number) =>
  Number(knob<number | null>(`${FILES}.${name}`) ?? reality);

const room = (knob: Knob) => Math.min(knob("text"), filing(knob, "text", knob("text")));

const store = (ctx: Ctx): Record<string, Entry> => {
  const got = ctx.read(`${FILES}/stores/${ctx.who.id}`);
  if (got === null) {
    if (ctx.read(FILES) === null) ctx.refuse(`this world holds no ${FILES} app; code edits the files app's store`);
    return {};
  }
  return got as Record<string, Entry>;
};

const parent = (path: string) => path.slice(0, path.lastIndexOf("/")) || "/";

function clean(path: string, ctx: Ctx): string {
  const { refuse, knob } = ctx;
  const name = filing(knob, "name", 40);
  const depth = filing(knob, "depth", 8);
  if (!path.startsWith("/") || path === "/")
    return refuse(`a path starts with / and names a file in your store, like /src/main.ts, not ${path}`);
  const parts = path.slice(1).split("/");
  if (parts.some((part) => part === "." || part === ".."))
    return refuse("a path stays inside your store; . and .. are not names");
  if (parts.length > depth) return refuse(`a path is at most ${depth} names deep`);
  for (const part of parts)
    if (!part || part.length > name || !SEGMENT.test(part) || part.trim() !== part)
      return refuse(`each name is 1 to ${name} letters, digits, spaces, dots, hyphens or underscores, not "${part}"`);
  return path;
}

function saveable(at: string, text: string, ctx: Ctx) {
  const files = store(ctx);
  const up = parent(at);
  const most = filing(ctx.knob, "entries", 200);
  const limit = filing(ctx.knob, "text", ctx.knob("text"));
  if (text.length > limit) ctx.refuse(`a file holds at most ${limit} characters; cut ${text.length - limit}`);
  if (files[at]?.kind === "folder") ctx.refuse(`${at} is a folder now; close this and start a new file`);
  if (up !== "/" && files[up]?.kind !== "folder") ctx.refuse(`no folder ${up} any more; make it in ${FILES} first`);
  if (!files[at] && Object.keys(files).length >= most)
    ctx.refuse(`your store holds at most ${most} files and folders; delete one in ${FILES} first`);
}

const session = (state: Code, ctx: Ctx): Session => {
  const one = state.sessions[ctx.who.id];
  if (!one?.open) return ctx.refuse("open a file first");
  return one;
};

const put = (state: Code, me: string, one: Session): Code => ({ sessions: { ...state.sessions, [me]: one } });

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const path = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("path") });

const line = (knob: Knob): Schema => ({ type: "integer", minimum: 0, maximum: knob("lines") - 1 });

const open = (state: Code) => Object.values(state.sessions).filter((one) => one.open).length;

export default app<Code>({
  id: "code",
  title: "Code",
  line: "The editor: open a file of your store, edit it line by line with a cursor, save it back. Your buffer is yours alone.",
  boot: () => ({ sessions: {} }),
  verbs: {
    open: {
      line: "Open a file of your store into your buffer; the cursor goes to its first line.",
      args: (knob) => shape({ path: path(knob) }),
      run: (state, args, ctx) => {
        const at = clean(String(args.path), ctx);
        const entry = store(ctx)[at];
        if (entry?.kind !== "file") return ctx.refuse(`no file ${at} in your store; code.new starts one`);
        ctx.say({ note: `Opened ${at}; read code for its lines`, path: at, lines: entry.text.split("\n").length });
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
      args: (knob) => shape({ path: path(knob) }),
      run: (state, args, ctx) => {
        const at = clean(String(args.path), ctx);
        const files = store(ctx);
        if (files[at]) return ctx.refuse(`${at} exists already; code.open opens it`);
        const up = parent(at);
        if (up !== "/" && files[up]?.kind !== "folder") return ctx.refuse(`no folder ${up}; make it in ${FILES} first`);
        ctx.say({ note: `Started ${at}; it lands in your store on save`, path: at });
        return put(state, ctx.who.id, { open: at, buffer: [""], cursor: { line: 0, col: 0 }, dirty: true });
      },
    },
    edit: {
      line: (knob) =>
        `Replace one line of your buffer, or append one at the line after the last; a line is at most ${knob("line")} characters.`,
      args: (knob) => shape({ line: line(knob), text: { type: "string", maxLength: knob("line") } }),
      run: (state, args, ctx) => {
        const was = session(state, ctx);
        const n = Number(args.line);
        const text = String(args.text);
        const most = room(ctx.knob);
        if (n > was.buffer.length)
          return ctx.refuse(
            `the buffer has ${was.buffer.length} lines; edit one of them or append at ${was.buffer.length}`,
          );
        if (text.includes("\n")) return ctx.refuse("a line holds no line break; edit the next line instead");
        const buffer =
          n === was.buffer.length ? [...was.buffer, text] : was.buffer.map((one, i) => (i === n ? text : one));
        if (buffer.join("\n").length > most) return ctx.refuse(`a file holds at most ${most} characters`);
        ctx.say({ note: `Line ${n} of ${was.open} written; save to keep it`, line: n, lines: buffer.length });
        return put(state, ctx.who.id, { ...was, buffer, cursor: { line: n, col: text.length }, dirty: true });
      },
    },
    cursor: {
      line: "Move your cursor to a line and a column of the buffer.",
      args: (knob) => shape({ line: line(knob), col: { type: "integer", minimum: 0, maximum: knob("line") } }),
      run: (state, args, ctx) => {
        const was = session(state, ctx);
        const n = Number(args.line);
        const col = Number(args.col);
        if (n >= was.buffer.length) return ctx.refuse(`the buffer has ${was.buffer.length} lines, numbered from 0`);
        if (col > was.buffer[n]!.length) return ctx.refuse(`line ${n} has ${was.buffer[n]!.length} characters`);
        ctx.say({ note: `Cursor at line ${n}, column ${col}`, line: n, col });
        return put(state, ctx.who.id, { ...was, cursor: { line: n, col } });
      },
    },
    save: {
      line: `Save your buffer to its file in the ${FILES} app.`,
      args: shape({}),
      run: (state, _args, ctx) => {
        const was = session(state, ctx);
        const text = was.buffer.join("\n");
        saveable(was.open!, text, ctx);
        ctx.wish(act(`${FILES}.write`, { path: was.open!, text } as Obj));
        ctx.say({ note: `Saved ${was.open} to ${FILES}`, path: was.open! });
        return put(state, ctx.who.id, { ...was, dirty: false });
      },
    },
    close: {
      line: "Close your buffer; unsaved lines are lost.",
      args: shape({}),
      run: (state, _args, ctx) => {
        const was = session(state, ctx);
        ctx.say({ note: `Closed ${was.open}${was.dirty ? "; unsaved lines are lost" : ""}`, path: was.open! });
        return put(state, ctx.who.id, NONE);
      },
    },
  },
  moves: (state) => (open(state) ? ["open", "new", "edit", "cursor", "save", "close"] : ["open", "new"]),
  read: sliced(
    (state: Code, me: string) => state.sessions[me] ?? NONE,
    (state: Code, who: Who | null) => (who ? (state.sessions[who.id] ?? NONE) : { editing: open(state) }),
  ),
  knobs: KNOBS,
});
