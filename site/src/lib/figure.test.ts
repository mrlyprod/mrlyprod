import { expect, test } from "bun:test";
import { drop, due, fit, OPENER, queue, scan, TILE } from "./figure.ts";

type Job = { name: string; gap: number };

test("a host canvas is its css box times the device pixels, capped at 512 for a tile and 1024 for an opener, in the figure aspect", () => {
  expect(fit(150, 2, TILE)).toEqual([300, 300]);
  expect(fit(300, 3, TILE)).toEqual([512, 512]);
  expect(fit(400, 2, OPENER)).toEqual([800, 800]);
  expect(fit(700, 2, OPENER)).toEqual([1024, 1024]);
  expect(fit(700, 2, OPENER, [512, 512])).toEqual([512, 512]);
  expect(fit(300, 1, OPENER, [1200, 630])).toEqual([300, 158]);
});

test("the queue draws one host a frame, nearest first, and a route change drops the rest", async () => {
  const frames: (() => void)[] = [];
  const drawn: string[] = [];
  const line = queue<Job>({
    near: (one) => one.gap,
    paint: (one) => drawn.push(one.name),
    frame: (fn) => frames.push(fn),
    cancel: () => {},
  });
  const tick = async () => {
    frames.shift()?.();
    await new Promise((done) => setTimeout(done, 0));
  };
  for (const [name, gap] of [
    ["far", 900],
    ["near", 0],
    ["mid", 300],
  ] satisfies [string, number][])
    line.add({ name, gap });
  await tick();
  await tick();
  line.drop();
  await tick();
  await tick();
  expect([drawn, line.size, frames.length]).toEqual([["near", "mid"], 0, 0]);
});

test("a seen host is due again when its ink changed or it loops, so a looping figure resumes on intersect and on a visible tab", () => {
  const host = (seen: boolean, key: string, loop: boolean) => ({ seen, key, loop });
  expect([
    due(host(true, "a", false), "a"),
    due(host(true, "b", false), "a"),
    due(host(true, "a", true), "a"),
    due(host(false, "b", true), "a"),
  ]).toEqual([false, true, true, false]);
});

test("a route change frees every host canvas but the kept ones still on the page", () => {
  const real = globalThis.IntersectionObserver;
  Object.assign(globalThis, {
    IntersectionObserver: class {
      observe() {}
      unobserve() {}
    },
  });
  const made = (connected: boolean) => {
    const node = { width: 800, height: 800 };
    return {
      node,
      host: Object.assign({} as HTMLElement, {
        isConnected: connected,
        dataset: { figure: "home" },
        closest: () => null,
        querySelector: () => node,
        setAttribute() {},
        removeAttribute() {},
      }),
    };
  };
  const [page, kept, gone] = [made(true), made(true), made(false)];
  scan({ querySelectorAll: () => [page.host] });
  scan({ querySelectorAll: () => [kept.host, gone.host] }, false, true);
  drop();
  Object.assign(globalThis, { IntersectionObserver: real });
  expect([page, kept, gone].map(({ node }) => node.width)).toEqual([0, 800, 0]);
});
