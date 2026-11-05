import { blockedBy, NOTICE, notice, someone } from "@mrlyprod/apps/shared/people.ts";
import type { Ctx, Json, Knob, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, sliced, stream, walk } from "@mrlyprod/os/contract/index.ts";

export type Terrain = "water" | "sand" | "grass" | "forest" | "hill" | "mountain";

export type Spot = { q: number; r: number };

export type Place = { id: number; q: number; r: number; name: string; by: string; at: number; shared: boolean };

export type Post = { id: number; by: string; text: string; at: number };

export type Said = { by: string; text: string; at: number };

export type Country = { id: number; name: string; capital: Spot };

export type Maps = {
  seed: number;
  radius: number;
  cells: Record<string, Terrain>;
  next: number;
  places: Record<string, Place>;
  people: string[] | null;
  where: Record<string, Spot>;
  walking: Record<string, number>;
  rooms: Record<string, Said[]>;
  boards: Record<string, Post[]>;
  posted: number;
  keeps: number;
  countries: Record<string, Country>;
  spawn: string;
  city: number;
  near: number;
  own: string;
};

const KNOBS = {
  radius: {
    line: "Cells from the centre of the map to its edge.",
    schema: { type: "integer", minimum: 1, maximum: 32 },
    reality: 8,
  },
  shore: {
    line: "How far the sea creeps in from the edge of the map; 0 is all land.",
    schema: { type: "integer", minimum: 0, maximum: 20 },
    reality: 4,
  },
  land: {
    line: "The mix of terrain on land.",
    schema: { enum: ["mixed", "green", "rugged", "sandy", "wooded"] },
    reality: "mixed",
  },
  countries: {
    line: "Seeded named countries the map is divided into.",
    schema: { type: "integer", minimum: 1, maximum: 64 },
    reality: 4,
  },
  borders: {
    line: "What is a country for? none: no countries at all. open: anyone crosses. closed: nobody goes from one to another.",
    schema: { enum: ["none", "open", "closed"] },
    reality: "open",
  },
  own: {
    line: "Who owns a place? private: only its pinner changes it. shared: anyone does. none: nobody pins at all.",
    schema: { enum: ["private", "shared", "none"] },
    reality: "private",
  },
  city: {
    line: "How big should a city be? The most people one cell holds; 1 means everyone lives alone.",
    schema: { type: "integer", minimum: 1, maximum: 4096 },
    reality: 4096,
  },
  spawn: {
    line: "Where people start: scattered over the land, or all in one place.",
    schema: { enum: ["scattered", "together"] },
    reality: "scattered",
  },
  walk: {
    line: "World milliseconds walking takes per cell; 0 means you arrive at once.",
    schema: { type: "integer", minimum: 0, maximum: 3_600_000 },
    reality: 0,
  },
  swim: {
    line: "Whether people may stand on water cells.",
    schema: { type: "boolean" },
    reality: true,
  },
  greet: {
    line: "Whether those present get a notice when someone arrives on their cell.",
    schema: { type: "boolean" },
    reality: true,
  },
  near: {
    line: "Cells away that still count as near.",
    schema: { type: "integer", minimum: 1, maximum: 16 },
    reality: 3,
  },
  history: {
    line: "Lines of talk one cell keeps.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 50,
  },
  text: {
    line: "Characters in one thing said on a cell.",
    schema: { type: "integer", minimum: 16, maximum: 5000 },
    reality: 500,
  },
  name: {
    line: "Characters in a place's name.",
    schema: { type: "integer", minimum: 8, maximum: 200 },
    reality: 40,
  },
  pins: {
    line: "Places one person pins.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  crowd: {
    line: "Places one cell holds.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 1000,
  },
  squares: {
    line: "Whether a place may be pinned as shared, so anyone standing there posts on its noticeboard.",
    schema: { type: "boolean" },
    reality: true,
  },
  posts: {
    line: "Posts one noticeboard keeps; 0 means no place has a noticeboard.",
    schema: { type: "integer", minimum: 0, maximum: 1000 },
    reality: 20,
  },
  board: {
    line: "Characters in one post on a noticeboard.",
    schema: { type: "integer", minimum: 16, maximum: 5000 },
    reality: 280,
  },
} satisfies Knobs;

const SHOWN = 8;

const BOARD = 5;

const PINNED = 3;

const ALSO = 3;

const ROOM = 3;

const CLIP = 120;

