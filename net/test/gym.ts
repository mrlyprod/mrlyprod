import { BEAT, gym } from "../doors/gym/gym.ts";
import { install } from "../hosts/local/apps.ts";
import type { Act, App, Config, Envelope, Line, Move, Who, World } from "../kernel/index.ts";
import { below, digest, kernel, replay } from "../kernel/index.ts";
import { bus } from "../sdk/system.ts";
import { drawFor, round, SKIP } from "../sim/bots.ts";
import { goose } from "../sim/goose.ts";

/* TYPES */

export type Played = { same: number; refused: number; acts: number };

export type Scripted = { world: World; acts: number; verbs: Record<string, number>; refused: Record<string, number> };

/* PLAY */

export async function geese(config: Config, worlds: number, rounds: number): Promise<Played> {
  const one = gym();
  const ask = async (body: object) => JSON.parse(await one.answer(JSON.stringify(body)));
  const actors = config.actors ?? ["goose"];
  const seeds = Array.from({ length: worlds }, (_, i) => i);
  await ask({ op: "boot", config, seeds });
  let refused = 0;
  let acts = 0;
  for (let round = 0; round < rounds; round++) {
    const moves: Move[][] = await ask({ op: "list" });
    const before = one.worlds() as World[];
    const played = moves.map((legal, i): Act & { who: Who; at: number } => {
      const draw = drawFor(before[i]!.seed, before[i]!.tick, "goose");
      const who: Who = { id: actors[below(draw, actors.length)]!, kind: "human" };
      return { ...goose(legal, draw, SKIP)!, who, at: before[i]!.at + BEAT };
    });
    const stepped: Envelope[] = await ask({ op: "call", acts: played });
    for (const envelope of stepped) {
      refused += envelope.last.ok ? 0 : 1;
      acts++;
    }
  }
  const after = one.worlds() as World[];
  const lines: Line[][] = await ask({ op: "lines" });
  const ids = config.apps.map((app) => (typeof app === "string" ? app : app.id));
  const k = kernel(await install(ids));
  const same = seeds.filter((seed, i) => digest(replay(k, { ...config, seed }, lines[i]!)) === digest(after[i])).length;
  return { same, refused, acts };
}

/* SCRIPTED */

export function scripted(apps: readonly App[], config: Config, rounds: number): Scripted {
  const k = kernel(apps);
  let world = k.boot(config);
  const out: Scripted = { world, acts: 0, verbs: {}, refused: {} };
  const step = (act: Act) => {
    const done = k.call(world, act);
    world = done.world;
    out.acts++;
    out.verbs[act.verb] = (out.verbs[act.verb] ?? 0) + 1;
    if (!done.envelope.last.ok)
      out.refused[done.envelope.last.note!] = (out.refused[done.envelope.last.note!] ?? 0) + 1;
    return done;
  };
  const call = bus(apps, step);
  const players = (config.actors ?? []).map((id): Who => ({ id, kind: "human" }));
  for (let i = 0; i < rounds; i++)
    round({ world: () => world, list: () => k.list(world), call }, players, (i + 1) * BEAT, apps);
  return { ...out, world };
}
