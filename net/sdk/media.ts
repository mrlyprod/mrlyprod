import type { App, Bot, Obj, Schema, Step } from "./system.ts";
import { app, below, pick, sliced } from "./system.ts";

/* TYPES */

export type Title = {
  id: number;
  title: string;
  by: string;
  minutes: number;
  about: string;
  at: number;
  plays: number;
};

export type Media = {
  next: number;
  titles: Record<string, Title>;
  library: Record<string, number[]>;
  queue: Record<string, number[]>;
  now: Record<string, number | null>;
  plays: Record<string, Record<string, number>>;
};

export type Shelf = { id: string; title: string; line: string; unit: string; titles: readonly string[] };

export const TITLE = 100;

export const ABOUT = 280;

export const MINUTES = 600;

export const LIBRARY = 50;

export const QUEUE = 20;

export const OWN = 30;

/* STATE */

const idOf = (args: Obj) => Number(args.title);

const played = (state: Media, me: string, id: number): Media => {
  const one = state.titles[id]!;
  const mine = state.plays[me] ?? {};
  return {
    ...state,
    titles: { ...state.titles, [id]: { ...one, plays: one.plays + 1 } },
    now: { ...state.now, [me]: id },
    plays: { ...state.plays, [me]: { ...mine, [id]: (mine[id] ?? 0) + 1 } },
  };
};

const row = (state: Media, me: string, id: number) => {
  const one = state.titles[id]!;
  return { id, title: one.title, by: one.by, minutes: one.minutes, plays: state.plays[me]?.[id] ?? 0 };
};

const mine = (state: Media, me: string) => {
  const now = state.now[me] ?? null;
  return {
    now: now === null ? null : row(state, me, now),
    queue: (state.queue[me] ?? []).map((id) => row(state, me, id)),
    library: (state.library[me] ?? []).map((id) => row(state, me, id)),
  };
};

/* SCHEMAS */

const naming = (state: Media): Schema => ({
  type: "object",
  properties: { title: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) } },
  required: ["title"],
  additionalProperties: false,
});

const publishing: Schema = {
  type: "object",
  properties: {
    title: { type: "string", minLength: 1, maxLength: TITLE },
    minutes: { type: "integer", minimum: 1, maximum: MINUTES },
    about: { type: "string", maxLength: ABOUT },
  },
  required: ["title", "minutes"],
  additionalProperties: false,
};

/* BOT */

const policy =
  (shelf: Shelf): Bot<Media> =>
  ({ state, me, draw }): Step => {
    const ids = Object.keys(state.titles).map(Number);
    const library = state.library[me] ?? [];
    const queue = state.queue[me] ?? [];
    const published = Object.values(state.titles).filter((title) => title.by === me).length;
    if (queue.length && below(draw, 2)) return { verb: "next", args: {} };
    if (!ids.length || (published < 3 && !below(draw, 5)))
      return { verb: "publish", args: { title: pick(draw, shelf.titles), minutes: 3 + below(draw, 58) } };
    const fresh = ids.filter((id) => !library.includes(id));
    const turn = below(draw, 6);
    if (turn === 0 && fresh.length && library.length < LIBRARY)
      return { verb: "add", args: { title: pick(draw, fresh) } };
    if (turn === 1 && queue.length < QUEUE) return { verb: "queue", args: { title: pick(draw, ids) } };
    if (turn === 5 && library.length > 5) return { verb: "remove", args: { title: pick(draw, library) } };
    if (turn < 5) return { verb: "play", args: { title: pick(draw, library.length ? library : ids) } };
    return null;
  };

/* APP */

export function media(shelf: Shelf): App {
  const { unit } = shelf;
  return app<Media>({
    id: shelf.id,
    title: shelf.title,
    line: shelf.line,
    boot: () => ({ next: 1, titles: {}, library: {}, queue: {}, now: {}, plays: {} }),
    verbs: {
      publish: {
        line: `Publish a ${unit} as metadata: a title of 1 to ${TITLE} characters, its minutes and a line about it; you publish at most ${OWN}.`,
        args: publishing,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const title = String(args.title).trim();
          if (!title) return ctx.refuse("the title is only spaces; name it");
          if (Object.values(state.titles).filter((one) => one.by === me).length >= OWN)
            return ctx.refuse(`you published ${OWN} ${unit}s, the most one person may`);
          const id = state.next;
          const made: Title = {
            id,
            title,
            by: me,
            minutes: Number(args.minutes),
            about: String(args.about ?? ""),
            at: ctx.at,
            plays: 0,
          };
          return { ...state, next: id + 1, titles: { ...state.titles, [id]: made } };
        },
      },
      add: {
        line: `Add a ${unit} to your library; it keeps at most ${LIBRARY}.`,
        args: naming,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const id = idOf(args);
          const library = state.library[me] ?? [];
          if (library.includes(id)) return ctx.refuse(`${unit} ${id} is in your library already`);
          if (library.length >= LIBRARY) return ctx.refuse(`your library holds ${LIBRARY} ${unit}s; remove one first`);
          return { ...state, library: { ...state.library, [me]: [...library, id] } };
        },
      },
      remove: {
        line: `Remove a ${unit} from your library.`,
        args: naming,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const id = idOf(args);
          const library = state.library[me] ?? [];
          if (!library.includes(id)) return ctx.refuse(`${unit} ${id} is not in your library`);
          return { ...state, library: { ...state.library, [me]: library.filter((one) => one !== id) } };
        },
      },
      play: {
        line: `Play a ${unit}: it becomes what you play now and counts one play.`,
        args: naming,
        run: (state, args, ctx) => played(state, ctx.who.id, idOf(args)),
      },
      queue: {
        line: `Queue a ${unit} to play later; your queue holds at most ${QUEUE}.`,
        args: naming,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const queue = state.queue[me] ?? [];
          if (queue.length >= QUEUE) return ctx.refuse(`your queue holds ${QUEUE} ${unit}s; play the next first`);
          return { ...state, queue: { ...state.queue, [me]: [...queue, idOf(args)] } };
        },
      },
      next: {
        line: `Play the first ${unit} of your queue.`,
        run: (state, _args, ctx) => {
          const me = ctx.who.id;
          const [first, ...rest] = state.queue[me] ?? [];
          if (first === undefined) return ctx.refuse(`your queue is empty; queue a ${unit} first`);
          return played({ ...state, queue: { ...state.queue, [me]: rest } }, me, first);
        },
      },
    },
    moves: (state) => (state.next > 1 ? ["publish", "add", "remove", "play", "queue", "next"] : ["publish"]),
    read: sliced(mine),
    bot: policy(shelf),
  });
}