const LANDS: Record<string, Terrain[]> = {
  mixed: ["mountain", "hill", "hill", "forest", "forest", "forest", "grass", "grass", "grass", "grass", "sand"],
  green: ["hill", "forest", "forest", "grass", "grass", "grass", "grass", "grass", "grass", "grass", "sand"],
  rugged: ["mountain", "mountain", "mountain", "hill", "hill", "hill", "hill", "forest", "forest", "grass", "sand"],
  sandy: ["hill", "grass", "grass", "sand", "sand", "sand", "sand", "sand", "sand", "sand", "sand"],
  wooded: ["mountain", "hill", "forest", "forest", "forest", "forest", "forest", "forest", "forest", "grass", "sand"],
};

const HEADS = [
  "Ar",
  "Bel",
  "Cor",
  "Dun",
  "El",
  "Far",
  "Gal",
  "Har",
  "Is",
  "Jor",
  "Kal",
  "Lor",
  "Mar",
  "Nor",
  "Os",
  "Pel",
  "Quin",
  "Ros",
  "Sol",
  "Tal",
  "Ul",
  "Ver",
  "Wyn",
  "Zan",
];

const TAILS = ["ania", "avia", "eth", "ia", "land", "mark", "mere", "ora", "stan", "via", "wick", "ynn"];

export const key = (q: number, r: number) => `${q},${r}`;

export const inside = (q: number, r: number, radius: number) =>
  Math.abs(q) <= radius && Math.abs(r) <= radius && Math.abs(q + r) <= radius;

export const away = (a: Spot, b: Spot) =>
  (Math.abs(a.q - b.q) + Math.abs(a.r - b.r) + Math.abs(a.q + a.r - b.q - b.r)) / 2;

const grid = (radius: number): Spot[] => {
  const out: Spot[] = [];
  for (let q = -radius; q <= radius; q++)
    for (let r = -radius; r <= radius; r++) if (inside(q, r, radius)) out.push({ q, r });
  return out;
};

const drawing = (seed: number, word: string) => {
  const salt = [...word].reduce((s, c) => Math.imul(s ^ c.charCodeAt(0), 16777619) >>> 0, seed >>> 0);
  const { draw } = stream(salt);
  draw();
  draw();
  return draw;
};

export function cells(seed: number, radius: number, shore: number, land: string): Record<string, Terrain> {
  const { draw } = stream(seed);
  const mix = LANDS[land]!;
  const out: Record<string, Terrain> = {};
  for (const { q, r } of grid(radius)) {
    const far = Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
    out[key(q, r)] = mix[below(draw, 10) + Math.floor((far * shore) / radius)] ?? "water";
  }
  return out;
}

function partition(seed: number, radius: number, terrain: Record<string, Terrain>, want: number) {
  const draw = drawing(seed, "countries");
  const all = grid(radius);
  const dry = all.filter((one) => terrain[key(one.q, one.r)] !== "water");
  const pool = dry.length >= want ? dry : all;
  const taken = new Set<string>();
  const out: Record<string, Country> = {};
  for (let id = 1; id <= want && pool.length; id++) {
    const capital = pool.splice(below(draw, pool.length), 1)[0]!;
    let name = "";
    while (!name || taken.has(name)) name = `${HEADS[below(draw, HEADS.length)]}${TAILS[below(draw, TAILS.length)]}`;
    taken.add(name);
    out[id] = { id, name, capital };
  }
  return out;
}

export const countryOf = (state: Maps, spot: Spot): Country | null =>
  Object.values(state.countries).reduce<Country | null>(
    (best, one) => (!best || away(one.capital, spot) < away(best.capital, spot) ? one : best),
    null,
  );

const crowded = (state: Maps, spot: Spot) =>
  Object.values(state.where).filter((one) => one.q === spot.q && one.r === spot.r).length >= state.city;

function spawned(state: Maps, id: string): Spot {
  const all = grid(state.radius);
  const dry = all.filter((one) => state.cells[key(one.q, one.r)] !== "water");
  const spots = dry.length ? dry : all;
  const draw = drawing(state.seed, state.spawn === "together" ? "gather" : `spawn/${id}`);
  const first = spots[below(draw, spots.length)]!;
  if (!crowded(state, first)) return first;
  const closest = [...spots].sort((a, b) => away(first, a) - away(first, b) || a.q - b.q || a.r - b.r);
  return closest.find((one) => !crowded(state, one)) ?? first;
}

