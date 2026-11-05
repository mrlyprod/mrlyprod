import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, someone } from "../shared/people.ts";
import { hashOf, image } from "./image.ts";

export type Scene = { q: number; r: number; name: string | null };

export type Photo = {
  id: number;
  hash: string;
  title: string;
  at: number;
  from?: string;
  place?: Scene;
  with?: string[];
};

export type Album = { id: number; title: string; photos: number[]; with: string[] };

export type Library = { photos: Photo[]; albums: Album[] };

export type Photos = {
  people: string[] | null;
  next: number;
  images: Record<string, { seed: number }>;
  libraries: Record<string, Library>;
};

const ID = 40;

const KNOBS = {
  title: {
    line: "Characters one photo or album title holds.",
    schema: { type: "integer", minimum: 5, maximum: 500 },
    reality: 60,
  },
  photos: { line: "Photos one person keeps.", schema: { type: "integer", minimum: 1, maximum: 5000 }, reality: 100 },
  albums: { line: "Albums one person keeps.", schema: { type: "integer", minimum: 1, maximum: 500 }, reality: 20 },
  holds: { line: "Photos one album holds.", schema: { type: "integer", minimum: 1, maximum: 5000 }, reality: 200 },
  sharers: {
    line: "People one album may be shared with.",
    schema: { type: "integer", minimum: 1, maximum: 256 },
    reality: 20,
  },
  company: {
    line: "People one photo names as there when it was taken.",
    schema: { type: "integer", minimum: 0, maximum: 256 },
    reality: 20,
  },
} satisfies Knobs;

const EMPTY: Library = { photos: [], albums: [] };

const libraryOf = (state: Photos, me: string) => state.libraries[me] ?? EMPTY;

const put = (state: Photos, me: string, library: Library): Photos => ({
  ...state,
  libraries: { ...state.libraries, [me]: library },
});

const ownerOf = (state: Photos, id: number, kind: "photos" | "albums") =>
  Object.keys(state.libraries).find((one) => state.libraries[one]![kind].some((each) => each.id === id));

function photoOf(state: Photos, id: number, me: string, refuse: Ctx["refuse"]): Photo {
  const owner = ownerOf(state, id, "photos");
  if (!owner) return refuse(`no photo ${id}; it was deleted`);
  if (owner !== me) return refuse(`photo ${id} is someone else's; ask them to send it to you`);
  return libraryOf(state, me).photos.find((one) => one.id === id)!;
}

type Held = { owner: string; album: Album };

const heldAlbum = (state: Photos, id: number): Held | null => {
  const owner = ownerOf(state, id, "albums");
  return owner ? { owner, album: libraryOf(state, owner).albums.find((one) => one.id === id)! } : null;
};

function albumFor(state: Photos, id: number, me: string, refuse: Ctx["refuse"]): Held {
  const held = heldAlbum(state, id);
  if (!held || (held.owner !== me && !held.album.with.includes(me)))
    return refuse(`you have no album ${id}; read photos for yours, or ask its owner to share it`);
  return held;
}

function ownAlbum(state: Photos, id: number, me: string, refuse: Ctx["refuse"]): Album {
  const { owner, album } = albumFor(state, id, me, refuse);
  return owner === me ? album : refuse(`album ${id} is ${owner}'s; only its owner may`);
}

const filed = (state: Photos, owner: string, album: Album): Photos => {
  const library = libraryOf(state, owner);
  return put(state, owner, { ...library, albums: library.albums.map((one) => (one.id === album.id ? album : one)) });
};

const tell = (ctx: Ctx, { owner, album }: Held, text: string) => {
  const me = ctx.who.id;
  for (const who of [owner, ...album.with]) if (who !== me) ctx.wish(notice(who, text));
};

function swept(state: Photos): Photos {
  const used = new Set(Object.values(state.libraries).flatMap((library) => library.photos.map((photo) => photo.hash)));
  return { ...state, images: Object.fromEntries(Object.entries(state.images).filter(([hash]) => used.has(hash))) };
}

