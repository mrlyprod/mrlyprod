import type { Archive } from "@mrlyprod/apps/archive/app.ts";
import { dialed } from "./types.ts";

const knob = dialed("archive");

import { below, ME, pick, SLICE } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

const bot: Bot<Archive> = ({ state, me, actors, draw, world }): Step => {
  const mine = Object.values(state.pages).filter((one) => one.by === me);
  if (mine.length >= knob(world.rules, "keep") - 2 && mine.length) return { verb: "delete", args: { id: mine[0]!.id } };
  const apps = Object.keys(world.apps).filter((id) => id !== "archive");
  if (!apps.length || below(draw, 3)) return null;
  const app = pick(draw, apps);
  const others = actors.filter((id) => id !== me);
  const paths = [app, `${app}/${SLICE}${ME}`, ...(others.length ? [`${app}/${SLICE}${pick(draw, others)}`] : [])];
  return { verb: "save", args: { path: pick(draw, paths) } };
};

export default bot;
