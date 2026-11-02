import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

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
};

export const SUBJECT = 100;

export const TEXT = 2000;

export const TO = 8;

export const KEEP = 100;

export const FOLDERS: Folder[] = ["inbox", "sent", "archive"];

const ID = 40;

/* MAILBOXES */

const reply = (subject: string) => (/^re: /i.test(subject) ? subject : `Re: ${subject}`).slice(0, SUBJECT);

const entryOf = (state: Mail, me: string, id: number, refuse: Ctx["refuse"]): Entry => {
  const entry = state.boxes[me]?.[id];
  if (!entry) return refuse(state.messages[id] ? `message ${id} is not in your mailbox` : `no message ${id}`);
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
  let next: Mail = { ...state, next: state.next + 1, messages: { ...state.messages, [message.id]: message } };
  const lay = (id: string, entry: Entry) => {
    const box: Record<string, Entry> = { ...next.boxes[id], [message.id]: entry };
    const keys = Object.keys(box)
      .map(Number)
      .sort((a, b) => a - b);
    const gone = keys.slice(0, Math.max(0, keys.length - KEEP));
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
    ctx.wish(notice(to, `New mail: ${message.subject}`));
  }
  return next;
}

const mine = (state: Mail, id: string) => {
  const box = state.boxes[id] ?? {};
  const ids = Object.keys(box)
    .map(Number)
    .sort((a, b) => b - a);
  const rows = (folder: Folder) =>
    ids.filter((key) => box[key]!.folder === folder).map((key) => ({ ...state.messages[key]!, read: box[key]!.read }));
  return {
    unread: ids.filter((key) => box[key]!.folder === "inbox" && !box[key]!.read).length,
    ...Object.fromEntries(FOLDERS.map((folder) => [folder, rows(folder)])),
  };
};

/* SCHEMAS */

const person = (state: Mail): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

const message = (state: Mail): Schema => ({ type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) });

const sending = (state: Mail): Schema => ({
  type: "object",
  properties: {
    to: { type: "array", items: person(state), minItems: 1, maxItems: TO },
    subject: { type: "string", minLength: 1, maxLength: SUBJECT },
    text: { type: "string", minLength: 1, maxLength: TEXT },
  },
  required: ["to", "subject", "text"],
  additionalProperties: false,
});

const replying = (state: Mail): Schema => ({
  type: "object",
  properties: { id: message(state), text: { type: "string", minLength: 1, maxLength: TEXT } },
  required: ["id", "text"],
  additionalProperties: false,
});

const marking = (state: Mail): Schema => ({
  type: "object",
  properties: { id: message(state), read: { type: "boolean" } },
  required: ["id", "read"],
  additionalProperties: false,
});

const naming = (state: Mail): Schema => ({
  type: "object",
  properties: { id: message(state) },
  required: ["id"],
  additionalProperties: false,
});

/* APP */

export default app<Mail>({
  id: "mail",
  title: "Mail",
  line: "Mail between users: threads, an inbox, sent and archive folders, and read state.",
  boot: ({ actors }) => ({ people: actors, next: 1, messages: {}, boxes: {} }),
  verbs: {
    send: {
      line: `Mail up to ${TO} people: a subject of 1 to ${SUBJECT} characters and a text of 1 to ${TEXT}; a mailbox keeps its last ${KEEP} messages.`,
      args: sending,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = [...new Set((args.to as string[]).map(String))];
        const subject = String(args.subject);
        const text = String(args.text);
        if (to.includes(me)) return ctx.refuse("you are among the recipients; mail goes to others");
        if (!subject.trim() || !text.trim()) return ctx.refuse("the subject or the text is only spaces; say something");
        const id = state.next;
        return post(state, { id, thread: id, from: me, to, subject, text, at: ctx.at }, ctx);
      },
    },
    reply: {
      line: "Answer a message in your mailbox, in its thread: to its sender, or to its recipients when you sent it.",
      args: replying,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.id);
        const text = String(args.text);
        const entry = entryOf(state, me, id, ctx.refuse);
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const was = state.messages[id]!;
        const to = was.from === me ? was.to : [was.from];
        const read = filed(state, me, id, { ...entry, read: true });
        const message = {
          id: state.next,
          thread: was.thread,
          from: me,
          to,
          subject: reply(was.subject),
          text,
          at: ctx.at,
        };
        return post(read, message, ctx);
      },
    },
    mark: {
      line: "Mark a message in your mailbox read or unread.",
      args: marking,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        const entry = entryOf(state, ctx.who.id, id, ctx.refuse);
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
        return filed(state, me, id, { ...entry, folder: state.messages[id]!.from === me ? "sent" : "inbox" });
      },
    },
    delete: {
      line: "Delete a message from your mailbox for good; the others keep their copies.",
      args: naming,
      run: (state, args, ctx) => {
        const id = Number(args.id);
        entryOf(state, ctx.who.id, id, ctx.refuse);
        return filed(state, ctx.who.id, id, null);
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["send", "reply", "mark", "archive", "restore", "delete"] : ["send"]),
  read: sliced(mine),
  bot,
});
