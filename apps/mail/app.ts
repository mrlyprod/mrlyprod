import type { Ctx, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, SLICE, sliced } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, reach, someone } from "../shared/people.ts";

export type Message = {
  id: number;
  thread: number;
  from: string;
  to: string[];
  subject: string;
  text: string;
  at: number;
};

export type Folder = "inbox" | "sent" | "archive";

export type Entry = { folder: Folder; read: boolean };

export type Mail = {
  people: string[] | null;
  next: number;
  messages: Record<string, Message>;
  boxes: Record<string, Record<string, Entry>>;
  open: Record<string, number>;
};

const KNOBS = {
  subject: {
    line: "Characters in a mail's subject.",
    schema: { type: "integer", minimum: 8, maximum: 1000 },
    reality: 100,
  },
  text: {
    line: "Characters in a mail's text.",
    schema: { type: "integer", minimum: 20, maximum: 100_000 },
    reality: 2000,
  },
  to: {
    line: "People one mail can go to.",
    schema: { type: "integer", minimum: 1, maximum: 256 },
    reality: 8,
  },
  keep: {
    line: "Messages a mailbox keeps.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 100,
  },
} satisfies Knobs;

export const FOLDERS: Folder[] = ["inbox", "sent", "archive"];

const ID = 40;

const SHOWN = 4;

const PREVIEW = 60;

const clip = (text: string, size: number) => (text.length > size ? `${text.slice(0, size - 3)}...` : text);

const reply = (subject: string, size: number) => clip(/^re: /i.test(subject) ? subject : `Re: ${subject}`, size);

const entryOf = (state: Mail, me: string, id: number, refuse: Ctx["refuse"]): Entry => {
  const entry = state.boxes[me]?.[id];
  if (!entry) return refuse(`message ${id} is not in your mailbox; read mail lists yours`);
  return entry;
};

const filed = (state: Mail, me: string, id: number, entry: Entry | null): Mail => {
  const box = { ...state.boxes[me] };
  if (entry) box[id] = entry;
  else delete box[id];
  const boxes = { ...state.boxes, [me]: box };
  const messages = { ...state.messages };
  if (!entry && !Object.values(boxes).some((one) => one[id])) delete messages[id];
  return { ...state, boxes, messages };
};

function post(state: Mail, message: Message, ctx: Ctx): Mail {
  const keep = ctx.knob("keep");
  let next: Mail = { ...state, next: state.next + 1, messages: { ...state.messages, [message.id]: message } };
  const lay = (id: string, entry: Entry) => {
    const box: Record<string, Entry> = { ...next.boxes[id], [message.id]: entry };
    const keys = Object.keys(box)
      .map(Number)
      .sort((a, b) => a - b);
    const gone = keys.slice(0, Math.max(0, keys.length - keep));
    for (const old of gone) delete box[old];
    next = { ...next, boxes: { ...next.boxes, [id]: box } };
    for (const old of gone)
      if (!Object.values(next.boxes).some((one) => one[old])) {
        const messages = { ...next.messages };
        delete messages[old];
        next = { ...next, messages };
      }
  };
  lay(message.from, { folder: "sent", read: true });
  for (const to of message.to) {
    lay(to, { folder: "inbox", read: false });
    ctx.wish(notice(to, `New mail ${message.id} from ${message.from}: ${message.subject}`));
  }
  ctx.say({
    note: `Mailed ${message.to.join(", ")}: message ${message.id}`,
    id: message.id,
    thread: message.thread,
    to: message.to,
  });
  return next;
}

const refuseBlockers = (people: string[], ctx: Ctx) => {
  for (const one of people) reach(ctx, one);
};

const list = (value: unknown) => [...new Set(((value as string[] | undefined) ?? []).map(String))];

const audience = (was: Message, me: string, all: boolean) => {
  if (all) return [...new Set([was.from, ...was.to])].filter((one) => one !== me);
  return was.from === me ? was.to : [was.from];
};

