import type { Ctx, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";

export type Note = { id: number; text: string; private: boolean; at: number };

export type Notes = { next: number; notes: Record<string, Note[]> };

export const TEXT = 4000;

export const KEEP = 100;

const LATEST = 5;

const noteOf = (state: Notes, id: number, me: string, refuse: Ctx["refuse"]): Note => {
  const owner = Object.keys(state.notes).find((one) => state.notes[one]!.some((note) => note.id === id));
  if (!owner) return refuse(`no note ${id}; it was deleted`);
  if (owner !== me) return refuse(`note ${id} is not yours`);
  return state.notes[me]!.find((note) => note.id === id)!;
};

const put = (state: Notes, me: string, notes: Note[]): Notes => ({ ...state, notes: { ...state.notes, [me]: notes } });

const swap = (state: Notes, me: string, note: Note): Notes =>
  put(
    state,
    me,
    state.notes[me]!.map((one) => (one.id === note.id ? note : one)),
  );

const seen = (notes: Note[], id: string, who: Who | null) =>
  who?.id === id ? notes : notes.filter((note) => !note.private);

const mine = (state: Notes, id: string, who: Who | null) => ({
  notes: [...seen(state.notes[id] ?? [], id, who)].reverse(),
});

const yours = (state: Notes, id: string) => ({
  count: (state.notes[id] ?? []).length,
  latest: [...(state.notes[id] ?? [])].reverse().slice(0, LATEST),
  all: "notes/@me",
});

const open = (state: Notes, who: Who | null) => ({
  ...(who ? { yours: yours(state, who.id) } : {}),
  next: state.next,
  notes: Object.fromEntries(Object.entries(state.notes).map(([id, notes]) => [id, seen(notes, id, who)])),
});

const text: Schema = { type: "string", minLength: 1, maxLength: TEXT };

const number: Schema = { type: "integer", minimum: 1 };

export default app<Notes>({
  id: "notes",
  title: "Notes",
  line: "Plain-text notes per user, public unless marked private. Your own, private ones too: notes/@me.",
  boot: () => ({ next: 1, notes: {} }),
  verbs: {
    write: {
      line: `Write a note of 1 to ${TEXT} characters, public unless private is true; you keep at most ${KEEP}.`,
      args: {
        type: "object",
        properties: { text, private: { type: "boolean" } },
        required: ["text"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const body = String(args.text);
        const list = state.notes[me] ?? [];
        if (!body.trim()) return ctx.refuse("the note is only spaces; write something");
        if (list.length >= KEEP) return ctx.refuse(`you keep ${KEEP} notes; delete one first`);
        const note: Note = { id: state.next, text: body, private: args.private === true, at: ctx.at };
        ctx.say({
          note: `Wrote ${note.private ? "private" : "public"} note ${note.id}; notes/@me holds it`,
          id: note.id,
        });
        return { ...put(state, me, [...list, note]), next: state.next + 1 };
      },
    },
    edit: {
      line: "Replace the text of a note of yours.",
      args: {
        type: "object",
        properties: { id: number, text },
        required: ["id", "text"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const note = noteOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        const body = String(args.text);
        if (!body.trim()) return ctx.refuse("the note is only spaces; write something, or delete it");
        ctx.say({ note: `Changed note ${note.id}`, id: note.id });
        return swap(state, ctx.who.id, { ...note, text: body, at: ctx.at });
      },
    },
    mark: {
      line: "Make a note of yours private or public.",
      args: {
        type: "object",
        properties: { id: number, private: { type: "boolean" } },
        required: ["id", "private"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const note = noteOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        ctx.say({ note: `Note ${note.id} is ${args.private === true ? "private" : "public"}`, id: note.id });
        return swap(state, ctx.who.id, { ...note, private: args.private === true });
      },
    },
    delete: {
      line: "Delete a note of yours.",
      args: {
        type: "object",
        properties: { id: number },
        required: ["id"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const note = noteOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        ctx.say({ note: `Deleted note ${note.id}`, id: note.id });
        return put(
          state,
          ctx.who.id,
          state.notes[ctx.who.id]!.filter((one) => one.id !== note.id),
        );
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["write", "edit", "mark", "delete"] : ["write"]),
  read: sliced(mine, open, true),
});