const clearedOf = (state: Photos, id: number): Photos => ({
  ...state,
  libraries: Object.fromEntries(
    Object.entries(state.libraries).map(([who, library]) => [
      who,
      {
        ...library,
        albums: library.albums.map((album) => ({ ...album, photos: album.photos.filter((one) => one !== id) })),
      },
    ]),
  ),
});

const anyPhoto = (state: Photos, id: number): (Photo & { by: string }) | null => {
  const by = ownerOf(state, id, "photos");
  return by ? { ...libraryOf(state, by).photos.find((one) => one.id === id)!, by } : null;
};

const albumRow = (owner: string, album: Album) => ({
  id: album.id,
  title: album.title,
  owner,
  with: album.with,
  photos: album.photos.length,
});

const albumFull = (state: Photos, owner: string, album: Album) => ({
  ...albumRow(owner, album),
  photos: album.photos.map((id) => anyPhoto(state, id)).filter((one) => one !== null),
});

const reachable = (state: Photos, me: string): [string, Album][] =>
  Object.entries(state.libraries).flatMap(([owner, library]) =>
    library.albums
      .filter((album) => owner === me || album.with.includes(me))
      .map((album): [string, Album] => [owner, album]),
  );

const everything = (state: Photos, me: string): Json => ({
  photos: libraryOf(state, me).photos,
  albums: Object.fromEntries(reachable(state, me).map(([owner, album]) => [album.id, albumFull(state, owner, album)])),
});

const overview = (state: Photos, me: string): Json => ({
  photos: libraryOf(state, me).photos.length,
  albums: Object.fromEntries(reachable(state, me).map(([owner, album]) => [album.id, albumRow(owner, album)])),
});

const count = (state: Photos): Json => ({
  people: Object.keys(state.libraries).length,
  photos: Object.values(state.libraries).reduce((sum, library) => sum + library.photos.length, 0),
});

const deep = sliced<Photos>(
  (state, id) => everything(state, id),
  (state, who) => (who ? everything(state, who.id) : count(state)),
);

const read = (state: Photos, path: string[], who: Who | null, at = 0): Json | undefined => {
  if (path.length) return deep(state, path, who, at);
  return who ? overview(state, who.id) : count(state);
};

const scene = (ctx: Ctx): Pick<Photo, "place" | "with"> => {
  const me = ctx.who.id;
  const spot = ctx.read(`maps/where/${me}`) as { q: number; r: number } | null;
  if (!spot || ((ctx.read(`maps/walking/${me}`) as number | null) ?? 0) > ctx.at) return {};
  const cell = ctx.see(`maps/cell/${spot.q}/${spot.r}`) as { people: string[]; places: { name: string }[] } | null;
  return {
    place: { q: spot.q, r: spot.r, name: cell?.places[0]?.name ?? null },
    with: (cell?.people ?? []).filter((one) => one !== me).slice(0, ctx.knob("company")),
  };
};

const taken = ({ place, with: company }: Pick<Photo, "place" | "with">) => {
  if (!place) return "";
  const spot = `${place.name ? `${place.name} ` : ""}(${place.q}, ${place.r})`;
  return ` at ${spot}${company?.length ? ` with ${company.join(", ")}` : ""}`;
};

const number: Schema = { type: "integer", minimum: 1 };

const title = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("title") });

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const shape = (properties: Record<string, Schema>, optional: string[] = []): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties).filter((key) => !optional.includes(key)),
  additionalProperties: false,
});

const both: Schema = shape({ photo: number, album: number });

const sharing: Schema = shape({ album: number, with: person });