export const whereOf = (state: Maps, id: string): Spot => state.where[id] ?? spawned(state, id);

const walkingTo = (state: Maps, id: string, at: number) => ((state.walking[id] ?? 0) > at ? state.walking[id]! : null);

const present = (state: Maps, spot: Spot, at: number) =>
  Object.keys(state.where)
    .filter((id) => state.where[id]!.q === spot.q && state.where[id]!.r === spot.r && !walkingTo(state, id, at))
    .sort();

const placesAt = (state: Maps, spot: Spot) =>
  Object.values(state.places).filter((one) => one.q === spot.q && one.r === spot.r);

const pinnedBy = (state: Maps, id: string) => Object.values(state.places).filter((one) => one.by === id);

const kept = <T>(list: T[], n: number) => (n ? list.slice(-n) : []);

const sharedPlace = (state: Maps, one: Place) => one.shared || state.own === "shared";

const boardOf = (state: Maps, id: number) => (Object.hasOwn(state.boards, id) ? state.boards[id]! : []);

const postLine = (id: number, one: Post) =>
  one.text.length > CLIP ? { ...one, text: clip(one.text, CLIP), full: `maps/board/${id}` } : one;

const boardsAt = (state: Maps, spot: Spot) =>
  state.keeps
    ? placesAt(state, spot).map((one) => ({
        place: one.id,
        name: one.name,
        by: one.by,
        shared: sharedPlace(state, one),
        posts: boardOf(state, one.id)
          .slice(-BOARD)
          .map((post) => postLine(one.id, post)),
      }))
    : [];

const hereAt = (state: Maps, spot: Spot, me: string, at: number) => ({
  people: present(state, spot, at)
    .filter((one) => one !== me)
    .slice(0, SHOWN),
  boards: boardsAt(state, spot),
});

const boardPlace = (state: Maps, args: Obj, me: string, ctx: Ctx): Place => {
  if (!state.keeps) return ctx.refuse("no place has a noticeboard in this world");
  stillWalking(state, me, ctx);
  const spot = whereOf(state, me);
  let one: Place;
  if (args.place !== undefined) {
    const hit = state.places[Number(args.place)];
    if (!hit) return ctx.refuse(`no place ${args.place}; read maps for the places near you`);
    one = hit;
  } else {
    const there = placesAt(state, spot);
    if (!there.length)
      return ctx.refuse(`no place on (${spot.q}, ${spot.r}) has a noticeboard; maps.pin one here, or maps.go to one`);
    if (there.length > 1)
      return ctx.refuse(
        `${there.length} places stand here, so say which: place ${there.map((each) => each.id).join(", ")}`,
      );
    one = there[0]!;
  }
  if (one.q !== spot.q || one.r !== spot.r)
    return ctx.refuse(
      `${one.name} is at (${one.q}, ${one.r}) and you are at (${spot.q}, ${spot.r}); maps.go place ${one.id} first`,
    );
  if (one.by !== me && !sharedPlace(state, one))
    return ctx.refuse(`place ${one.id} is ${one.by}'s and not shared; only they post on it`);
  return one;
};

const placeOf = (state: Maps, id: number, me: string, ctx: Ctx): Place => {
  const one = state.places[id];
  if (!one) return ctx.refuse(`no place ${id}; it was unpinned`);
  if (one.by !== me && ctx.knob<string>("own") !== "shared")
    return ctx.refuse(`place ${id} is ${one.by}'s; only they change it`);
  return one;
};

function aimed(state: Maps, args: Obj, refuse: Ctx["refuse"]): Spot | null {
  if (args.place !== undefined) {
    const one = state.places[Number(args.place)];
    return one ? { q: one.q, r: one.r } : refuse(`no place ${args.place}; read maps for the places near you`);
  }
  if (args.q === undefined && args.r === undefined) return null;
  if (args.q === undefined || args.r === undefined) return refuse("a cell takes both q and r");
  const q = Number(args.q);
  const r = Number(args.r);
  if (!inside(q, r, state.radius))
    return refuse(`(${q}, ${r}) is off the map; q, r and q + r stay within ${state.radius} of the centre`);
  return { q, r };
}

const called = (state: Maps, args: Obj, spot: Spot) => {
  const named = args.place === undefined ? placesAt(state, spot)[0] : state.places[Number(args.place)];
  return named ? `${named.name} (${spot.q}, ${spot.r})` : `(${spot.q}, ${spot.r})`;
};

