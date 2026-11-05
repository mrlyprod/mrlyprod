import type { Ctx, Json, Knob, Knobs, Obj, Peek, Schema, Setting, Who } from "@mrlyprod/os/contract/index.ts";
import { app, nearest, sliced } from "@mrlyprod/os/contract/index.ts";
import { reserved as isReserved, skeleton } from "../shared/names.ts";
import { notice, reach } from "../shared/people.ts";

export type Card = {
  handles: string[];
  created: number;
  birthday?: string;
  birthplace?: string;
  picture?: number;
  model?: string;
  favorites?: Record<string, string[]>;
  parents?: string[];
  asked?: string[];
  hidden?: string[];
};

export type Iden = { cards: Record<string, Card> };

const entry = (what: string): Setting => ({
  line: `${what}: free entry, select from the world's vocabulary, or off.`,
  schema: { enum: ["free", "select", "off"] },
  reality: "free",
});

const KNOBS = {
  handles: {
    line: "Handles one person may hold.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 3,
  },
  size: {
    line: "Longest handle.",
    schema: { type: "integer", minimum: 8, maximum: 64 },
    reality: 20,
  },
  birthday: entry("Birthdays"),
  birthplace: entry("Birthplaces"),
  favorites: entry("Favorites"),
  picture: {
    line: "Profile pictures, always a photo you own: on or off.",
    schema: { enum: ["free", "off"] },
    reality: "free",
  },
  model: {
    line: "The model a person says they run on: off, claimed (say anything), or true (a claim must match the real one).",
    schema: { enum: ["off", "claimed", "true"] },
    reality: "claimed",
  },
  source: {
    line: "The app whose lists are this world's vocabulary: <app>/<kind> gives a list of id and name. None means free.",
    schema: { type: "string", maxLength: 40 },
    reality: "",
  },
  groups: {
    line: "Kinds of the vocabulary gathered under one word, such as food or place.",
    schema: { type: "object" },
    reality: {},
  },
  models: {
    line: "What each person truly runs on, by id, for a world whose model knob is true.",
    schema: { type: "object" },
    reality: {},
  },
  parents: {
    line: "Parents one person may have. Zero means nobody has any.",
    schema: { type: "integer", minimum: 0, maximum: 16 },
    reality: 2,
  },
  each: {
    line: "Favorites one person keeps of each kind.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 10,
  },
  text: {
    line: "Longest birthplace, favorite or model name a person may type freely.",
    schema: { type: "integer", minimum: 20, maximum: 500 },
    reality: 80,
  },
  near: {
    line: "Options a refusal names when a value is not in the vocabulary.",
    schema: { type: "integer", minimum: 1, maximum: 20 },
    reality: 5,
  },
} satisfies Knobs;

const LEAST = 3;

export const HANDLES = /^[a-z][a-z0-9-]*$/;

const reserved = (handle: string, ctx: Ctx) => isReserved(handle) || Object.hasOwn((ctx.read("") ?? {}) as Obj, handle);

const lookalike = (state: Iden, handle: string, ctx: Ctx) =>
  [...Object.keys((ctx.read("") ?? {}) as Obj), ...Object.keys(state.cards)].find(
    (id) => skeleton(id) === skeleton(handle),
  ) ??
  Object.values(state.cards)
    .flatMap((card) => card.handles)
    .find((one) => skeleton(one) === skeleton(handle));

const naming = (knob: Knob): Schema => ({
  type: "object",
  properties: { handle: { type: "string", minLength: LEAST, maxLength: knob("size") } },
  required: ["handle"],
  additionalProperties: false,
});

const holder = (state: Iden, handle: string) =>
  Object.keys(state.cards).find((id) => id === handle || state.cards[id]!.handles.includes(handle));

const book = (state: Iden) =>
  Object.fromEntries(
    Object.entries(state.cards).flatMap(([id, card]) => card.handles.map((handle): [string, string] => [handle, id])),
  );

const ID = 64;

const OPEN = 500;

const DATE = 32;

const KIND = /^[a-z][a-z-]*$/;

const DAY = /^\d{4}-\d{2}-\d{2}$/;

const BASICS = ["show", "film", "book", "song", "activity", "food"];

const SAME: Record<string, string> = { movie: "film", movies: "film", tv: "show", series: "show" };

const WORDS = { birthday: "date", birthplace: "place" };

export const FIELDS = ["birthday", "birthplace", "picture", "model", "favorites", "parents", "children"];

