import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";
import { hashOf, image } from "./image.ts";

/* TYPES */

export type Photo = { id: number; hash: string; title: string; at: number };

export type Album = { id: number; title: string; photos: number[] };

export type Library = { photos: Photo[]; albums: Album[] };

export type Photos = { next: number; images: Record<string, { seed: number }>; libraries: Record<string, Library> };

export const TITLE = 60;

export const PHOTOS = 100;

export const ALBUMS = 20;

/* LIBRARIES */

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
  if (owner !== me) return refuse(`photo ${id} is ${owner}'s`);
  return libraryOf(state, me).photos.find((one) => one.id === id)!;
}

function albumOf(state: Photos, id: number, me: string, refuse: Ctx["refuse"]): Album {
  const owner = ownerOf(state, id, "albums");
  if (!owner) return refuse(`no album ${id}`);
  if (owner !== me) return refuse(`album ${id} is ${owner}'s`);
  return libraryOf(state, me).albums.find((one) => one.id === id)!;
}

const filed = (library: Library, album: Album): Library => ({
  ...library,
  albums: library.albums.map((one) => (one.id === album.id ? album : one)),
});

function swept(state: Photos): Photos {
  const used = new Set(Object.values(state.libraries).flatMap((library) => library.photos.map((photo) => photo.hash)));
  return { ...state, images: Object.fromEntries(Object.entries(state.images).filter(([hash]) => used.has(hash))) };
}

/* SCHEMAS */

const number = (state: Photos): Schema => ({ type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) });

const title: Schema = { type: "string", minLength: 1, maxLength: TITLE };

const both = (state: Photos): Schema => ({
  type: "object",
  properties: { photo: number(state), album: number(state) },
  required: ["photo", "album"],
  additionalProperties: false,
});

/* APP */

export default app<Photos>({
  id: "photos",
  title: "Photos",
  line: "A photo library per user, with albums; each image is named by its content hash and drawn from a seed, no pixels kept.",
  boot: () => ({ next: 1, images: {}, libraries: {} }),
  verbs: {
    take: {
      line: `Take a photo: a new image drawn from the world's seed; you keep at most ${PHOTOS}.`,
      args: { type: "object", properties: { title }, additionalProperties: false },
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const library = libraryOf(state, me);
        if (library.photos.length >= PHOTOS) return ctx.refuse(`you keep ${PHOTOS} photos; delete one first`);
        const name = args.title === undefined ? `Photo ${state.next}` : String(args.title);
        if (!name.trim()) return ctx.refuse("the title is only spaces; name the photo or leave the title out");
        const seed = ctx.draw();
        const hash = hashOf(image(seed));
        const photo: Photo = { id: state.next, hash, title: name, at: ctx.at };
        return {
          ...put(state, me, { ...library, photos: [...library.photos, photo] }),
          images: { ...state.images, [hash]: { seed } },
          next: state.next + 1,
        };
      },
    },
    delete: {
      line: "Delete a photo of yours, from your library and every album of yours.",
      args: (state) => ({
        type: "object",
        properties: { photo: number(state) },
        required: ["photo"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const photo = photoOf(state, Number(args.photo), me, ctx.refuse);
        const library = libraryOf(state, me);
        return swept(
          put(state, me, {
            photos: library.photos.filter((one) => one.id !== photo.id),
            albums: library.albums.map((album) => ({ ...album, photos: album.photos.filter((id) => id !== photo.id) })),
          }),
        );
      },
    },
    album: {
      line: `Make an album: a title of 1 to ${TITLE} characters; you keep at most ${ALBUMS}.`,
      args: { type: "object", properties: { title }, required: ["title"], additionalProperties: false },
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const library = libraryOf(state, me);
        const name = String(args.title);
        if (!name.trim()) return ctx.refuse("the title is only spaces; name the album");
        if (library.albums.length >= ALBUMS) return ctx.refuse(`you keep ${ALBUMS} albums; that is the most`);
        const album: Album = { id: state.next, title: name, photos: [] };
        return { ...put(state, me, { ...library, albums: [...library.albums, album] }), next: state.next + 1 };
      },
    },
    add: {
      line: "Put a photo of yours in an album of yours.",
      args: both,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const photo = photoOf(state, Number(args.photo), me, ctx.refuse);
        const album = albumOf(state, Number(args.album), me, ctx.refuse);
        if (album.photos.includes(photo.id)) return ctx.refuse(`photo ${photo.id} is already in ${album.title}`);
        return put(state, me, filed(libraryOf(state, me), { ...album, photos: [...album.photos, photo.id] }));
      },
    },
    remove: {
      line: "Take a photo out of an album of yours; it stays in your library.",
      args: both,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const photo = photoOf(state, Number(args.photo), me, ctx.refuse);
        const album = albumOf(state, Number(args.album), me, ctx.refuse);
        if (!album.photos.includes(photo.id)) return ctx.refuse(`photo ${photo.id} is not in ${album.title}`);
        return put(
          state,
          me,
          filed(libraryOf(state, me), { ...album, photos: album.photos.filter((id) => id !== photo.id) }),
        );
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["take", "delete", "album", "add", "remove"] : ["take", "album"]),
  read: sliced(libraryOf),
  bot,
});