const tell = (ctx: Ctx, people: string[], me: string, text: string, low = false) => {
  const reached = people.filter((one) => one !== me && !blockedBy(ctx, one, me));
  for (const one of reached) ctx.wish(notice(one, text, false, low));
  return reached;
};

const clip = (text: string, size: number) => {
  if (text.length <= size) return text;
  const cut = text.slice(0, size - 3);
  const space = cut.lastIndexOf(" ");
  return `${space > size / 2 ? cut.slice(0, space) : cut}...`;
};

const FULL = "maps/room";

const roomLine = (line: Said) =>
  line.text.length > CLIP ? { ...line, text: clip(line.text, CLIP), full: FULL } : line;

const roomAt = (state: Maps, spot: Spot) => (state.rooms[key(spot.q, spot.r)] ?? []).slice(-ROOM).map(roomLine);

const overheard = (head: string, text: string) => {
  const tail = ` (${FULL})`;
  return head.length + text.length <= NOTICE
    ? `${head}${text}`
    : `${head}${clip(text, NOTICE - head.length - tail.length)}${tail}`;
};

function stillWalking(state: Maps, me: string, ctx: Ctx) {
  const until = walkingTo(state, me, ctx.at);
  if (until) ctx.refuse(`you are still walking; you arrive at world time ${until}, and it is ${ctx.at}`);
}

function crossing(state: Maps, from: Spot, to: Spot, ctx: Ctx) {
  if (ctx.knob<string>("borders") !== "closed") return;
  const mine = countryOf(state, from);
  const theirs = countryOf(state, to);
  if (mine && theirs && mine.id !== theirs.id)
    ctx.refuse(`the border of ${theirs.name} is closed; stay inside ${mine.name}`);
}

const cell = (state: Maps, path: string[], at: number): Json | undefined => {
  const q = Number(path[1]);
  const r = Number(path[2]);
  if (!Number.isInteger(q) || !Number.isInteger(r) || !inside(q, r, state.radius)) return undefined;
  return walk(
    {
      q,
      r,
      terrain: state.cells[key(q, r)]!,
      country: countryOf(state, { q, r })?.name ?? null,
      people: present(state, { q, r }, at),
      places: placesAt(state, { q, r }),
      boards: boardsAt(state, { q, r }),
    },
    path.slice(3),
  );
};

const nations = (state: Maps, at: number) => {
  const tally = (spots: Spot[]) => {
    const out: Record<number, number> = {};
    for (const spot of spots) {
      const home = countryOf(state, spot);
      if (home) out[home.id] = (out[home.id] ?? 0) + 1;
    }
    return out;
  };
  const land = tally(grid(state.radius));
  const people = tally(
    Object.keys(state.where)
      .filter((id) => !walkingTo(state, id, at))
      .map((id) => state.where[id]!),
  );
  const places = tally(Object.values(state.places));
  return Object.fromEntries(
    Object.values(state.countries).map((one) => [
      one.id,
      { ...one, cells: land[one.id] ?? 0, people: people[one.id] ?? 0, places: places[one.id] ?? 0 },
    ]),
  );
};

const nearby = (state: Maps, me: string, spot: Spot, at: number) =>
  Object.keys(state.where)
    .filter((id) => id !== me && !walkingTo(state, id, at))
    .map((id) => ({ id, ...state.where[id]!, away: away(spot, state.where[id]!) }))
    .filter((one) => one.away >= 1 && one.away <= state.near)
    .sort((a, b) => a.away - b.away || a.id.localeCompare(b.id))
    .slice(0, SHOWN);

const closest = (state: Maps, spot: Spot) =>
  Object.values(state.places)
    .map((one) => ({
      id: one.id,
      name: one.name,
      q: one.q,
      r: one.r,
      by: one.by,
      shared: sharedPlace(state, one),
      away: away(spot, one),
    }))
    .sort((a, b) => a.away - b.away || a.id - b.id)
    .slice(0, SHOWN);