const cardOf = (state: Iden, id: string): Card | null => (Object.hasOwn(state.cards, id) ? state.cards[id]! : null);

const parentsOf = (state: Iden, id: string) => cardOf(state, id)?.parents ?? [];

const askedOf = (state: Iden, id: string) => cardOf(state, id)?.asked ?? [];

const childrenOf = (state: Iden, id: string) =>
  Object.keys(state.cards).filter((one) => parentsOf(state, one).includes(id));

const asksOf = (state: Iden, id: string) => Object.keys(state.cards).filter((one) => askedOf(state, one).includes(id));

const hides = (card: Card | null, field: string) => (card?.hidden ?? []).includes(field);

const pictured = (id: string, photo: number | undefined, peek: Peek): Json | undefined => {
  if (photo === undefined) return undefined;
  const photos = peek(`photos/libraries/${id}/photos`) as { id: number; title: string; hash: string }[] | null;
  const one = photos?.find((each) => each.id === photo);
  return one && { photo, title: one.title, hash: one.hash };
};

const profile = (state: Iden, id: string, mine: boolean, peek: Peek): Obj => {
  const card = cardOf(state, id);
  const children = childrenOf(state, id).filter((one) => mine || !hides(cardOf(state, one), "parents"));
  const fields: Record<string, Json | undefined> = {
    birthday: card?.birthday,
    birthplace: card?.birthplace,
    picture: pictured(id, card?.picture, peek),
    model: card?.model,
    favorites: card?.favorites,
    parents: card?.parents,
    children: children.length ? children : undefined,
  };
  return Object.fromEntries(
    Object.entries(fields).filter((pair): pair is [string, Json] => {
      const [field, value] = pair;
      return value !== undefined && (mine || !hides(card, field));
    }),
  );
};

const shown = (state: Iden, id: string, who: Who | null, _at: number, peek: Peek): Json | undefined => {
  const owner = holder(state, id) ?? id;
  const mine = who?.id === owner;
  const card = cardOf(state, owner) ?? (mine ? { handles: [], created: 0 } : null);
  if (!card) return undefined;
  const base = { id: owner, handles: card.handles, created: card.created, ...profile(state, owner, mine, peek) };
  if (!mine) return base;
  const asks = asksOf(state, owner);
  return {
    ...base,
    kind: who.kind,
    ...(card.hidden?.length ? { hidden: card.hidden } : {}),
    ...(card.asked?.length ? { asked: card.asked } : {}),
    ...(asks.length ? { asks } : {}),
  };
};

const front = (state: Iden, who: Who): Json => {
  const card = cardOf(state, who.id);
  const asks = asksOf(state, who.id);
  return {
    id: who.id,
    kind: who.kind,
    handles: card?.handles ?? [],
    created: card?.created ?? 0,
    ...(asks.length ? { asks } : {}),
    profile: "read identity/@me",
    book: book(state),
  };
};

const whole = (state: Iden, who: Who | null, at: number, peek: Peek): Json =>
  who
    ? front(state, who)
    : {
        cards: Object.fromEntries(Object.keys(state.cards).map((id) => [id, shown(state, id, null, at, peek) ?? null])),
        book: book(state),
      };

const tidy = (card: Card): Card =>
  Object.fromEntries(
    Object.entries(card).filter(([key, value]) => {
      if (key === "handles") return true;
      if (Array.isArray(value)) return value.length > 0;
      if (value && typeof value === "object") return Object.keys(value).length > 0;
      return value !== undefined && value !== "";
    }),
  ) as Card;

const filed = (state: Iden, id: string, change: Partial<Card>, ctx: Ctx): Iden => ({
  cards: {
    ...state.cards,
    [id]: tidy({ ...(cardOf(state, id) ?? { handles: [], created: ctx.at }), ...change }),
  },
});