function repliedTo(was: Message, args: Obj, ctx: Ctx): string[] {
  const me = ctx.who.id;
  const added = list(args.add);
  if (added.includes(me)) return ctx.refuse("you are among those you added; mail goes to others");
  refuseBlockers(added, ctx);
  const named = audience(was, me, args.all === true);
  const open = named.filter((one) => !blockedBy(ctx, one, me) && !blockedBy(ctx, me, one));
  if (!open.length && !added.length) {
    refuseBlockers(named, ctx);
    return ctx.refuse("nobody is left to mail");
  }
  const to = [...new Set([...open, ...added])];
  const cap = ctx.knob("to");
  if (to.length > cap)
    return ctx.refuse(`this reply would reach ${to.length} people and the most is ${cap}; add fewer`);
  return to;
}

const row = (message: Message, read: boolean) => {
  const { id, thread, from, to, subject, text, at } = message;
  const preview = text.length > PREVIEW ? `${text.slice(0, PREVIEW - 3)}...` : text;
  return { id, thread, from, to, subject, preview, at, read };
};

const view = (state: Mail, id: string, deep: boolean) => {
  const box = state.boxes[id] ?? {};
  const ids = Object.keys(box)
    .map(Number)
    .sort((a, b) => b - a);
  const rows = (keys: number[]) => keys.map((key) => row(state.messages[key]!, box[key]!.read));
  const inFolder = (folder: Folder) => ids.filter((key) => box[key]!.folder === folder);
  const unread = ids.filter((key) => box[key]!.folder === "inbox" && !box[key]!.read).length;
  const open = state.open[id];
  const reading = open !== undefined && box[open] ? state.messages[open]! : null;
  if (!deep)
    return {
      unread,
      open: reading,
      ...Object.fromEntries(FOLDERS.map((folder) => [folder, rows(inFolder(folder).slice(0, SHOWN))])),
      more: "mail/inbox, mail/messages/<id>, mail/threads/<thread>",
    };
  const threads = [...new Set(ids.map((key) => state.messages[key]!.thread))];
  return {
    unread,
    open: reading,
    ...Object.fromEntries(FOLDERS.map((folder) => [folder, rows(inFolder(folder))])),
    messages: Object.fromEntries(ids.map((key) => [key, state.messages[key]!])),
    threads: Object.fromEntries(
      threads.map((thread) => [
        thread,
        ids.filter((key) => state.messages[key]!.thread === thread).map((key) => state.messages[key]!),
      ]),
    ),
  };
};

const totals = (state: Mail) => ({
  mailboxes: Object.keys(state.boxes).length,
  messages: Object.keys(state.messages).length,
});

const deepens = (path: string[]) => path.length > (path[0]?.startsWith(SLICE) ? 1 : 0);

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const message: Schema = { type: "integer", minimum: 1 };

const body = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("text") });

const people = (knob: Knob, minItems: number): Schema => ({
  type: "array",
  items: person,
  minItems,
  maxItems: knob("to"),
});

const sending = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    to: people(knob, 1),
    subject: { type: "string", minLength: 1, maxLength: knob("subject") },
    text: body(knob),
  },
  required: ["to", "subject", "text"],
  additionalProperties: false,
});

const replying = (knob: Knob): Schema => ({
  type: "object",
  properties: { id: message, text: body(knob), all: { type: "boolean" }, add: people(knob, 0) },
  required: ["id", "text"],
  additionalProperties: false,
});

const forwarding = (knob: Knob): Schema => ({
  type: "object",
  properties: { id: message, to: people(knob, 1), text: body(knob) },
  required: ["id", "to"],
  additionalProperties: false,
});

const marking: Schema = {
  type: "object",
  properties: { id: message, read: { type: "boolean" } },
  required: ["id", "read"],
  additionalProperties: false,
};

const naming: Schema = {
  type: "object",
  properties: { id: message },
  required: ["id"],
  additionalProperties: false,
};