const mine = (state: Maps, id: string, _who: Who | null, at: number) => {
  const spot = whereOf(state, id);
  const here = hereAt(state, spot, id, at);
  const pinned = pinnedBy(state, id);
  return {
    q: spot.q,
    r: spot.r,
    terrain: state.cells[key(spot.q, spot.r)]!,
    country: countryOf(state, spot)?.name ?? null,
    arrives: walkingTo(state, id, at),
    company: here.people.length,
    here,
    near: nearby(state, id, spot, at),
    places: closest(state, spot),
    pinned: pinned.slice(-PINNED).map((one) => ({
      id: one.id,
      name: one.name,
      q: one.q,
      r: one.r,
      shared: sharedPlace(state, one),
      also: placesAt(state, one)
        .filter((other) => other.id !== one.id)
        .slice(0, ALSO)
        .map((other) => `${other.name} by ${other.by}`),
    })),
    pins: pinned.length,
    room: roomAt(state, spot),
    radius: state.radius,
  };
};

function looked(state: Maps, path: string[], who: Who | null, at: number): Json | undefined {
  const rest = path.slice(1);
  switch (path[0]) {
    case "cell":
      return cell(state, path, at);
    case "cells":
      return walk(state.cells, rest);
    case "countries":
      return walk(nations(state, at), rest);
    case "where":
      return walk(state.where, rest);
    case "atlas":
      return walk(state.places, rest);
    case "board": {
      if (!rest.length) return walk(state.boards, []);
      const one = state.places[Number(rest[0])];
      return one ? walk(boardOf(state, one.id), rest.slice(1)) : undefined;
    }
    case "room": {
      const spot = who && whereOf(state, who.id);
      return spot ? walk(state.rooms[key(spot.q, spot.r)] ?? [], rest) : undefined;
    }
    default:
      return sliced(mine)(state, path, who, at);
  }
}

const coordinate = (knob: Knob): Schema => ({ type: "integer", minimum: -knob("radius"), maximum: knob("radius") });

const placed: Schema = { type: "integer", minimum: 1 };

const person: Schema = { type: "string", minLength: 1, maxLength: 40 };

const naming = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("name") });

const going = (knob: Knob): Schema => ({
  type: "object",
  properties: { q: coordinate(knob), r: coordinate(knob), place: placed },
  additionalProperties: false,
});

const inviting = (knob: Knob): Schema => ({
  type: "object",
  properties: { person: person, place: placed, q: coordinate(knob), r: coordinate(knob) },
  required: ["person"],
  additionalProperties: false,
});

const pinning = (knob: Knob): Schema => ({
  type: "object",
  properties: { name: naming(knob), q: coordinate(knob), r: coordinate(knob), shared: { type: "boolean" } },
  required: ["name"],
  additionalProperties: false,
});

const posting = (knob: Knob): Schema => ({
  type: "object",
  properties: { text: { type: "string", minLength: 1, maxLength: knob("board") }, place: placed },
  required: ["text"],
  additionalProperties: false,
});

const changing = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { place: placed, ...more },
  required: ["place", ...Object.keys(more)],
  additionalProperties: false,
});

