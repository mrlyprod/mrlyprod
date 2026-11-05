import type { App, Ctx, Json, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { notice, reach, someone } from "./people.ts";

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
  people: string[] | null;
  listed: number;
  next: number;
  titles: Record<string, Title>;
  library: Record<string, number[]>;
  queue: Record<string, number[]>;
  now: Record<string, number | null>;
  plays: Record<string, Record<string, number>>;
  follows: Record<string, string[]>;
};

export type Shelf = { id: string; title: string; line: string; unit: string };

const ID = 40;

export const KNOBS = {
  title: { line: "Characters one title holds.", schema: { type: "integer", minimum: 10, maximum: 1000 }, reality: 100 },
  about: {
    line: "Characters one about line holds.",
    schema: { type: "integer", minimum: 0, maximum: 5000 },
    reality: 280,
  },
  minutes: {
    line: "Longest running time in minutes.",
    schema: { type: "integer", minimum: 1, maximum: 10000 },
    reality: 600,
  },
  library: { line: "Titles one library keeps.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 50 },
  queue: { line: "Titles one queue holds.", schema: { type: "integer", minimum: 1, maximum: 500 }, reality: 20 },
  own: { line: "Titles one person may publish.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 30 },
  follows: {
    line: "People one person may follow here.",
    schema: { type: "integer", minimum: 1, maximum: 256 },
    reality: 50,
  },
  listed: {
    line: "Newest titles one overview lists.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
} satisfies Knobs;

const idOf = (args: Obj) => Number(args.title);

const titleOf = (state: Media, id: number, unit: string, refuse: Ctx["refuse"]): Title =>
  state.titles[id] ?? refuse(`${unit} ${id} was unpublished; pick another from the latest list`);

const heard = (state: Media, ctx: Ctx, one: Title) => {
  const me = ctx.who.id;
  const fresh = !(state.plays[me]?.[one.id] ?? 0);
  if (fresh && one.by !== me) ctx.wish(notice(one.by, `${me} played your ${one.title}`));
};

const played = (state: Media, me: string, id: number): Media => {
  const one = state.titles[id]!;
  const mine = state.plays[me] ?? {};
  return {
    ...state,
    titles: { ...state.titles, [id]: { ...one, plays: one.plays + (one.by === me ? 0 : 1) } },
    now: { ...state.now, [me]: id },
    plays: { ...state.plays, [me]: { ...mine, [id]: (mine[id] ?? 0) + 1 } },
  };
};

const listeners = (state: Media, id: number) =>
  Object.fromEntries(Object.entries(state.plays).flatMap(([who, each]) => (each[id] ? [[who, each[id]!]] : [])));

const row = (state: Media, me: string, id: number) => {
  const one = state.titles[id]!;
  return { id, title: one.title, by: one.by, minutes: one.minutes, plays: state.plays[me]?.[id] ?? 0 };
};

const open = (one: Title) => ({
  id: one.id,
  title: one.title,
  by: one.by,
  minutes: one.minutes,
  about: one.about,
  at: one.at,
  plays: one.plays,
});

const newest = (state: Media) =>
  Object.values(state.titles)
    .sort((a, b) => b.at - a.at || b.id - a.id)
    .slice(0, state.listed)
    .map(open);

const mine = (state: Media, me: string) => {
  const now = state.now[me] ?? null;
  return {
    now: now === null ? null : row(state, me, now),
    queue: (state.queue[me] ?? []).map((id) => row(state, me, id)),
    library: (state.library[me] ?? []).map((id) => row(state, me, id)),
    published: Object.values(state.titles)
      .filter((one) => one.by === me)
      .map((one) => ({ ...open(one), listeners: listeners(state, one.id) })),
    following: state.follows[me] ?? [],
    followers: Object.keys(state.follows).filter((who) => state.follows[who]!.includes(me)),
    latest: newest(state),
  };
};

const catalog = (state: Media) => Object.fromEntries(Object.values(state.titles).map((one) => [one.id, open(one)]));

const whole = (state: Media, who: Who | null): Json => ({
  ...(who ? mine(state, who.id) : { latest: newest(state) }),
  titles: catalog(state),
});

const deep = sliced<Media>(mine, whole);

const read = (state: Media, path: string[], who: Who | null, at = 0): Json | undefined => {
  if (path.length) return deep(state, path, who, at);
  return who ? mine(state, who.id) : { latest: newest(state), total: Object.keys(state.titles).length };
};

const titled = (more: Record<string, Schema> = {}, optional: string[] = []): Schema => ({
  type: "object",
  properties: { title: { type: "integer", minimum: 1 }, ...more },
  required: ["title", ...Object.keys(more).filter((key) => !optional.includes(key))],
  additionalProperties: false,
});

const naming: Schema = {
  type: "object",
  properties: { person: { type: "string", minLength: 1, maxLength: ID } },
  required: ["person"],
  additionalProperties: false,
};

const without = (lists: Record<string, number[]>, id: number) =>
  Object.fromEntries(Object.entries(lists).map(([who, list]) => [who, list.filter((one) => one !== id)]));

const gone = (state: Media, id: number): Media => {
  const { [id]: _, ...titles } = state.titles;
  return {
    ...state,
    titles,
    library: without(state.library, id),
    queue: without(state.queue, id),
    now: Object.fromEntries(Object.entries(state.now).map(([who, one]) => [who, one === id ? null : one])),
    plays: Object.fromEntries(
      Object.entries(state.plays).map(([who, each]) => {
        const { [id]: _played, ...rest } = each;
        return [who, rest];
      }),
    ),
  };
};

export function media(shelf: Shelf): App {
  const { unit } = shelf;
  const owned = (state: Media, id: number, ctx: Ctx): Title => {
    const one = titleOf(state, id, unit, ctx.refuse);
    return one.by === ctx.who.id ? one : ctx.refuse(`${unit} ${id} is ${one.by}'s; only its publisher may`);
  };
  return app<Media>({
    id: shelf.id,
    title: shelf.title,
    line: `${shelf.line} Everyone sees what is published; your library, queue and plays are yours, and a publisher sees who played what. Read ${shelf.id}/titles for all that is out.`,
    boot: ({ actors, knob }) => ({
      people: actors,
      listed: knob("listed"),
      next: 1,
      titles: {},
      library: {},
      queue: {},
      now: {},
      plays: {},
      follows: {},
    }),
    verbs: {
      publish: {
        line: (knob) =>
          `Publish a ${unit} as metadata: a title of 1 to ${knob("title")} characters, its minutes and a line about it; you publish at most ${knob("own")}, and your followers hear of it.`,
        args: (knob) => ({
          type: "object",
          properties: {
            title: { type: "string", minLength: 1, maxLength: knob("title") },
            minutes: { type: "integer", minimum: 1, maximum: knob("minutes") },
            about: { type: "string", maxLength: knob("about") },
          },
          required: ["title", "minutes"],
          additionalProperties: false,
        }),
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const title = String(args.title).trim();
          const most = ctx.knob("own");
          if (!title) return ctx.refuse("the title is only spaces; name it");
          if (Object.values(state.titles).filter((one) => one.by === me).length >= most)
            return ctx.refuse(`you published ${most} ${unit}s, the most one person may; unpublish one first`);
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
          for (const who of Object.keys(state.follows))
            if (state.follows[who]!.includes(me)) ctx.wish(notice(who, `${me} published ${unit} ${id}: ${title}`));
          ctx.say({ note: `Published ${unit} ${id}: ${title}`, id });
          return { ...state, next: id + 1, titles: { ...state.titles, [id]: made } };
        },
      },
      edit: {
        line: `Change a ${unit} you published: any of its name, minutes and about.`,
        args: (knob) =>
          titled(
            {
              name: { type: "string", minLength: 1, maxLength: knob("title") },
              minutes: { type: "integer", minimum: 1, maximum: knob("minutes") },
              about: { type: "string", maxLength: knob("about") },
            },
            ["name", "minutes", "about"],
          ),
        run: (state, args, ctx) => {
          const one = owned(state, idOf(args), ctx);
          const name = args.name === undefined ? one.title : String(args.name).trim();
          if (!name) return ctx.refuse("the name is only spaces; name it");
          if (args.name === undefined && args.minutes === undefined && args.about === undefined)
            return ctx.refuse("say what changes: a name, minutes or about");
          const next: Title = {
            ...one,
            title: name,
            minutes: args.minutes === undefined ? one.minutes : Number(args.minutes),
            about: args.about === undefined ? one.about : String(args.about),
          };
          ctx.say({ note: `Changed ${unit} ${one.id}: ${next.title}`, id: one.id });
          return { ...state, titles: { ...state.titles, [one.id]: next } };
        },
      },
      unpublish: {
        line: `Take a ${unit} you published down: it leaves every library and queue, and whoever kept it is told.`,
        args: titled(),
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const one = owned(state, idOf(args), ctx);
          for (const who of Object.keys(state.library))
            if (who !== me && state.library[who]!.includes(one.id))
              ctx.wish(notice(who, `${me} unpublished ${one.title}; it left your library`));
          ctx.say({ note: `Unpublished ${unit} ${one.id}: ${one.title}`, id: one.id });
          return gone(state, one.id);
        },
      },
      add: {
        line: (knob) => `Add a ${unit} to your library; it keeps at most ${knob("library")}.`,
        args: titled(),
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const id = idOf(args);
          const library = state.library[me] ?? [];
          const most = ctx.knob("library");
          titleOf(state, id, unit, ctx.refuse);
          if (library.includes(id)) return ctx.refuse(`${unit} ${id} is in your library already`);
          if (library.length >= most) return ctx.refuse(`your library holds ${most} ${unit}s; remove one first`);
          ctx.say({ note: `Added ${unit} ${id} to your library`, id });
          return { ...state, library: { ...state.library, [me]: [...library, id] } };
        },
      },
      remove: {
        line: `Remove a ${unit} from your library.`,
        args: titled(),
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const id = idOf(args);
          const library = state.library[me] ?? [];
          if (!library.includes(id)) return ctx.refuse(`${unit} ${id} is not in your library`);
          ctx.say({ note: `Removed ${unit} ${id} from your library`, id });
          return { ...state, library: { ...state.library, [me]: library.filter((one) => one !== id) } };
        },
      },
      play: {
        line: `Play a ${unit}: it becomes what you play now and counts one play, unless you published it; its publisher sees that you did and hears of your first play.`,
        args: titled(),
        run: (state, args, ctx) => {
          const one = titleOf(state, idOf(args), unit, ctx.refuse);
          heard(state, ctx, one);
          ctx.say({ note: `Playing ${unit} ${one.id}: ${one.title}`, id: one.id });
          return played(state, ctx.who.id, one.id);
        },
      },
      queue: {
        line: (knob) => `Queue a ${unit} to play later; your queue holds at most ${knob("queue")}.`,
        args: titled(),
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const queue = state.queue[me] ?? [];
          const most = ctx.knob("queue");
          titleOf(state, idOf(args), unit, ctx.refuse);
          if (queue.length >= most) return ctx.refuse(`your queue holds ${most} ${unit}s; play the next first`);
          ctx.say({ note: `Queued ${unit} ${idOf(args)}; ${queue.length + 1} in your queue`, id: idOf(args) });
          return { ...state, queue: { ...state.queue, [me]: [...queue, idOf(args)] } };
        },
      },
      next: {
        line: `Play the first ${unit} of your queue.`,
        run: (state, _args, ctx) => {
          const me = ctx.who.id;
          const [first, ...rest] = state.queue[me] ?? [];
          if (first === undefined) return ctx.refuse(`your queue is empty; queue a ${unit} first`);
          heard(state, ctx, state.titles[first]!);
          ctx.say({ note: `Playing ${unit} ${first}: ${state.titles[first]!.title}`, id: first });
          return played({ ...state, queue: { ...state.queue, [me]: rest } }, me, first);
        },
      },
      follow: {
        line: (knob) =>
          `Follow someone: you hear when they publish a ${unit}. They are told. You follow at most ${knob("follows")}.`,
        args: naming,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const who = someone(state.people, String(args.person), ctx);
          const list = state.follows[me] ?? [];
          const most = ctx.knob("follows");
          if (who === me) return ctx.refuse("you hear yourself already; follow someone else");
          reach(ctx, who);
          if (list.includes(who)) return ctx.refuse(`you follow ${who} already`);
          if (list.length >= most) return ctx.refuse(`you follow ${most} people; unfollow someone first`);
          ctx.wish(notice(who, `${me} follows your ${unit}s`));
          ctx.say({ note: `You follow ${who}`, id: who });
          return { ...state, follows: { ...state.follows, [me]: [...list, who] } };
        },
      },
      unfollow: {
        line: "Stop following someone.",
        args: naming,
        run: (state, args, ctx) => {
          const me = ctx.who.id;
          const who = String(args.person);
          const list = state.follows[me] ?? [];
          if (!list.includes(who)) return ctx.refuse(`you do not follow ${who}`);
          ctx.say({ note: `You no longer follow ${who}`, id: who });
          return { ...state, follows: { ...state.follows, [me]: list.filter((one) => one !== who) } };
        },
      },
    },
    moves: (state) =>
      state.next > 1
        ? ["publish", "edit", "unpublish", "add", "remove", "play", "queue", "next", "follow", "unfollow"]
        : ["publish", "follow", "unfollow"],
    read,
    knobs: KNOBS,
  });
}
