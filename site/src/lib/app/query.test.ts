import { expect, test } from "bun:test";
import { createElement } from "react";
import { renderToString } from "react-dom/server";
import type { Query } from "./query.ts";
import { stamp, useQuery } from "./query.ts";

type Queued = [() => void, number];

const read = (search: string, defaults: Query) => {
  Object.assign(globalThis, { location: { search } });
  let state: Query | undefined;
  const Probe = () => {
    [state] = useQuery(defaults);
    return null;
  };
  renderToString(createElement(Probe));
  Reflect.deleteProperty(globalThis, "location");
  return state;
};

test("an empty query value takes the fallback", () => {
  expect(read("?max=", { max: 5 })).toEqual({ max: 5 });
  expect(read("?max=abc", { max: 5 })).toEqual({ max: 5 });
});

test("two stamps in 200 ms write the url once, with the later keys merged over the earlier", () => {
  const queued: Queued[] = [];
  const wrote: string[] = [];
  const real = globalThis.setTimeout;
  Object.assign(globalThis, {
    setTimeout: (fn: () => void, ms: number) => queued.push([fn, ms]),
    location: { search: "?a=1&c=3", pathname: "/x", hash: "" },
    document: { documentElement: { getAttribute: () => null } },
    history: { state: null, replaceState: (_state: unknown, _title: string, url: string) => wrote.push(url) },
  });
  stamp({ a: 2 });
  stamp({ b: true, c: "" });
  const early = wrote.length;
  queued[0][0]();
  globalThis.setTimeout = real;
  for (const name of ["location", "document", "history"]) Reflect.deleteProperty(globalThis, name);
  expect([queued.map(([, ms]) => ms), early, wrote]).toEqual([[200], 0, ["/x?a=2&b=1"]]);
});

test("a stamp on a new page drops the keys the last page left pending", () => {
  const queued: Queued[] = [];
  const wrote: string[] = [];
  const real = globalThis.setTimeout;
  Object.assign(globalThis, {
    setTimeout: (fn: () => void, ms: number) => queued.push([fn, ms]),
    document: { documentElement: { getAttribute: () => null } },
    history: { state: null, replaceState: (_state: unknown, _title: string, url: string) => wrote.push(url) },
    location: { search: "", pathname: "/old/", hash: "" },
  });
  stamp({ max: 20 });
  Object.assign(globalThis, { location: { search: "", pathname: "/new/", hash: "" } });
  stamp({ seed: 5 });
  queued[0][0]();
  globalThis.setTimeout = real;
  for (const name of ["location", "document", "history"]) Reflect.deleteProperty(globalThis, name);
  expect(wrote).toEqual(["/new/?seed=5"]);
});

test("a write while the router is busy waits and lands once it is done", () => {
  const queued: Queued[] = [];
  const wrote: string[] = [];
  let busy: string | null = "true";
  const real = globalThis.setTimeout;
  Object.assign(globalThis, {
    setTimeout: (fn: () => void, ms: number) => queued.push([fn, ms]),
    document: { documentElement: { getAttribute: () => busy } },
    history: { state: null, replaceState: (_state: unknown, _title: string, url: string) => wrote.push(url) },
    location: { search: "", pathname: "/x/", hash: "" },
  });
  stamp({ max: 20 });
  queued[0][0]();
  const early = wrote.length;
  busy = null;
  queued[1][0]();
  globalThis.setTimeout = real;
  for (const name of ["location", "document", "history"]) Reflect.deleteProperty(globalThis, name);
  expect([queued.length, early, wrote]).toEqual([2, 0, ["/x/?max=20"]]);
});
