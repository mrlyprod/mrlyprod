import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Photos } from "./app.ts";

/* WORDS */

const ALBUMS = ["Family", "Holiday", "Garden", "Work", "Friends"];

const TITLES = ["Breakfast", "The park", "Sunset", "The office", "A walk", "Birthday", "The beach", "Snow"];

/* BOT */

const bot: Bot<Photos> = ({ state, me, draw }): Step => {
  const library = state.libraries[me] ?? { photos: [], albums: [] };
  const { photos, albums } = library;
  if (photos.length >= 30) return { verb: "delete", args: { photo: photos[0]!.id } };
  const fresh = ALBUMS.filter((title) => !albums.some((album) => album.title === title));
  if (fresh.length && (!albums.length || (albums.length < 3 && !below(draw, 12))))
    return { verb: "album", args: { title: pick(draw, fresh) } };
  const loose = photos.filter((photo) => !albums.some((album) => album.photos.includes(photo.id)));
  if (loose.length && below(draw, 2))
    return { verb: "add", args: { photo: pick(draw, loose).id, album: pick(draw, albums).id } };
  const full = albums.filter((album) => album.photos.length);
  if (full.length && !below(draw, 12)) {
    const album = pick(draw, full);
    return { verb: "remove", args: { photo: pick(draw, album.photos), album: album.id } };
  }
  return { verb: "take", args: { title: pick(draw, TITLES) } };
};

export default bot;
