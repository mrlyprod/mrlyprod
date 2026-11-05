import type { Media } from "@mrlyprod/apps/shared/media.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";
import { dialed } from "./types.ts";

export const media =
  (id: string, titles: readonly string[]): Bot<Media> =>
  ({ state, me, actors, draw, world }): Step => {
    const knob = dialed(id);
    const ids = Object.keys(state.titles).map(Number);
    const library = state.library[me] ?? [];
    const queue = state.queue[me] ?? [];
    const own = Object.values(state.titles).filter((title) => title.by === me);
    const makers = Object.values(state.titles).map((title) => title.by);
    const following = state.follows[me] ?? [];
    const others = [...new Set([...actors, ...makers])].filter((one) => one !== me && !following.includes(one));
    if (queue.length && below(draw, 2)) return { verb: "next", args: {} };
    if (!ids.length || (own.length < 3 && !below(draw, 5)))
      return { verb: "publish", args: { title: pick(draw, titles), minutes: 3 + below(draw, 58) } };
    const fresh = ids.filter((id) => !library.includes(id));
    const turn = below(draw, 9);
    if (turn === 0 && fresh.length && library.length < knob(world.rules, "library"))
      return { verb: "add", args: { title: pick(draw, fresh) } };
    if (turn === 1 && queue.length < knob(world.rules, "queue"))
      return { verb: "queue", args: { title: pick(draw, ids) } };
    if (turn === 5 && library.length > 5) return { verb: "remove", args: { title: pick(draw, library) } };
    if (turn === 6 && others.length && following.length < 4)
      return { verb: "follow", args: { person: pick(draw, others) } };
    if (turn === 7 && own.length) {
      const one = pick(draw, own);
      return own.length > 2 && !below(draw, 3)
        ? { verb: "unpublish", args: { title: one.id } }
        : {
            verb: "edit",
            args: { title: one.id, about: `Now with ${1 + below(draw, 9)} more minutes of ${one.title}.` },
          };
    }
    if (turn === 8 && following.length > 2) return { verb: "unfollow", args: { person: pick(draw, following) } };
    if (turn < 5) return { verb: "play", args: { title: pick(draw, library.length ? library : ids) } };
    return null;
  };
