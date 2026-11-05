import type { Books } from "@mrlyprod/apps/books/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import { reader } from "@mrlyprod/os/kernel/index.ts";
import type { Bot, Step } from "./types.ts";

const TITLES = [
  "A Year on the Hill",
  "The Well",
  "Letters Home",
  "Bread and Salt",
  "Maps I Drew",
  "Rain Days",
  "The Mill",
];

const CHAPTERS = ["Morning", "The Market", "Rain", "A Letter", "The Road", "Night", "Spring"];

const VERDICTS = ["Read it twice.", "Slow at the start, worth it at the end.", "Nobody writes rain like this.", ""];

const TEXTS = [
  "The day began early. The market was already loud by the time the bread came out.",
  "Rain fell for three days. The river rose to the second step of the bridge.",
  "A letter came from the coast. It said little, and all of it mattered.",
  "The road to the mill is longer in winter. Nobody knows why.",
];

const bot: Bot<Books> = ({ state, me, actors, draw, world }): Step => {
  const books = Object.values(state.books);
  const own = books.filter((one) => one.by === me);
  const mine = books.filter((one) => one.by === me || one.coauthors.includes(me));
  const shelf = state.shelves[me] ?? {};
  if (!own.length || (own.length < 2 && !below(draw, 8))) return { verb: "write", args: { title: pick(draw, TITLES) } };
  const room = own.filter(
    (one) => one.coauthors.length < reader(world.rules, "books")<number>("authors") && one.chapters.length,
  );
  const friends = actors.filter((one) => one !== me);
  if (room.length && friends.length && !below(draw, 6)) {
    const one = pick(draw, room);
    const free = friends.filter((each) => !one.coauthors.includes(each));
    if (free.length) return { verb: "invite", args: { book: one.id, person: pick(draw, free) } };
  }
  const written = mine.filter((one) => one.chapters.length && one.chapters.some((each) => each.by === me));
  if (written.length && !below(draw, 12)) {
    const one = pick(draw, written);
    const chapter = pick(
      draw,
      one.chapters.filter((each) => each.by === me),
    );
    return { verb: "edit", args: { book: one.id, chapter: chapter.n, text: pick(draw, TEXTS) } };
  }
  const short = mine.filter((one) => one.chapters.length < 5);
  if (short.length && below(draw, 3) < 2)
    return {
      verb: "chapter",
      args: { book: pick(draw, short).id, title: pick(draw, CHAPTERS), text: pick(draw, TEXTS) },
    };
  const reviewable = books.filter(
    (one) => !mine.includes(one) && one.chapters.length && shelf[one.id] && !one.reviews.some((each) => each.by === me),
  );
  if (reviewable.length && !below(draw, 4))
    return {
      verb: "review",
      args: { book: pick(draw, reviewable).id, stars: 1 + below(draw, 2) + below(draw, 3), text: pick(draw, VERDICTS) },
    };
  const unread = books.filter((one) => !mine.includes(one) && !(one.id in shelf));
  if (unread.length && below(draw, 2)) return { verb: "shelve", args: { book: pick(draw, unread).id } };
  const open = Object.entries(shelf)
    .map(([id, at]) => ({ book: state.books[id]!, at }))
    .filter(({ book, at }) => at < book.chapters.length);
  if (open.length) {
    const { book, at } = pick(draw, open);
    return { verb: "read", args: { book: book.id, chapter: at + 1 } };
  }
  const done = Object.keys(shelf).map(Number);
  return done.length > 6 ? { verb: "unshelve", args: { book: pick(draw, done) } } : null;
};

export default bot;