export default app<Maps>({
  id: "maps",
  title: "Maps",
  line: "A hex map of countries where everyone stands somewhere: go, talk to whoever is on your cell, invite people to meet, pin places and post on their noticeboards. Deeper reads: maps/cell/q/r, maps/countries, maps/room, maps/board/place, maps/atlas, maps/cells, maps/where/id.",
  boot: ({ seed, actors, knob }) => {
    const radius = knob("radius");
    const terrain = cells(seed, radius, knob("shore"), knob("land"));
    const made: Maps = {
      seed,
      radius,
      cells: terrain,
      next: 1,
      places: {},
      people: actors,
      where: {},
      walking: {},
      rooms: {},
      boards: {},
      posted: 0,
      keeps: knob("posts"),
      countries: knob<string>("borders") === "none" ? {} : partition(seed, radius, terrain, knob("countries")),
      spawn: knob("spawn"),
      city: knob("city"),
      near: knob("near"),
      own: knob<string>("own"),
    };
    for (const id of actors ?? []) made.where[id] = spawned(made, id);
    return made;
  },
  knobs: KNOBS,
  verbs: {
    arrive: {
      inner: true,
      line: "Take your place on the map, if you have none yet; it happens by itself on your first act.",
      run: (state, _args, ctx) => {
        const me = ctx.who.id;
        const known = state.where[me];
        if (known) return ctx.refuse(`you are at (${known.q}, ${known.r}) already; maps.go moves you`);
        const spot = spawned(state, me);
        ctx.say({ note: `You are at (${spot.q}, ${spot.r})`, q: spot.q, r: spot.r });
        return { ...state, where: { ...state.where, [me]: spot } };
      },
    },
    go: {
      line: (knob) => {
        const walk = knob("walk");
        const pace = walk
          ? `Walking takes ${walk / 1000} seconds of world time per cell, and you cannot go on or talk until you arrive.`
          : "You arrive at once.";
        return `Go to a cell (q, r), or to a place by its id; going to the cell you stand on does nothing. Those present hear of it. ${pace}`;
      },
      args: going,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const from = whereOf(state, me);
        const to = aimed(state, args, ctx.refuse);
        if (!to) return ctx.refuse("say where: q and r, or a place");
        stillWalking(state, me, ctx);
        if (to.q === from.q && to.r === from.r) {
          ctx.say({
            note: `You are at ${called(state, args, to)} already; nothing to do`,
            q: to.q,
            r: to.r,
            here: hereAt(state, to, me, ctx.at),
            places: placesAt(state, to)
              .slice(0, ALSO)
              .map((one) => ({ id: one.id, name: one.name })),
          });
          return state;
        }
        if (!ctx.knob("swim") && state.cells[key(to.q, to.r)] === "water")
          return ctx.refuse(`(${to.q}, ${to.r}) is water; pick a land cell from maps/cells`);
        crossing(state, from, to, ctx);
        if (crowded(state, to))
          return ctx.refuse(`(${to.q}, ${to.r}) is full at ${state.city} people; pick a neighbouring cell`);
        const pace = ctx.knob("walk") * away(from, to);
        if (ctx.knob("greet"))
          tell(
            ctx,
            present(state, to, ctx.at),
            me,
            `${me} ${pace ? "is on the way to" : "arrived at"} ${called(state, args, to)}`,
            true,
          );
        const here = hereAt(state, to, me, ctx.at);
        ctx.say({
          note: `${pace ? "On the way to" : "You are at"} ${called(state, args, to)}`,
          q: to.q,
          r: to.r,
          ...(pace ? { arrives: ctx.at + pace } : {}),
          company: here.people.length,
          here,
          places: placesAt(state, to)
            .slice(0, ALSO)
            .map((one) => ({ id: one.id, name: one.name })),
          room: roomAt(state, to),
        });
        const walking = { ...state.walking };
        if (pace) walking[me] = ctx.at + pace;
        else delete walking[me];
        return { ...state, where: { ...state.where, [me]: to }, walking };
      },
    },
    say: {
      line: (knob) =>
        `Say something to everyone on your cell, 1 to ${knob("text")} characters; they get a notice, and the cell keeps its last ${knob("history")} lines.`,
      args: (knob) => ({
        type: "object",
        properties: { text: { type: "string", minLength: 1, maxLength: knob("text") } },
        required: ["text"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("that is only spaces; say something");
        stillWalking(state, me, ctx);
        const spot = whereOf(state, me);
        const room = key(spot.q, spot.r);
        const heard = tell(
          ctx,
          present(state, spot, ctx.at),
          me,
          overheard(`${me} at (${spot.q}, ${spot.r}): `, text),
          true,
        );
        const lines = kept([...(state.rooms[room] ?? []), { by: me, text, at: ctx.at }], ctx.knob("history"));
        const next = { ...state, rooms: { ...state.rooms, [room]: lines } };
        ctx.say({ note: `Said to ${heard.length} on (${spot.q}, ${spot.r})`, heard, room: roomAt(next, spot) });
        return next;
      },
    },
    invite: {
      line: "Ask someone to meet you at a place or a cell (q, r), or with neither, where you stand. They get a notice.",
      args: inviting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const person = someone(state.people, String(args.person), ctx);
        if (person === me) return ctx.refuse("you cannot invite yourself; invite someone else");
        if (blockedBy(ctx, person, me)) return ctx.refuse(`${person} is not taking this from you`);
        const spot = aimed(state, args, ctx.refuse) ?? whereOf(state, me);
        ctx.wish(notice(person, `${me} invites you to meet at ${called(state, args, spot)}`));
        ctx.say({ note: `Asked ${person} to meet at ${called(state, args, spot)}`, q: spot.q, r: spot.r });
        return state;
      },
    },
    pin: {
      line: (knob) =>
        `Pin a place where you stand, or on a cell (q, r), and name it, 1 to ${knob("name")} characters; you pin at most ${knob("pins")}.${knob<boolean>("squares") ? " Pin with shared true for a town square: anyone standing there posts on its noticeboard." : ""} See what stands there first: read maps/cell/q/r. Your reply lists it as also.`,
      args: pinning,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const spot = aimed(state, args, ctx.refuse) ?? whereOf(state, me);
        const text = String(args.name).trim();
        if (!text) return ctx.refuse("the name is only spaces; name the place");
        const shared = args.shared === true;
        if (shared && !ctx.knob("squares"))
          return ctx.refuse("shared places are off in this world; pin without shared");
        const there = placesAt(state, spot);
        const twin = there.find((one) => one.by === me && one.name.toLowerCase() === text.toLowerCase());
        if (twin)
          return ctx.refuse(`you pinned ${twin.name} on (${spot.q}, ${spot.r}) already, as place ${twin.id}; use that`);
        if (pinnedBy(state, me).length >= ctx.knob("pins"))
          return ctx.refuse(`you pinned ${ctx.knob("pins")} places; unpin one first`);
        if (there.length >= ctx.knob("crowd"))
          return ctx.refuse(`(${spot.q}, ${spot.r}) holds ${there.length} places, its most; pick a neighbouring cell`);
        const id = state.next;
        const place: Place = { id, q: spot.q, r: spot.r, name: text, by: me, at: ctx.at, shared };
        ctx.say({
          note: `Pinned ${shared ? "shared " : ""}place ${id}: ${text} (${spot.q}, ${spot.r})`,
          id,
          shared,
          also: there.slice(0, ALSO).map((other) => `${other.name} by ${other.by}`),
        });
        return { ...state, next: id + 1, places: { ...state.places, [id]: place } };
      },
    },
    post: {
      line: (knob) =>
        knob<number>("posts")
          ? `Post on the noticeboard of a place where you stand, 1 to ${knob("board")} characters; a board keeps its last ${knob("posts")}. Everyone standing there reads it in maps under here. Anyone posts on a shared place; on others only their pinner. Name the place when several stand on your cell.`
          : "No place has a noticeboard in this world.",
      args: posting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = boardPlace(state, args, me, ctx);
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("that is only spaces; write something");
        const id = state.posted + 1;
        if (one.by !== me && !blockedBy(ctx, one.by, me))
          ctx.wish(notice(one.by, overheard(`${me} posted on ${one.name}: `, text)));
        const posts = kept([...boardOf(state, one.id), { id, by: me, text, at: ctx.at }], state.keeps);
        const next = { ...state, posted: id, boards: { ...state.boards, [one.id]: posts } };
        ctx.say({
          note: `Posted on ${one.name} (place ${one.id}); it keeps ${posts.length} of ${state.keeps}`,
          id,
          place: one.id,
          board: boardsAt(next, one).find((each) => each.place === one.id)!.posts,
        });
        return next;
      },
    },
    rename: {
      line: (knob) =>
        knob<string>("own") === "shared" ? "Rename any place; here places are shared." : "Rename a place of yours.",
      args: (knob) => changing({ name: naming(knob) }),
      run: (state, args, ctx) => {
        const one = placeOf(state, Number(args.place), ctx.who.id, ctx);
        const text = String(args.name).trim();
        if (!text) return ctx.refuse("the name is only spaces; name the place");
        ctx.say({ note: `Place ${one.id} is now ${text}`, id: one.id });
        return { ...state, places: { ...state.places, [one.id]: { ...one, name: text } } };
      },
    },
    unpin: {
      line: (knob) =>
        knob<string>("own") === "shared"
          ? "Take any place off the map; here places are shared."
          : "Take a place of yours off the map.",
      args: changing(),
      run: (state, args, ctx) => {
        const one = placeOf(state, Number(args.place), ctx.who.id, ctx);
        const places = { ...state.places };
        delete places[one.id];
        const boards = { ...state.boards };
        delete boards[one.id];
        ctx.say({ note: `Unpinned place ${one.id}: ${one.name}`, id: one.id });
        return { ...state, places, boards };
      },
    },
  },
  moves: (state) => [
    ...(state.people ? [] : ["arrive"]),
    "go",
    "say",
    "invite",
    ...(state.own === "none" ? [] : ["pin"]),
    ...(Object.keys(state.places).length ? [...(state.keeps ? ["post"] : []), "rename", "unpin"] : []),
  ],
  read: looked,
  after: ({ world, line }) => {
    const state = world.apps.maps as unknown as Maps;
    return state.people || state.where[line.who.id] ? null : { verb: "maps.arrive", args: {} };
  },
});
