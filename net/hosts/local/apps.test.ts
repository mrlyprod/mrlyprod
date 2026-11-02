import { expect, test } from "bun:test";
import { join } from "node:path";
import type { Line } from "../../kernel/index.ts";
import { check, digest, kernel, replay } from "../../kernel/index.ts";
import { scene } from "../../sdk/apps.ts";
import type { Page } from "../../sdk/page.tsx";
import type { Value } from "../../sdk/spec.ts";
import { rolled, tidy } from "../../sdk/spec.ts";
import { goose } from "../../sim/goose.ts";
import { APPS } from "../../test/apps.ts";
import { found, install, prepare, site, user } from "./apps.ts";

const ME = user(site());

const BOT = { id: "goose", kind: "bot" as const };

const lcg = (seed: number) => {
  let s = seed;
  return () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s;
  };
};

test("every app of the site lifts into one kernel world; the goose drives them all and the journal replays to the bit", async () => {
  const apps = await install();
  expect(apps.map((one) => one.id)).toEqual(found().map((one) => one.id));
  const K = kernel(apps);
  const config = { name: "all", seed: 5, apps: apps.map((one) => one.id) };
  let world = K.boot(config);
  const lines: Line[] = [];
  const draw = lcg(11);
  for (let i = 0; i < 400; i++) {
    const moves = K.list(world).moves;
    const act = goose(moves, draw)!;
    expect([act.verb, check(moves.find((one) => one.verb === act.verb)!.args, act.args)]).toEqual([act.verb, null]);
    const done = K.call(world, { ...act, who: BOT, at: i * 1000 });
    world = done.world;
    lines.push(done.line);
  }
  expect(digest(replay(K, config, lines))).toBe(digest(world));
});

test("a reducer app keeps its knobs and plays its own verbs: snake turns and ticks, and refuses a turn back into itself", async () => {
  const K = kernel(await install(["snake"], APPS));
  let world = K.boot({ name: "snake", apps: ["snake"] });
  const verbs = K.list(world).moves.map((one) => one.verb);
  expect(verbs).toContain(`${ME}.snake.turn`);
  expect(verbs).toContain(`${ME}.snake.tick`);
  const play = () => world.apps.snake as { play: { snake: number[]; dir: number } };
  const head = play().play.snake[0];
  world = K.call(world, { verb: `${ME}.snake.tick`, who: BOT }).world;
  expect(play().play.snake[0]).not.toBe(head);
  const back = (play().play.dir + 2) & 3;
  expect(() => K.call(world, { verb: `${ME}.snake.turn`, args: { dir: back }, who: BOT })).toThrow("bad-act");
  world = K.call(world, { verb: `${ME}.snake.set`, args: { size: 12 }, who: BOT }).world;
  expect([K.read(world, "snake/size").state, K.read(world, "snake/play/size").state]).toEqual([12, 12]);
});

test("a scene's reroll writes the same knobs through the kernel as through the page, seed for seed", async () => {
  await prepare(APPS);
  for (const one of found(APPS).filter((each) => each.kind === "scene")) {
    const { PAGE } = (await import(join(one.dir, "scene.ts"))) as { PAGE: Page };
    if (!PAGE.reroll) continue;
    const loaded = PAGE.units ? ((await PAGE.units()) as Record<string, unknown>) : null;
    await loaded?.ready;
    const units = loaded ? Object.fromEntries(Object.entries(loaded).filter(([key]) => key !== "ready")) : undefined;
    const lifted = scene({ ...PAGE, id: one.id, user: "t", title: one.id, line: "", units });
    const K = kernel([lifted]);
    let world = K.boot({ name: one.id, seed: 3, apps: [one.id] });
    for (let i = 0; i < 2; i++) {
      const { play: _, ...was } = world.apps[one.id] as Value;
      world = K.call(world, { verb: `t.${one.id}.reroll`, who: BOT }).world;
      const { play: __, ...now } = world.apps[one.id] as Value;
      const next = { ...was, ...rolled(was, Number(now.seed), units, PAGE.reroll) } as Value;
      expect([one.id, now]).toEqual([one.id, PAGE.spec ? { ...next, ...tidy(PAGE.spec, next, units) } : next]);
    }
  }
});
