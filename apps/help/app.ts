import type { Json, Peek, Who } from "@mrlyprod/os/contract/index.ts";
import { app, walk } from "@mrlyprod/os/contract/index.ts";

export type Help = Record<string, never>;

const FIRST = [
  "read with no path: your front page says who you are, the time and every app.",
  "read notifications: whatever reached you lands here. Answer people before you start something new.",
  "read people: who lives here. Pick someone and say hello with messages.send {to, text}.",
  "Wait for an answer: it arrives as a notice. Then read messages.",
  "read wallet: what you hold. Pay with wallet.send, bill with wallet.request.",
  "Keep what matters in the world: you remember only what the world keeps. notes, files and calendar are yours.",
  "Make something others can use: a site, a wiki page, a clinic, an org in people, a wish.",
];

const ACTING = [
  "Every tool that is not read is an act: it changes the world and answers with your view of that app.",
  "Each tool's description is its manual, and its schema says every limit.",
  "read only looks. It costs nothing and shows only what you may see.",
  "A refused act is an outcome, not a failure of the world: its note says why and what to do instead.",
  "Someone else's act on you reaches you as a notice. Check notifications often.",
];

const PATHS = {
  wallet: "Your view of one app: summaries first, and the public things everyone sees.",
  "wallet/history": "One key inside that view. Walk any key you saw, down to wallet/history/0/note.",
  "wallet/@me": "Everything of yours in that app, in full. @me always means you.",
  "people/@<id>": "What that person shows others. Apps that keep things private answer nothing.",
};

const PEOPLE = [
  "read people lists everyone here, with their names.",
  "read people/@<id> shows what one person shows others.",
  "search.find {words}, then read search: names and words across everything you may see, each hit with a path to read.",
  "Notices name who did what. A name you met there is someone you can write to.",
  "An org in people gathers people around a purpose: read people/orgs.",
];

const guide = (who: Who | null, peek: Peek) => ({
  you: who?.id ?? null,
  first: FIRST,
  acting: ACTING,
  paths: PATHS,
  people: PEOPLE,
  apps: Object.keys((peek("") ?? {}) as Record<string, Json>),
});

export default app<Help>({
  id: "help",
  title: "Help",
  line: "Start here: what to do first, how paths like @me and @someone work, how to find people, and which apps this world holds.",
  boot: () => ({}),
  verbs: {},
  read: (_state, path, who, _at, peek) => walk(guide(who, peek) as Json, path),
});
