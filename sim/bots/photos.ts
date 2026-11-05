import type { Photos } from "@mrlyprod/apps/photos/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const ALBUMS = ["Family", "Holiday", "Garden", "Work", "Friends"];

const TITLES = ["Breakfast", "The park", "Sunset", "The office", "A walk", "Birthday", "The beach", "Snow"];

const bot: Bot<Photos> = ({ state, me, actors, draw }): Step => {
  const library = state.libraries[me] ?? { photos: [], albums: [] };
  const { photos, albums } = library;
  const others = [...new Set([...actors, ...Object.keys(state.libraries)])].filter((one) => one !== me);
  const shared = Object.values(state.libraries).flatMap((one) => one.albums.filter((album) => album.with.includes(me)));
  if (photos.length >= 30) return { verb: "delete", args: { photo: photos[0]!.id } };
  if (photos.length && others.length && !below(draw, 10))
    return { verb: "send", args: { to: pick(draw, others), photo: pick(draw, photos).id } };
  if (albums.length && others.length && !below(draw, 10)) {
    const album = pick(draw, albums);
    const fresh = others.filter((one) => !album.with.includes(one));
    if (album.with.length && !below(draw, 3))
      return { verb: "unshare", args: { album: album.id, with: pick(draw, album.with) } };
    if (fresh.length) return { verb: "share", args: { album: album.id, with: pick(draw, fresh) } };
  }
  if (shared.length && photos.length && !below(draw, 4)) {
    const album = pick(draw, shared);
    const fresh = photos.filter((photo) => !album.photos.includes(photo.id));
    if (fresh.length) return { verb: "add", args: { photo: pick(draw, fresh).id, album: album.id } };
  }
  const fresh = ALBUMS.filter((title) => !albums.some((album) => album.title === title));
  if (fresh.length && (!albums.length || (albums.length < 3 && !below(draw, 12))))
    return { verb: "album", args: { title: pick(draw, fresh) } };
  if (albums.length > 2 && !below(draw, 30)) return { verb: "drop", args: { album: pick(draw, albums).id } };
  const loose = photos.filter((photo) => !albums.some((album) => album.photos.includes(photo.id)));
  if (loose.length && albums.length && below(draw, 2))
    return { verb: "add", args: { photo: pick(draw, loose).id, album: pick(draw, albums).id } };
  const full = albums.filter((album) => album.photos.length);
  if (full.length && !below(draw, 12)) {
    const album = pick(draw, full);
    return { verb: "remove", args: { photo: pick(draw, album.photos), album: album.id } };
  }
  return { verb: "take", args: { title: pick(draw, TITLES) } };
};

export default bot;
