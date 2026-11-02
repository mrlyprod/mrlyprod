import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Books } from "./app.ts";

/* WORDS */

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

const TEXTS = [
  "The day began early. The market was already loud by the time the bread came out.",
  "Rain fell for three days. The river rose to the second step of the bridge.",
  "A letter came from the coast. It said little, and all of it mattered.",
  "The road to the mill is longer in winter. Nobody knows why.",
];

/* POLICY */

export const bot: Bot<Books> = ({ state, me, draw }): Step => {
  const books = Object.values(state.books);
  const mine = books.filter((one) => one.by === me);
  const shelf = state.shelves[me] ?? {};
  if (!mine.length || (mine.length < 2 && !below(draw, 8)))
    return { verb: "write", args: { title: pick(draw, TITLES) } };
  const short = mine.filter((one) => one.chapters.length < 5);
  if (short.length && below(draw, 3) < 2)
    return {
      verb: "chapter",
      args: { book: pick(draw, short).id, title: pick(draw, CHAPTERS), text: pick(draw, TEXTS) },
    };
  const unread = books.filter((one) => one.by !== me && !(one.id in shelf));
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
