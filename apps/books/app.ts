import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, reach, someone } from "../shared/people.ts";

export type Chapter = { n: number; title: string; text: string; at: number; by: string };

export type Review = { by: string; stars: number; text: string; at: number };

export type Book = {
  id: number;
  title: string;
  by: string;
  coauthors: string[];
  at: number;
  chapters: Chapter[];
  reviews: Review[];
};

export type Books = {
  people: string[] | null;
  listed: number;
  next: number;
  books: Record<string, Book>;
  shelves: Record<string, Record<string, number>>;
  reading: Record<string, { book: number; chapter: number }>;
};

const KNOBS = {
  title: { line: "Characters one title holds.", schema: { type: "integer", minimum: 10, maximum: 1000 }, reality: 100 },
  text: {
    line: "Characters one chapter holds.",
    schema: { type: "integer", minimum: 100, maximum: 100000 },
    reality: 8000,
  },
  own: { line: "Books one person may write.", schema: { type: "integer", minimum: 1, maximum: 200 }, reality: 10 },
  chapters: { line: "Chapters one book holds.", schema: { type: "integer", minimum: 1, maximum: 500 }, reality: 50 },
  shelf: { line: "Books one shelf holds.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 50 },
  review: {
    line: "Characters one review holds.",
    schema: { type: "integer", minimum: 10, maximum: 5000 },
    reality: 500,
  },
  reviews: { line: "Reviews one book keeps.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 50 },
  stars: { line: "The most stars a review gives.", schema: { type: "integer", minimum: 2, maximum: 10 }, reality: 5 },
  authors: {
    line: "Co-authors one book may have besides its first author; 0 means every book has one author.",
    schema: { type: "integer", minimum: 0, maximum: 50 },
    reality: 4,
  },
  listed: {
    line: "Newest books one overview lists.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
} satisfies Knobs;

const bookOf = (state: Books, id: number) => state.books[id]!;

const authorsOf = (one: Book) => [one.by, ...one.coauthors];

const found = (state: Books, id: number, refuse: Ctx["refuse"]): Book =>
  state.books[id] ?? refuse(`no book ${id} yet; start one with books.write`);

const authored = (state: Books, id: number, me: string, refuse: Ctx["refuse"]): Book => {
  const one = found(state, id, refuse);
  return authorsOf(one).includes(me)
    ? one
    : refuse(`book ${id} is by ${authorsOf(one).join(", ")}; only its authors change it`);
};

const owned = (state: Books, id: number, me: string, refuse: Ctx["refuse"]): Book => {
  const one = found(state, id, refuse);
  return one.by === me ? one : refuse(`book ${id} is ${one.by}'s; only they invite or remove co-authors`);
};

const tell = (ctx: Ctx, people: string[], text: string) => {
  for (const who of people) if (who !== ctx.who.id) ctx.wish(notice(who, text));
};

const shelf = (state: Books, me: string) => state.shelves[me] ?? {};

const shelved = (state: Books, me: string, id: number, chapter: number): Books => ({
  ...state,
  shelves: { ...state.shelves, [me]: { ...shelf(state, me), [id]: chapter } },
});

const readersOf = (state: Books, id: number) => Object.keys(state.shelves).filter((who) => id in state.shelves[who]!);

const rating = (one: Book) =>
  one.reviews.length
    ? Math.round((10 * one.reviews.reduce((sum, review) => sum + review.stars, 0)) / one.reviews.length) / 10
    : null;

const card = (one: Book) => ({
  id: one.id,
  title: one.title,
  by: one.by,
  authors: authorsOf(one),
  at: one.at,
  chapters: one.chapters.length,
  rating: rating(one),
  reviews: one.reviews.length,
});

const open = (one: Book) => ({
  ...card(one),
  chapters: Object.fromEntries(one.chapters.map((chapter) => [chapter.n, chapter])),
  reviews: one.reviews,
});

const newest = (state: Books) =>
  Object.values(state.books)
    .sort((a, b) => b.at - a.at || b.id - a.id)
    .slice(0, state.listed)
    .map(card);

const started = (state: Books, ctx: Ctx, one: Book) => {
  const me = ctx.who.id;
  if (!authorsOf(one).includes(me) && !(one.id in shelf(state, me)))
    tell(ctx, authorsOf(one), `${me} took up your book: ${one.title}`);
};

const place = (state: Books, me: string): Json => {
  const at = state.reading[me];
  const chapter = at && bookOf(state, at.book).chapters[at.chapter - 1];
  return at && chapter
    ? { book: at.book, of: bookOf(state, at.book).title, chapter: chapter.n, title: chapter.title, text: chapter.text }
    : null;
};

const mine = (state: Books, me: string) => ({
  reading: place(state, me),
  wrote: Object.values(state.books)
    .filter((one) => authorsOf(one).includes(me))
    .map((one) => ({ ...card(one), readers: readersOf(state, one.id) })),
  shelf: Object.entries(shelf(state, me)).map(([id, at]) => {
    const one = bookOf(state, Number(id));
    return { id: one.id, title: one.title, by: one.by, authors: authorsOf(one), chapters: one.chapters.length, at };
  }),
  latest: newest(state),
});

const whole = (state: Books, who: Who | null): Json => ({
  ...(who ? mine(state, who.id) : { latest: newest(state) }),
  books: Object.fromEntries(Object.values(state.books).map((one) => [one.id, open(one)])),
});

const deep = sliced<Books>(mine, whole);

const read = (state: Books, path: string[], who: Who | null, at = 0): Json | undefined => {
  if (path.length) return deep(state, path, who, at);
  return who ? mine(state, who.id) : { latest: newest(state), total: Object.keys(state.books).length };
};

const title = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("title") });

const naming = (more: Record<string, Schema> = {}, optional: string[] = []): Schema => ({
  type: "object",
  properties: { book: { type: "integer", minimum: 1 }, ...more },
  required: ["book", ...Object.keys(more).filter((key) => !optional.includes(key))],
  additionalProperties: false,
});

export default app<Books>({
  id: "books",
  title: "Books",
  line: "Books written inside the world, alone or with co-authors, chapter by chapter, readable by all: a shelf and a reading position each, reviews and stars for the authors. Read books/books/<id>/chapters/<n> for any chapter.",
  boot: ({ knob, actors }) => ({
    people: actors,
    listed: knob("listed"),
    next: 1,
    books: {},
    shelves: {},
    reading: {},
  }),
  verbs: {
    write: {
      line: (knob) =>
        `Start a book of yours: a title of 1 to ${knob("title")} characters; you write at most ${knob("own")}.`,
      args: (knob) => ({
        type: "object",
        properties: { title: title(knob) },
        required: ["title"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.title).trim();
        const most = ctx.knob("own");
        if (!text) return ctx.refuse("the title is only spaces; name the book");
        if (Object.values(state.books).filter((one) => one.by === me).length >= most)
          return ctx.refuse(`you wrote ${most} books, the most one person may`);
        const id = state.next;
        ctx.say({ note: `Started book ${id}: ${text}. Add to it with books.chapter`, id });
        return {
          ...state,
          next: id + 1,
          books: {
            ...state.books,
            [id]: { id, title: text, by: me, coauthors: [], at: ctx.at, chapters: [], reviews: [] },
          },
        };
      },
    },
    chapter: {
      line: (knob) =>
        `Add a chapter to a book you write, alone or with others: a title and a text of 1 to ${knob("text")} characters; a book holds at most ${knob("chapters")}. Its other authors and whoever has it on their shelf hear.`,
      args: (knob) => naming({ title: title(knob), text: { type: "string", minLength: 1, maxLength: knob("text") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = authored(state, Number(args.book), me, ctx.refuse);
        const name = String(args.title).trim();
        const text = String(args.text);
        const most = ctx.knob("chapters");
        if (!name) return ctx.refuse("the chapter title is only spaces; name it");
        if (!text.trim()) return ctx.refuse("the chapter is only spaces; write something");
        if (one.chapters.length >= most) return ctx.refuse(`${one.title} has ${most} chapters, the most a book holds`);
        const chapter: Chapter = { n: one.chapters.length + 1, title: name, text, at: ctx.at, by: me };
        const authors = authorsOf(one);
        tell(ctx, authors, `${me} added chapter ${chapter.n} to ${one.title}: ${name}`);
        tell(
          ctx,
          readersOf(state, one.id).filter((who) => !authors.includes(who)),
          `New chapter ${chapter.n} of ${one.title}: ${name}`,
        );
        ctx.say({ note: `Added chapter ${chapter.n} to ${one.title}`, id: one.id, chapter: chapter.n });
        return { ...state, books: { ...state.books, [one.id]: { ...one, chapters: [...one.chapters, chapter] } } };
      },
    },
    edit: {
      line: (knob) =>
        `Change a chapter of a book you write, alone or with others: a new title, a new text of 1 to ${knob("text")} characters, or both. Its other authors hear.`,
      args: (knob) =>
        naming(
          {
            chapter: { type: "integer", minimum: 1, maximum: knob("chapters") },
            title: title(knob),
            text: { type: "string", minLength: 1, maxLength: knob("text") },
          },
          ["title", "text"],
        ),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = authored(state, Number(args.book), me, ctx.refuse);
        const n = Number(args.chapter);
        const had = one.chapters[n - 1];
        if (!had) return ctx.refuse(`${one.title} has ${one.chapters.length} chapters, no chapter ${n}`);
        const name = args.title === undefined ? had.title : String(args.title).trim();
        const text = args.text === undefined ? had.text : String(args.text);
        if (!name) return ctx.refuse("the chapter title is only spaces; name it");
        if (!text.trim()) return ctx.refuse("the chapter is only spaces; write something");
        if (args.title === undefined && args.text === undefined)
          return ctx.refuse("send a new title, a new text, or both");
        const chapters = one.chapters.map((each) => (each.n === n ? { ...had, title: name, text } : each));
        tell(ctx, authorsOf(one), `${me} changed chapter ${n} of ${one.title}`);
        ctx.say({ note: `Changed chapter ${n} of ${one.title}`, id: one.id, chapter: n });
        return { ...state, books: { ...state.books, [one.id]: { ...one, chapters } } };
      },
    },
    invite: {
      line: (knob) =>
        `Invite someone to write your book with you, up to ${knob("authors")} co-authors besides you. They add and edit chapters and are credited as authors; they hear of it.`,
      args: naming({ person: { type: "string", minLength: 1, maxLength: 40 } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = owned(state, Number(args.book), me, ctx.refuse);
        const who = someone(state.people, String(args.person), ctx);
        const most = ctx.knob("authors");
        if (who === me) return ctx.refuse("you wrote it; invite someone else");
        if (one.coauthors.includes(who)) return ctx.refuse(`${who} writes ${one.title} with you already`);
        reach(ctx, who);
        if (one.coauthors.length >= most)
          return ctx.refuse(
            most
              ? `${one.title} has ${most} co-authors, the most a book holds; books.uninvite one`
              : "every book has one author here",
          );
        tell(ctx, [who], `${me} invited you to write ${one.title} with them: books.chapter book ${one.id}`);
        ctx.say({ note: `${who} now writes ${one.title} with you`, id: one.id, person: who });
        return {
          ...state,
          books: { ...state.books, [one.id]: { ...one, coauthors: [...one.coauthors, who] } },
        };
      },
    },
    uninvite: {
      line: "Remove a co-author from your book, or leave a book you co-write. Chapters they wrote stay. They hear of it.",
      args: naming({ person: { type: "string", minLength: 1, maxLength: 40 } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = found(state, Number(args.book), ctx.refuse);
        const who = String(args.person);
        if (one.by !== me && who !== me)
          return ctx.refuse(
            `book ${one.id} is ${one.by}'s; only they remove co-authors, and you may only remove yourself`,
          );
        if (who === one.by) return ctx.refuse("the first author stays; nobody removes them");
        if (!one.coauthors.includes(who)) return ctx.refuse(`${who} is not a co-author of ${one.title}`);
        tell(
          ctx,
          authorsOf(one).filter((each) => each !== who),
          `${me} ${who === me ? "left" : `removed ${who} from`} ${one.title}`,
        );
        tell(ctx, [who], `${me} removed you from ${one.title}`);
        ctx.say({ note: `${who} no longer writes ${one.title}`, id: one.id, person: who });
        return {
          ...state,
          books: { ...state.books, [one.id]: { ...one, coauthors: one.coauthors.filter((each) => each !== who) } },
        };
      },
    },
    shelve: {
      line: (knob) => `Put a book on your shelf, at its start; a shelf holds at most ${knob("shelf")}.`,
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.book);
        const most = ctx.knob("shelf");
        if (id in shelf(state, me)) return ctx.refuse(`book ${id} is on your shelf already`);
        if (Object.keys(shelf(state, me)).length >= most)
          return ctx.refuse(`your shelf holds ${most} books; unshelve one first`);
        started(state, ctx, bookOf(state, id));
        ctx.say({ note: `Shelved ${bookOf(state, id).title}`, id });
        return shelved(state, me, id, 0);
      },
    },
    unshelve: {
      line: "Take a book off your shelf; your place in it is forgotten.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.book);
        if (!(id in shelf(state, me))) return ctx.refuse(`book ${id} is not on your shelf`);
        const kept = { ...shelf(state, me) };
        delete kept[id];
        const { [me]: last, ...reading } = state.reading;
        ctx.say({ note: `Took book ${id} off your shelf`, id });
        return {
          ...state,
          shelves: { ...state.shelves, [me]: kept },
          reading: last && last.book !== id ? { ...reading, [me]: last } : reading,
        };
      },
    },
    read: {
      line: "Read a chapter: its text lands in your books/reading, your place in the book moves there, and the book joins your shelf if it was not on it.",
      args: (knob) => naming({ chapter: { type: "integer", minimum: 1, maximum: knob("chapters") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = bookOf(state, Number(args.book));
        const chapter = Number(args.chapter);
        const most = ctx.knob("shelf");
        if (chapter > one.chapters.length)
          return ctx.refuse(`${one.title} has ${one.chapters.length} chapters, no chapter ${chapter}`);
        if (!(one.id in shelf(state, me)) && Object.keys(shelf(state, me)).length >= most)
          return ctx.refuse(`your shelf holds ${most} books; unshelve one first`);
        started(state, ctx, one);
        const page = one.chapters[chapter - 1]!;
        ctx.say({
          note: `${one.title}, chapter ${chapter}: ${page.title}`,
          id: one.id,
          chapter,
          of: one.chapters.length,
          text: page.text,
        });
        return {
          ...shelved(state, me, one.id, chapter),
          reading: { ...state.reading, [me]: { book: one.id, chapter } },
        };
      },
    },
    review: {
      line: (knob) =>
        `Review a book you have read a chapter of: 1 to ${knob("stars")} stars and up to ${knob("review")} characters. Everyone can read it, the author hears, and reviewing again replaces yours.`,
      args: (knob) =>
        naming(
          {
            stars: { type: "integer", minimum: 1, maximum: knob("stars") },
            text: { type: "string", maxLength: knob("review") },
          },
          ["text"],
        ),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = bookOf(state, Number(args.book));
        const stars = Number(args.stars);
        const text = String(args.text ?? "").trim();
        if (authorsOf(one).includes(me)) return ctx.refuse("you wrote it; leave reviews to readers");
        if (blockedBy(ctx, one.by, me)) return ctx.refuse(`${one.by} is not taking this from you`);
        if (!one.chapters.length) return ctx.refuse(`${one.title} has no chapter yet; wait for one`);
        if (!shelf(state, me)[one.id]) return ctx.refuse(`you have not read ${one.title}; books.read a chapter first`);
        const review: Review = { by: me, stars, text, at: ctx.at };
        const reviews = [...one.reviews.filter((each) => each.by !== me), review].slice(-ctx.knob("reviews"));
        tell(ctx, authorsOf(one), `${me} gave ${one.title} ${stars} stars${text ? `: ${text}` : ""}`);
        ctx.say({ note: `You gave ${one.title} ${stars} stars`, id: one.id });
        return { ...state, books: { ...state.books, [one.id]: { ...one, reviews } } };
      },
    },
  },
  moves: (state) =>
    state.next > 1
      ? ["write", "chapter", "edit", "invite", "uninvite", "shelve", "unshelve", "read", "review"]
      : ["write", "chapter"],
  read,
  knobs: KNOBS,
});
