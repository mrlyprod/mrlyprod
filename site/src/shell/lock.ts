import { site } from "site:routes";
import type { Saver as Listed } from "../../scripts/build.ts";
import type { Ready } from "../lib/app/hooks.ts";
import type { Scene, Stop } from "../lib/app/run.ts";
import { run } from "../lib/app/run.ts";
import { calm } from "./prefs.ts";
import { all, find, load, on, roll } from "./savers.ts";

/* TYPES */

export type Leave = (then?: () => void) => void;

export type Exits = { leave: Leave; end: () => void };

export type Chosen = { pick: string; entry: Listed | null; query: string };

export type Named = { pick: string; id: string; as: string; leave: Leave };

export type Lock = (() => void) &
  Named &
  Partial<Pick<Stop, "pause" | "play" | "coast" | "fix" | "set" | "late" | "scene">>;

export type Pool = { pool?: string[] };

const RANDOM = "random";
const SEEDS = 4294967296;
const CAP = 1500;
const QUIET = 2000;

const number = (text: string) => (text.trim() !== "" && Number.isFinite(Number(text)) ? Number(text) : text);

function parse(query: string): Record<string, string | number> {
  return Object.fromEntries([...new URLSearchParams(query)].map(([key, text]) => [key, number(text)]));
}

function exits(scene: Scene): Exits {
  let go: (() => void) | null | undefined = null;
  let left = false;
  let still: ReturnType<typeof setTimeout> | 0 = 0;
  let cap: ReturnType<typeof setTimeout> | 0 = 0;
  const done = () => {
    clearTimeout(cap);
    const then = go;
    go = null;
    left = false;
    then?.();
  };
  const fall = () => {
    clearTimeout(still);
    left = true;
    if (!scene.windDown) return done();
    cap = setTimeout(done, CAP);
    scene.windDown(done);
  };
  const stay = () => {
    scene.hold!();
    clearTimeout(still);
    still = setTimeout(fall, QUIET);
  };
  const key = (e: Event & Partial<Pick<KeyboardEvent, "metaKey" | "ctrlKey" | "altKey">>) => {
    if (go && !left && scene.hold && !e.metaKey && !e.ctrlKey && !e.altKey) stay();
  };
  const shove = (e: Event) => {
    if (
      !go ||
      left ||
      (e.type === "wheel"
        ? Math.abs((e as WheelEvent).deltaX) > Math.abs((e as WheelEvent).deltaY)
        : (e as TouchEvent).touches?.length > 1)
    )
      return;
    fall();
  };
  const ears: [string, EventListener][] = [
    ["keydown", key],
    ["wheel", shove],
    ["touchmove", shove],
  ];
  for (const [type, fn] of ears) window.addEventListener(type, fn, true);
  const leave: Leave = (then) => {
    if (left) return;
    go = then;
    if (calm()) return done();
    if (scene.hold) stay();
    else fall();
  };
  const end = () => {
    for (const [type, fn] of ears) window.removeEventListener(type, fn, true);
    clearTimeout(still);
    clearTimeout(cap);
    go = null;
  };
  return { leave, end };
}

function choose(pick: string, pool?: string[]): Chosen {
  const at = pick.indexOf("?");
  const head = at < 0 ? pick : pick.slice(0, at);
  const named = head === RANDOM ? null : find(head);
  if (named) return { pick, entry: named, query: at < 0 ? "" : pick.slice(at + 1) };
  if (!head) return { pick: "", entry: find(site.desk) ?? all()[0] ?? null, query: "" };
  const kept = (pool ?? on()).map(find).filter(Boolean);
  const list = kept.length ? kept : all();
  return { pick: RANDOM, entry: list[Math.floor(Math.random() * list.length)] ?? null, query: "" };
}

export async function lock(canvas: HTMLCanvasElement, pick = "", { pool }: Pool = {}): Promise<Lock> {
  const chosen = choose(pick, pool);
  const loaded = chosen.entry ? await load(chosen.entry.id) : null;
  if (!loaded)
    return Object.assign(() => {}, { pick: chosen.pick, id: "", as: "", leave: (then?: () => void) => then?.() });
  const { entry, scene, saver } = loaded;
  const seed = Math.floor(Math.random() * SEEDS);
  const value = chosen.query ? { seed, ...parse(chosen.query) } : roll(saver, seed);
  const held: Partial<Ready> & Record<string, unknown> = scene.units ? await scene.units() : {};
  await held.ready;
  const { ready, ...units } = held;
  const stop = run(canvas, scene.make, { ...value, ...units }, { lock: true, still: calm() || scene.PAGE?.still });
  const { leave, end } = exits(stop.scene);
  const close = () => {
    end();
    stop();
  };
  return Object.assign(close, stop, {
    pick: chosen.pick,
    id: entry.id,
    as: `${entry.id}?${new URLSearchParams(Object.entries(value).map(([key, v]) => [key, String(v)]))}`,
    leave,
  });
}
