import { CARPET, SPONGE, shape } from "./bang.ts";
import type { Body } from "./body.ts";
import { earth } from "./earth.ts";
import { land } from "./land.ts";
import type { Route } from "./pretend.ts";
import { pick, rng } from "./rand.ts";
import { sim } from "./sim.ts";
import type { Theme } from "./theme.ts";

export type Source = {
  id: string;
  name: string;
  label: string;
  seed: number;
  rate: number;
  longest: number;
  fade: number;
  width: number;
  xray: boolean;
  lean: number;
  pitch: number;
  auto: number;
  build: (theme: Theme) => Body;
  route: (body: Body) => Route;
};

const HUBS = 24;
const HUBBY = 0.7;
const count = (n: number) => n.toLocaleString("en-US");

function reality(): Source {
  const mask = land();
  const cells = [...mask.keys()].filter((i) => mask[i]);
  const hubs = Array.from({ length: HUBS }, (_, i) => cells[pick(rng(11, 0xc0ffee + i), cells.length)]);
  return {
    id: "reality",
    name: "Reality",
    label: `<b>Earth</b> · ${count(mask.length)} tiles · ${count(cells.length)} land`,
    seed: 11,
    rate: 4,
    longest: 4,
    fade: 1.4,
    width: 2,
    xray: false,
    lean: -0.41,
    pitch: 0.3,
    auto: 0.1,
    build: (theme) => earth(theme),
    route: (body) => (rand) => {
      const from = rand() < HUBBY ? hubs[pick(rand, hubs.length)] : cells[pick(rand, cells.length)];
      let to = cells[pick(rand, cells.length)];
      if (to === from) to = cells[(cells.indexOf(to) + 1) % cells.length];
      return { from, to, start: 0, dur: 1.2 + 2.8 * body.span(from, to) };
    },
  };
}

function world(id: string, name: string, title: string, code: number, dim: number, level: number): Source {
  const made = shape(code, dim, level);
  const flat = dim === 2;
  return {
    id,
    name,
    label: `<b>${title} ${level}</b> · ${count(made.cells.length)} bots`,
    seed: 100 * code + level,
    rate: made.cells.length / 10,
    longest: 0.9,
    fade: 0.55,
    width: flat ? 3 : 2.5,
    xray: !flat,
    lean: flat ? 0 : -0.3,
    pitch: flat ? 0.95 : 0.35,
    auto: flat ? 0.12 : 0.16,
    build: (theme) => sim(made, flat ? 0.82 : 0.74, theme),
    route: () => (rand) => {
      const from = pick(rand, made.cells.length);
      const near = made.neighbors[from];
      return { from, to: near[pick(rand, near.length)], start: 0, dur: 0.5 + 0.4 * rand() };
    },
  };
}

export const SOURCES: Source[] = [
  reality(),
  world("sim1", "Sim 1", "Sierpinski carpet", CARPET, 2, 2),
  world("sim2", "Sim 2", "Menger sponge", SPONGE, 3, 2),
];
