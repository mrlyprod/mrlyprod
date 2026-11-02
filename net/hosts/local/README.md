# LOCAL

- The host with no window: a world in a folder on this machine, no network, for the doors.
- `apps.ts`: finds a site's apps in its `roots`: a system app (a folder with `app.ts`, its default export installed as is) or a scene app (`scene.ts` and `widget.tsx`, no page), initialises the wasm units, lifts each scene app into a kernel app with `../../sdk/apps.ts`.
- `store.ts`: a world is a folder, `config.json`, `journal.jsonl` and `snapshot.json` every 500 acts; open replays the journal from the last snapshot.
- `faults.jsonl` sits beside the journal: every `Fault` the kernel throws for a player's act (`bad-act`, `unknown-verb`, anything thrown before the journal appends) is one line, `{ tick, who, verb, args, fault, note }`, and the act never reaches the journal; `sim.ts` `guarded` is the table that catches them, and the report counts them (`faults`) and names the five commonest notes (`faulty`).
- Worlds live in `data/net/hosts/local/worlds/<name>/` at the repo root, gitignored and disposable.
- `MRLY_ROOT` picks the site, as for the web host.
- `sim.ts`: `mrlynet sim` on this host: the world folder as a `guarded` table, `../../sim/run.ts` `run` over it (the config's `actors`, `mrlynet sim net/mrly.net/world.json` plays `test-1` to `test-8`, or bots `bot-1` to `bot-N` when it names none, one act each a round, a round a second of world time, never the wall clock), then the report; how the players act is `../../sim/`, never this folder.
- The round is `../../sim/bots.ts` `round`, the code the tab's host hands its bots (`../../shell/world.ts`), so `mrlynet sim` and a tab reach one digest from one seed (`../../shell/world.test.ts`).
- A bot's draw for an act is seeded by the world's seed, the tick and its id (and the round's `at` once an app holds a `bot`), so a run resumes from its journal and two runs of 12 and 18 rounds equal one of 30, unless the first ends on a round where nobody acted.
- The players follow each app's `bot` where it has one (`../../sim/bots.ts`); round `i` acts at the start's `at` plus `i` beats, whether or not anyone acted before it.
- `store.ts` `call` runs the bus (`../../sdk/system.ts` `bus`): the act's notice wishes as `notifications.notice` acts, its act wishes, then each app's `after` (`sim.tick` while the world holds `sim`), each its own journal line; the gym door and the tab run the same bus, so one act sequence is one journal through any door (`../../doors/gym/gym.test.ts`).
- The report names the world's `seed` and its settled `rules`, tallies acts and refusals per player with its `kind`, acts per verb and refusals per verb (`refusals`), and replays the journal from the config to check the stored world to the bit (`replayed`).
- No model and no network: the bots are scripted; which apps a simulation holds is Carlo's call, open.
