import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Chapter = { n: number; title: string; text: string; at: number };

export type Book = { id: number; title: string; by: string; at: number; chapters: Chapter[] };

export type Books = { next: number; books: Record<string, Book>; shelves: Record<string, Record<string, number>> };

export const TITLE = 100;

export const TEXT = 8000;

export const OWN = 10;

export const CHAPTERS = 50;

export const SHELF = 50;

/* BOOKS */

const bookOf = (state: Books, id: number) => state.books[id]!;

const owned = (state: Books, id: number, me: string, refuse: Ctx["refuse"]): Book => {
  const one = bookOf(state, id);
  return one.by === me ? one : refuse(`book ${id} is ${one.by}'s; only its author adds chapters`);
};

const shelf = (state: Books, me: string) => state.shelves[me] ?? {};

const shelved = (state: Books, me: string, id: number, chapter: number): Books => ({
  ...state,
  shelves: { ...state.shelves, [me]: { ...shelf(state, me), [id]: chapter } },
});

const mine = (state: Books, me: string) => ({
  wrote: Object.values(state.books)
    .filter((one) => one.by === me)
    .map((one) => ({ id: one.id, title: one.title, chapters: one.chapters.length })),
  shelf: Object.entries(shelf(state, me)).map(([id, at]) => {
    const one = bookOf(state, Number(id));
    return { id: one.id, title: one.title, by: one.by, chapters: one.chapters.length, at };
  }),
});

/* SCHEMAS */

const title: Schema = { type: "string", minLength: 1, maxLength: TITLE };

const writing: Schema = { type: "object", properties: { title }, required: ["title"], additionalProperties: false };

const naming = (state: Books, more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { book: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) }, ...more },
  required: ["book", ...Object.keys(more)],
  additionalProperties: false,
});

/* APP */

export default app<Books>({
  id: "books",
  title: "Books",
  line: "Books written inside the world, chapter by chapter; a shelf and a reading position each.",
  boot: () => ({ next: 1, books: {}, shelves: {} }),
  verbs: {
    write: {
      line: `Start a book of yours: a title of 1 to ${TITLE} characters; you write at most ${OWN}.`,
      args: writing,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.title).trim();
        if (!text) return ctx.refuse("the title is only spaces; name the book");
        if (mine(state, me).wrote.length >= OWN) return ctx.refuse(`you wrote ${OWN} books, the most one person may`);
        const id = state.next;
        return {
          ...state,
          next: id + 1,
          books: { ...state.books, [id]: { id, title: text, by: me, at: ctx.at, chapters: [] } },
        };
      },
    },
    chapter: {
      line: `Add a chapter to a book of yours: a title and a text of 1 to ${TEXT} characters; a book holds at most ${CHAPTERS}.`,
      args: (state) => naming(state, { title, text: { type: "string", minLength: 1, maxLength: TEXT } }),
      run: (state, args, ctx) => {
        const one = owned(state, Number(args.book), ctx.who.id, ctx.refuse);
        const name = String(args.title).trim();
        const text = String(args.text);
        if (!name) return ctx.refuse("the chapter title is only spaces; name it");
        if (!text.trim()) return ctx.refuse("the chapter is only spaces; write something");
        if (one.chapters.length >= CHAPTERS)
          return ctx.refuse(`${one.title} has ${CHAPTERS} chapters, the most a book holds`);
        const chapter: Chapter = { n: one.chapters.length + 1, title: name, text, at: ctx.at };
        return { ...state, books: { ...state.books, [one.id]: { ...one, chapters: [...one.chapters, chapter] } } };
      },
    },
    shelve: {
      line: `Put a book on your shelf, at its start; a shelf holds at most ${SHELF}.`,
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.book);
        if (id in shelf(state, me)) return ctx.refuse(`book ${id} is on your shelf already`);
        if (Object.keys(shelf(state, me)).length >= SHELF)
          return ctx.refuse(`your shelf holds ${SHELF} books; unshelve one first`);
        return shelved(state, me, id, 0);
      },
    },
    unshelve: {
      line: "Take a book off your shelf; your place in it is forgotten.",
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.book);
        if (!(id in shelf(state, me))) return ctx.refuse(`book ${id} is not on your shelf`);
        const kept = { ...shelf(state, me) };
        delete kept[id];
        return { ...state, shelves: { ...state.shelves, [me]: kept } };
      },
    },
    read: {
      line: "Read a chapter: your place in the book moves there, and the book joins your shelf if it was not on it.",
      args: (state) => naming(state, { chapter: { type: "integer", minimum: 1, maximum: CHAPTERS } }),
      run: (state, args, ctx) => {
        const one = bookOf(state, Number(args.book));
        const chapter = Number(args.chapter);
        if (chapter > one.chapters.length)
          return ctx.refuse(`${one.title} has ${one.chapters.length} chapters, no chapter ${chapter}`);
        return shelved(state, ctx.who.id, one.id, chapter);
      },
    },
  },
  moves: (state) => (state.next > 1 ? ["write", "chapter", "shelve", "unshelve", "read"] : ["write"]),
  read: sliced(mine),
  bot,
});