export default app<Mail>({
  id: "mail",
  title: "Mail",
  line: "Mail between users: threads, reply-all, forward, an inbox, sent and archive folders, and read state.",
  boot: ({ actors }) => ({ people: actors, next: 1, messages: {}, boxes: {}, open: {} }),
  knobs: KNOBS,
  verbs: {
    send: {
      line: (knob) =>
        `Mail up to ${knob("to")} people: a subject of 1 to ${knob("subject")} characters and a text of 1 to ${knob("text")}; a mailbox keeps its last ${knob("keep")} messages.`,
      args: sending,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = list(args.to).map((one) => someone(state.people, one, ctx));
        const subject = String(args.subject);
        const text = String(args.text);
        if (to.includes(me)) return ctx.refuse("you are among the recipients; mail goes to others");
        if (!subject.trim() || !text.trim()) return ctx.refuse("the subject or the text is only spaces; say something");
        refuseBlockers(to, ctx);
        const id = state.next;
        return post(state, { id, thread: id, from: me, to, subject, text, at: ctx.at }, ctx);
      },
    },
    reply: {
      line: "Answer a message in your mailbox, in its thread: to its sender, or to its recipients when you sent it; all sends to everyone on it, add brings in more people.",
      args: replying,
      run: (state, args, ctx) => {
        for (const one of list(args.add)) someone(state.people, one, ctx);
        const me = ctx.who.id;
        const id = Number(args.id);
        const text = String(args.text);
        const entry = entryOf(state, me, id, ctx.refuse);
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const was = state.messages[id]!;
        const to = repliedTo(was, args, ctx);
        const read = filed(state, me, id, { ...entry, read: true });
        const message = {
          id: state.next,
          thread: was.thread,
          from: me,
          to,
          subject: reply(was.subject, ctx.knob("subject")),
          text,
          at: ctx.at,
        };
        return post(read, message, ctx);
      },
    },
    forward: {
      line: "Pass a message in your mailbox on to others, as a new thread, with an optional note above it.",
      args: forwarding,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = state.next;
        const was = state.messages[Number(args.id)];
        entryOf(state, me, Number(args.id), ctx.refuse);
        const to = list(args.to).map((one) => someone(state.people, one, ctx));
        if (to.includes(me)) return ctx.refuse("you are among the recipients; mail goes to others");
        refuseBlockers(to, ctx);
        const note = args.text === undefined ? "" : `${String(args.text)}\n\n`;
        const text = clip(`${note}From ${was!.from}: ${was!.text}`, ctx.knob("text"));
        const subject = clip(`Fwd: ${was!.subject}`, ctx.knob("subject"));
        return post(state, { id, thread: id, from: me, to, subject, text, at: ctx.at }, ctx);
      },
    },
    open: {
      line: "Open a message in your mailbox: it is marked read and the reply holds it in full.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.id);
        const entry = entryOf(state, me, id, ctx.refuse);
        ctx.say({ note: `Message ${id}`, ...state.messages[id]! });
        return { ...filed(state, me, id, { ...entry, read: true }), open: { ...state.open, [me]: id } };
      },
    },
    mark: {
      line: "Mark a message in your mailbox read or unread.",
      args: marking,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const entry = entryOf(state, ctx.who.id, id, ctx.refuse);
        ctx.say({ note: `Message ${id} is ${args.read ? "read" : "unread"}`, id });
        return filed(state, ctx.who.id, id, { ...entry, read: Boolean(args.read) });
      },
    },
    archive: {
      line: "Move a message of your inbox or sent folder to the archive.",
      args: naming,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const entry = entryOf(state, ctx.who.id, id, ctx.refuse);
        if (entry.folder === "archive") return ctx.refuse(`message ${id} is already in the archive`);
        ctx.say({ note: `Archived message ${id}`, id });
        return filed(state, ctx.who.id, id, { folder: "archive", read: true });
      },
    },
    restore: {
      line: "Move an archived message back to the inbox, or to sent when you wrote it.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.id);
        const entry = entryOf(state, me, id, ctx.refuse);
        if (entry.folder !== "archive") return ctx.refuse(`message ${id} is in ${entry.folder}, not the archive`);
        const folder = state.messages[id]!.from === me ? "sent" : "inbox";
        ctx.say({ note: `Moved message ${id} to ${folder}`, id });
        return filed(state, me, id, { ...entry, folder });
      },
    },
    delete: {
      line: "Delete a message from your mailbox for good; the others keep their copies.",
      args: naming,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        entryOf(state, ctx.who.id, id, ctx.refuse);
        ctx.say({ note: `Deleted message ${id}`, id });
        return filed(state, ctx.who.id, id, null);
      },
    },
  },
  moves: (state) =>
    state.next > 1 ? ["send", "reply", "forward", "open", "mark", "archive", "restore", "delete"] : ["send"],
  read: (state, path, who, at) =>
    sliced(
      (one: Mail, id: string) => view(one, id, deepens(path)),
      (one: Mail, by: Who | null) => (by ? view(one, by.id, deepens(path)) : totals(one)),
    )(state, path, who, at),
});
