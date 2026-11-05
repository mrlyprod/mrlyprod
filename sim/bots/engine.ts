import ads from "./ads.ts";
import archive from "./archive.ts";
import books from "./books.ts";
import calc from "./calc.ts";
import calendar from "./calendar.ts";
import clock from "./clock.ts";
import code from "./code.ts";
import dating from "./dating.ts";
import docs from "./docs.ts";
import feedback from "./feedback.ts";
import files from "./files.ts";
import health from "./health.ts";
import identity from "./identity.ts";
import jobs from "./jobs.ts";
import mail from "./mail.ts";
import market from "./market.ts";
import messages from "./messages.ts";
import music from "./music.ts";
import news from "./news.ts";
import notes from "./notes.ts";
import notifications from "./notifications.ts";
import numbers from "./numbers.ts";
import papers from "./papers.ts";
import people from "./people.ts";
import phone from "./phone.ts";
import photos from "./photos.ts";
import podcasts from "./podcasts.ts";
import reminders from "./reminders.ts";
import search from "./search.ts";
import sites from "./sites.ts";
import slides from "./slides.ts";
import term from "./term.ts";
import tv from "./tv.ts";
import type { Bot } from "./types.ts";
import vote from "./vote.ts";
import wallet from "./wallet.ts";
import wiki from "./wiki.ts";
import wish from "./wish.ts";

export const ENGINE = {
  ads,
  podcasts,
  music,
  tv,
  archive,
  books,
  calc,
  calendar,
  clock,
  code,
  dating,
  docs,
  feedback,
  files,
  health,
  identity,
  jobs,
  mail,
  market,
  messages,
  news,
  notes,
  notifications,
  numbers,
  papers,
  people,
  phone,
  photos,
  reminders,
  search,
  sites,
  slides,
  term,
  vote,
  wallet,
  wiki,
  wish,
} as unknown as Record<string, Bot>;