const plain = (text: string) =>
  text
    .normalize("NFKD")
    .replace(/\p{M}/gu, "")
    .toLowerCase()
    .replace(/['’]/g, "")
    .replace(/[^\p{L}\p{N}]+/gu, "-")
    .replace(/^-+|-+$/g, "");

const same = (a: string, b: string) => a === b || (!!plain(a) && plain(a) === plain(b));

type Option = { id: string; name: string; kind?: string };

const vocabulary = (ctx: Ctx): string | null => {
  const id = ctx.knob<string>("source");
  return id && ctx.see(id) !== null ? id : null;
};

const kindsIn = (ctx: Ctx, kind: string): string[] => {
  const groups = ctx.knob("groups") as Record<string, string[]>;
  return Object.hasOwn(groups, kind) ? groups[kind]! : [kind];
};

const listed = (ctx: Ctx, source: string, kind: string): Option[] => {
  const got = ctx.see(`${source}/${kind}`);
  if (!Array.isArray(got)) return [];
  return (got as Option[]).filter((one) => one?.name && (one.kind === undefined || one.kind === kind));
};

const optionsOf = (ctx: Ctx, kind: string): Option[] => {
  const source = vocabulary(ctx);
  return source ? kindsIn(ctx, kind).flatMap((one) => listed(ctx, source, one)) : [];
};

const closest = (word: string, names: string[], many: number): string[] => {
  const left = new Map(names.map((name) => [plain(name), name]));
  const out: string[] = [];
  while (out.length < many && left.size) {
    const one = nearest(plain(word), [...left.keys()])!;
    out.push(left.get(one)!);
    left.delete(one);
  }
  return out;
};

function chosen(ctx: Ctx, kind: string, value: string, options: Option[]): string {
  const hit = options.find((one) => one.id === value || same(one.id, value) || same(one.name, value));
  if (hit) return hit.name;
  const near = closest(
    value,
    options.map((one) => one.name),
    ctx.knob("near"),
  );
  const where = kindsIn(ctx, kind)
    .map((one) => `${vocabulary(ctx)}/${one}`)
    .join(", ");
  return ctx.refuse(`${value} is not a ${kind} in this world; nearest: ${near.join(", ")}; read ${where} for all`);
}

const entered = (ctx: Ctx, field: keyof typeof WORDS, value: string): { value: string; picked: boolean } => {
  const mode = ctx.knob<string>(field);
  if (mode === "off") return ctx.refuse(`nobody has a ${field} in this world`);
  const options = mode === "select" && value ? optionsOf(ctx, WORDS[field]) : [];
  return options.length ? { value: chosen(ctx, WORDS[field], value, options), picked: true } : { value, picked: false };
};

function dated(ctx: Ctx, date: string) {
  const when = Date.parse(`${date}T00:00:00Z`);
  const real = DAY.test(date) && Number.isFinite(when) && new Date(when).toISOString().startsWith(date);
  if (!real) ctx.refuse(`${date} is not a date; write it as YYYY-MM-DD, or send an empty date to clear it`);
  if (when > ctx.at)
    ctx.refuse(`${date} has not happened yet; today is ${new Date(ctx.at).toISOString().slice(0, 10)}`);
}

const singular = (word: string) => {
  if (Object.hasOwn(SAME, word)) return SAME[word]!;
  if (word.endsWith("ies")) return `${word.slice(0, -3)}y`;
  return word.endsWith("s") && !word.endsWith("ss") ? word.slice(0, -1) : word;
};

function kindNamed(ctx: Ctx, word: string): string {
  const low = word.trim().toLowerCase();
  if (!KIND.test(low)) return ctx.refuse(`${word} is not a kind; try ${BASICS.join(", ")}`);
  return optionsOf(ctx, low).length ? low : singular(low);
}

function favoriteKind(ctx: Ctx, kind: string, options: Option[]) {
  const source = vocabulary(ctx);
  if (options.length) return;
  if (source && ctx.knob<string>("favorites") === "select")
    ctx.refuse(`this world has no ${kind}; read ${source} for the kinds it has`);
  if (!BASICS.includes(kind))
    ctx.refuse(`no kind ${kind}; try ${BASICS.join(", ")}${source ? `, or a kind from read ${source}` : ""}`);
}

const personNamed = (state: Iden, name: string, ctx: Ctx): string => {
  const held = holder(state, name);
  if (held) return held;
  const one = name.includes("/") ? null : (ctx.read(`people/${name}`) as { name?: unknown } | null);
  return typeof one?.name === "string" ? name : ctx.refuse(`no person ${name}; read people for everyone here`);
};

const ancestors = (state: Iden, id: string): Set<string> => {
  const seen = new Set<string>();
  const walk = (one: string) => {
    for (const up of parentsOf(state, one)) {
      if (seen.has(up)) continue;
      seen.add(up);
      walk(up);
    }
  };
  walk(id);
  return seen;
};

function confirm(state: Iden, child: string, ctx: Ctx): Iden {
  const me = ctx.who.id;
  const most = ctx.knob("parents");
  if (parentsOf(state, child).length >= most) return ctx.refuse(`${child} already has ${most} parents`);
  if (ancestors(state, me).has(child)) return ctx.refuse(`${child} is your own ancestor`);
  ctx.wish(notice(child, `${me} confirmed: they are your parent now`));
  ctx.say({ note: `You are now the parent of ${child}`, child });
  return filed(
    state,
    child,
    { parents: [...parentsOf(state, child), me], asked: askedOf(state, child).filter((one) => one !== me) },
    ctx,
  );
}

function ask(state: Iden, person: string, ctx: Ctx): Iden {
  const me = ctx.who.id;
  const most = ctx.knob("parents");
  const have = parentsOf(state, me);
  const asked = askedOf(state, me);
  if (have.includes(person)) return ctx.refuse(`${person} is already your parent`);
  if (asked.includes(person))
    return ctx.refuse(`you already asked ${person}; wait for them, or use identity.unparent to withdraw`);
  if (ancestors(state, person).has(me)) return ctx.refuse(`${person} descends from you, so cannot be your parent`);
  if (have.length + asked.length >= most)
    return ctx.refuse(`you have ${most} parents or open asks, the most here; use identity.unparent to free one`);
  ctx.wish(notice(person, `${me} asks you to be their parent; say yes with identity.parent ${me}`));
  ctx.say({ note: `Asked ${person} to be your parent; they confirm with parent`, person });
  return filed(state, me, { asked: [...asked, person] }, ctx);
}

function unlink(state: Iden, person: string, ctx: Ctx): Iden {
  const me = ctx.who.id;
  const end = (child: string, change: Partial<Card>, mine: string, theirs: string) => {
    ctx.wish(notice(person, theirs));
    ctx.say({ note: mine, person });
    return filed(state, child, change, ctx);
  };
  const off = (list: string[], one: string) => list.filter((each) => each !== one);
  if (parentsOf(state, me).includes(person))
    return end(
      me,
      { parents: off(parentsOf(state, me), person) },
      `${person} is no longer your parent`,
      `${me} no longer counts you as a parent`,
    );
  if (parentsOf(state, person).includes(me))
    return end(
      person,
      { parents: off(parentsOf(state, person), me) },
      `${person} is no longer your child`,
      `${me} no longer counts you as a child`,
    );
  if (askedOf(state, me).includes(person))
    return end(
      me,
      { asked: off(askedOf(state, me), person) },
      `Withdrew your ask to ${person}`,
      `${me} withdrew their ask that you be their parent`,
    );
  if (askedOf(state, person).includes(me))
    return end(
      person,
      { asked: off(askedOf(state, person), me) },
      `Declined ${person}'s ask`,
      `${me} declined to be your parent`,
    );
  return ctx.refuse(`${person} is no parent, child or open ask of yours; read identity/@me`);
}

const taking = (properties: Record<string, Schema>, required = Object.keys(properties)): Schema => ({
  type: "object",
  properties,
  required,
  additionalProperties: false,
});

const texting = (knob: Knob, checked: boolean, min = 0): Schema => ({
  type: "string",
  minLength: min,
  maxLength: checked ? OPEN : knob("text"),
});

const liking = (knob: Knob): Schema =>
  taking({
    kind: { type: "string", minLength: 1, maxLength: ID },
    thing: texting(knob, knob<string>("favorites") === "select", 1),
  });

const person: Schema = taking({ person: { type: "string", minLength: 1, maxLength: ID } });

const field: Schema = taking({ field: { enum: FIELDS } });

const how = (knob: Knob, field: "birthday" | "birthplace" | "favorites") =>
  knob<string>(field) === "select" ? " From this world's vocabulary, if it has one; a refusal names the nearest." : "";

export default app<Iden>({
  id: "identity",
  title: "Identity",
  line: "Each person's handles and profile: birthday, birthplace, picture, model, favorites, parents. read identity/@me for yours, identity/@x for what x shows. No passwords yet.",
  boot: ({ actors }) => ({
    cards: Object.fromEntries((actors ?? []).map((id) => [id, { handles: [], created: 0 }])),
  }),
  verbs: {
    handle: {
      line: (knob) =>
        `Add a handle: ${LEAST} to ${knob("size")} lowercase letters, digits and hyphens, one nobody else holds. Limit: ${knob("handles")} handles.`,
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const handle = String(args.handle);
        if (!HANDLES.test(handle))
          return ctx.refuse(
            `a handle starts with a letter and holds lowercase letters, digits and hyphens, not ${handle}`,
          );
        if (reserved(handle, ctx)) return ctx.refuse(`${handle} is reserved; pick another`);
        const card = state.cards[me] ?? { handles: [], created: ctx.at };
        const taken = holder(state, handle);
        if (taken === me || handle === me) return ctx.refuse(`${handle} is already yours`);
        if (taken) return ctx.refuse(`${taken} holds the handle ${handle}; pick another`);
        const near = lookalike(state, handle, ctx);
        if (near && !(card.handles.includes(near) || near === me))
          return ctx.refuse(`${handle} looks too like ${near}; pick another`);
        const most = ctx.knob("handles");
        if (card.handles.length >= most)
          return ctx.refuse(`you are at the limit of ${most} handles; use identity.drop to free one`);
        ctx.say({ note: `You hold the handle ${handle}`, handle });
        return { cards: { ...state.cards, [me]: { ...card, handles: [...card.handles, handle] } } };
      },
    },
    drop: {
      line: "Give up one of your handles; anyone may take it after.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const handle = String(args.handle);
        const card = state.cards[me];
        if (!card?.handles.includes(handle)) return ctx.refuse(`you do not hold ${handle}; read identity for yours`);
        ctx.say({ note: `You gave up the handle ${handle}`, handle });
        return { cards: { ...state.cards, [me]: { ...card, handles: card.handles.filter((one) => one !== handle) } } };
      },
    },
    birthday: {
      line: (knob) => `Set your birthday, YYYY-MM-DD; an empty date clears it.${how(knob, "birthday")}`,
      args: taking({ date: { type: "string", maxLength: DATE } }),
      run: (state, args, ctx) => {
        const { value, picked } = entered(ctx, "birthday", String(args.date).trim());
        if (value && !picked) dated(ctx, value);
        ctx.say({ note: value ? `Your birthday is ${value}` : "Cleared your birthday", birthday: value });
        return filed(state, ctx.who.id, { birthday: value }, ctx);
      },
    },
    birthplace: {
      line: (knob) => `Set where you were born; an empty place clears it.${how(knob, "birthplace")}`,
      args: (knob) => taking({ place: texting(knob, knob<string>("birthplace") === "select") }),
      run: (state, args, ctx) => {
        const { value } = entered(ctx, "birthplace", String(args.place).trim());
        ctx.say({ note: value ? `You were born in ${value}` : "Cleared your birthplace", birthplace: value });
        return filed(state, ctx.who.id, { birthplace: value }, ctx);
      },
    },
    picture: {
      line: "Set your profile picture to a photo you own; send no photo to clear it.",
      args: taking({ photo: { type: "integer", minimum: 1 } }, []),
      run: (state, args, ctx) => {
        if (ctx.knob<string>("picture") === "off") return ctx.refuse("nobody has a picture in this world");
        const me = ctx.who.id;
        if (args.photo === undefined) {
          ctx.say({ note: "Cleared your picture" });
          return filed(state, me, { picture: undefined }, ctx);
        }
        const photo = Number(args.photo);
        const own = (ctx.see("photos/@me/photos") ?? []) as { id: number }[];
        if (!own.some((one) => one.id === photo))
          return ctx.refuse(`photo ${photo} is not yours; read photos/@me/photos, or take one with photos.take`);
        ctx.say({ note: `Photo ${photo} is your picture`, photo });
        return filed(state, me, { picture: photo }, ctx);
      },
    },
    model: {
      line: (knob) => {
        const mode = knob<string>("model");
        if (mode === "off") return "Nobody says what model they run on in this world.";
        return `Say what model you run on; an empty name clears it.${mode === "true" ? " It must be the true one." : " Anything goes: nobody checks."}`;
      },
      args: (knob) => taking({ name: texting(knob, knob<string>("model") === "true") }),
      run: (state, args, ctx) => {
        const mode = ctx.knob<string>("model");
        if (mode === "off") return ctx.refuse("nobody says what model they run on in this world");
        const me = ctx.who.id;
        const name = String(args.name).trim();
        const real = (ctx.knob<Obj>("models") ?? {}) as Record<string, string>;
        if (mode === "true" && name && Object.keys(real).length) {
          if (!Object.hasOwn(real, me)) return ctx.refuse("this world knows no model for you, and claims must be true");
          if (!same(real[me]!, name)) return ctx.refuse(`claims must be true here, and you run on ${real[me]}`);
        }
        ctx.say({ note: name ? `You say you run on ${name}` : "Cleared your model", model: name });
        return filed(state, me, { model: name }, ctx);
      },
    },
    favorite: {
      line: (knob) =>
        `Add a favorite: a kind (${BASICS.join(", ")}, or any kind the world's vocabulary has) and a thing. Up to ${knob("each")} of each kind.${how(knob, "favorites")}`,
      args: liking,
      run: (state, args, ctx) => {
        const mode = ctx.knob<string>("favorites");
        if (mode === "off") return ctx.refuse("nobody has favorites in this world");
        const me = ctx.who.id;
        const kind = kindNamed(ctx, String(args.kind));
        const typed = String(args.thing).trim();
        if (!typed) return ctx.refuse("the thing is only spaces; name it");
        const options = optionsOf(ctx, kind);
        favoriteKind(ctx, kind, options);
        const thing = mode === "select" && options.length ? chosen(ctx, kind, typed, options) : typed;
        const favorites = cardOf(state, me)?.favorites ?? {};
        const held = Object.hasOwn(favorites, kind) ? favorites[kind]! : [];
        if (held.some((one) => same(one, thing))) return ctx.refuse(`${thing} is already a favorite ${kind}`);
        const most = ctx.knob("each");
        if (held.length >= most)
          return ctx.refuse(`you keep ${most} of each kind at most; use identity.unfavorite to drop a ${kind}`);
        ctx.say({ note: `${thing} is now a favorite ${kind}`, kind, thing });
        return filed(state, me, { favorites: { ...favorites, [kind]: [...held, thing] } }, ctx);
      },
    },
    unfavorite: {
      line: "Drop a favorite: its kind and thing.",
      args: liking,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const word = String(args.kind).trim().toLowerCase();
        const favorites = cardOf(state, me)?.favorites ?? {};
        const kind = Object.hasOwn(favorites, word) ? word : singular(word);
        const held = Object.hasOwn(favorites, kind) ? favorites[kind]! : [];
        const thing = held.find((one) => same(one, String(args.thing).trim()));
        if (!thing) return ctx.refuse(`${args.thing} is not a favorite ${kind} of yours; read identity/@me`);
        const left = held.filter((one) => one !== thing);
        const { [kind]: _gone, ...rest } = favorites;
        ctx.say({ note: `${thing} is no longer a favorite ${kind}`, kind, thing });
        return filed(state, me, { favorites: left.length ? { ...rest, [kind]: left } : rest }, ctx);
      },
    },
    parent: {
      line: (knob) =>
        `Ask someone to be your parent; they confirm by naming you with parent too. Limit: ${knob("parents")} parents.`,
      args: person,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        if (!ctx.knob("parents")) return ctx.refuse("nobody has parents in this world");
        const other = personNamed(state, String(args.person), ctx);
        if (other === me) return ctx.refuse("that is you; name someone else");
        reach(ctx, other);
        return askedOf(state, other).includes(me) ? confirm(state, other, ctx) : ask(state, other, ctx);
      },
    },
    unparent: {
      line: "End a link with a parent or child, withdraw your ask, or decline theirs.",
      args: person,
      run: (state, args, ctx) => unlink(state, personNamed(state, String(args.person), ctx), ctx),
    },
    hide: {
      line: "Make a field of your profile private: others no longer see it.",
      args: field,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = String(args.field);
        const hidden = cardOf(state, me)?.hidden ?? [];
        if (hidden.includes(one)) return ctx.refuse(`your ${one} is already hidden`);
        ctx.say({ note: `Your ${one} is hidden`, field: one });
        return filed(state, me, { hidden: [...hidden, one] }, ctx);
      },
    },
    show: {
      line: "Make a hidden field of your profile public again.",
      args: field,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = String(args.field);
        const hidden = cardOf(state, me)?.hidden ?? [];
        if (!hidden.includes(one)) return ctx.refuse(`your ${one} is not hidden`);
        ctx.say({ note: `Others can see your ${one}`, field: one });
        return filed(state, me, { hidden: hidden.filter((each) => each !== one) }, ctx);
      },
    },
  },
  read: sliced(shown, whole, true),
  knobs: KNOBS,
});