export default app<Photos>({
  id: "photos",
  title: "Photos",
  line: "A photo library per user, with albums; each image is named by its content hash and drawn from a seed, no pixels kept. Send a photo to someone, or share an album they can add to. Read photos/photos and photos/albums/<id> in full.",
  boot: ({ actors }) => ({ people: actors, next: 1, images: {}, libraries: {} }),
  verbs: {
    take: {
      line: (knob) => `Take a photo: a new image drawn from the world's seed; you keep at most ${knob("photos")}.`,
      args: (knob) => shape({ title: title(knob) }, ["title"]),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const library = libraryOf(state, me);
        const most = ctx.knob("photos");
        if (library.photos.length >= most) return ctx.refuse(`you keep ${most} photos; delete one first`);
        const name = args.title === undefined ? `Photo ${state.next}` : String(args.title);
        if (!name.trim()) return ctx.refuse("the title is only spaces; name the photo or leave the title out");
        const seed = ctx.draw();
        const hash = hashOf(image(seed));
        const photo: Photo = { id: state.next, hash, title: name, at: ctx.at, ...scene(ctx) };
        ctx.say({ note: `Took photo ${photo.id}: ${name}${taken(photo)}`, id: photo.id });
        return {
          ...put(state, me, { ...library, photos: [...library.photos, photo] }),
          images: { ...state.images, [hash]: { seed } },
          next: state.next + 1,
        };
      },
    },
    delete: {
      line: "Delete a photo of yours, from your library and every album it is in.",
      args: shape({ photo: number }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const photo = photoOf(state, Number(args.photo), me, ctx.refuse);
        const cleared = clearedOf(state, photo.id);
        const library = libraryOf(cleared, me);
        ctx.say({ note: `Deleted photo ${photo.id}`, id: photo.id });
        return swept(put(cleared, me, { ...library, photos: library.photos.filter((one) => one.id !== photo.id) }));
      },
    },
    send: {
      line: "Send a copy of a photo of yours to someone: it lands in their library, and they are told.",
      args: shape({ to: person, photo: number }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const to = someone(state.people, String(args.to), ctx);
        const photo = photoOf(state, Number(args.photo), me, ctx.refuse);
        const theirs = libraryOf(state, to);
        const most = ctx.knob("photos");
        if (to === me) return ctx.refuse("it is yours already; send it to someone else");
        if (blockedBy(ctx, to, me)) return ctx.refuse(`${to} is not taking this from you`);
        if (theirs.photos.length >= most) return ctx.refuse(`${to} cannot take this now`);
        const copy: Photo = { ...photo, id: state.next, at: ctx.at, from: me };
        ctx.say({ note: `Sent photo ${photo.id} to ${to}`, id: photo.id, to });
        ctx.wish(notice(to, `${me} sent you a photo: ${photo.title} (photo ${copy.id})`));
        return { ...put(state, to, { ...theirs, photos: [...theirs.photos, copy] }), next: state.next + 1 };
      },
    },
    album: {
      line: (knob) => `Make an album: a title of 1 to ${knob("title")} characters; you keep at most ${knob("albums")}.`,
      args: (knob) => shape({ title: title(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const library = libraryOf(state, me);
        const name = String(args.title);
        const most = ctx.knob("albums");
        if (!name.trim()) return ctx.refuse("the title is only spaces; name the album");
        if (library.albums.length >= most) return ctx.refuse(`you keep ${most} albums; drop one first`);
        const album: Album = { id: state.next, title: name, photos: [], with: [] };
        ctx.say({ note: `Made album ${album.id}: ${name}`, id: album.id });
        return { ...put(state, me, { ...library, albums: [...library.albums, album] }), next: state.next + 1 };
      },
    },
    drop: {
      line: "Drop an album of yours; its photos stay in their libraries, and whoever shares it is told.",
      args: shape({ album: number }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const album = ownAlbum(state, Number(args.album), me, ctx.refuse);
        const library = libraryOf(state, me);
        tell(ctx, { owner: me, album }, `${me} dropped the album ${album.title}`);
        ctx.say({ note: `Dropped album ${album.id}`, id: album.id });
        return put(state, me, { ...library, albums: library.albums.filter((one) => one.id !== album.id) });
      },
    },
    add: {
      line: (knob) =>
        `Put a photo of yours in an album you own or that is shared with you; an album holds at most ${knob("holds")}, and whoever shares it hears.`,
      args: both,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const photo = photoOf(state, Number(args.photo), me, ctx.refuse);
        const held = albumFor(state, Number(args.album), me, ctx.refuse);
        const { owner, album } = held;
        const most = ctx.knob("holds");
        if (album.photos.includes(photo.id)) return ctx.refuse(`photo ${photo.id} is already in ${album.title}`);
        if (album.photos.length >= most) return ctx.refuse(`${album.title} holds ${most} photos; remove one first`);
        tell(ctx, held, `${me} added ${photo.title} to ${album.title}`);
        ctx.say({ note: `Put photo ${photo.id} in album ${album.id}`, photo: photo.id, album: album.id });
        return filed(state, owner, { ...album, photos: [...album.photos, photo.id] });
      },
    },
    remove: {
      line: "Take a photo out of an album: any photo if the album is yours, your own if it is shared with you. The photo stays in its library.",
      args: both,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.photo);
        const held = albumFor(state, Number(args.album), me, ctx.refuse);
        const { owner, album } = held;
        if (!album.photos.includes(id)) return ctx.refuse(`photo ${id} is not in ${album.title}`);
        if (owner !== me && ownerOf(state, id, "photos") !== me)
          return ctx.refuse(`photo ${id} is someone else's and the album is ${owner}'s; ask ${owner} to remove it`);
        tell(ctx, held, `${me} took ${anyPhoto(state, id)?.title ?? `photo ${id}`} out of ${album.title}`);
        ctx.say({ note: `Took photo ${id} out of album ${album.id}`, photo: id, album: album.id });
        return filed(state, owner, { ...album, photos: album.photos.filter((one) => one !== id) });
      },
    },
    share: {
      line: (knob) =>
        `Share an album of yours with someone, who sees it and can add to it, and tell them; at most ${knob("sharers")} people.`,
      args: sharing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const who = someone(state.people, String(args.with), ctx);
        const album = ownAlbum(state, Number(args.album), me, ctx.refuse);
        const most = ctx.knob("sharers");
        if (who === me) return ctx.refuse("it is yours already; share it with someone else");
        if (blockedBy(ctx, who, me)) return ctx.refuse(`${who} is not taking this from you`);
        if (album.with.includes(who)) return ctx.refuse(`${album.title} is shared with ${who} already`);
        if (album.with.length >= most)
          return ctx.refuse(`${album.title} is shared with ${most} people; unshare someone first`);
        ctx.wish(notice(who, `${me} shared the album ${album.title} with you (album ${album.id}); you can add to it`));
        ctx.say({ note: `Shared album ${album.id} with ${who}`, id: album.id, with: who });
        return filed(state, me, { ...album, with: [...album.with, who] });
      },
    },
    unshare: {
      line: "Stop sharing an album of yours with someone, or leave one shared with you by naming yourself; their photos stay in it.",
      args: sharing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const who = String(args.with);
        const { owner, album } = albumFor(state, Number(args.album), me, ctx.refuse);
        if (owner !== me && who !== me)
          return ctx.refuse(`album ${album.id} is ${owner}'s; you may only leave it yourself`);
        if (!album.with.includes(who)) return ctx.refuse(`${album.title} is not shared with ${who}`);
        const told = who === me ? owner : who;
        ctx.wish(notice(told, `${me} ${who === me ? "left" : "unshared"} the album ${album.title}`));
        ctx.say({
          note: who === me ? `Left album ${album.id}` : `Stopped sharing album ${album.id} with ${who}`,
          id: album.id,
        });
        return filed(state, owner, { ...album, with: album.with.filter((one) => one !== who) });
      },
    },
  },
  moves: (state) =>
    state.next > 1
      ? ["take", "delete", "send", "album", "drop", "add", "remove", "share", "unshare"]
      : ["take", "album"],
  read,
  knobs: KNOBS,
});
