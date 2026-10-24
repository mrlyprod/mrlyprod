import { expect, mock, test } from "bun:test";

const make = (_canvas, view, value) => ({ value, lock: view.lock, draw: () => {} });

const SAVERS = [
  { id: "still", title: "Still", pick: () => ({ tone: "red" }) },
  { id: "drift", title: "Drift", pick: (rand) => ({ tone: "blue", at: Math.floor(rand() * 100) }) },
];

const SITE = {
  desk: "two/still",
  savers: [
    { id: "two/still", app: "two", name: "Two", title: "Still" },
    { id: "two/drift", app: "two", name: "Two", title: "Drift" },
    { id: "one/one", app: "one", name: "One", title: "One" },
  ],
};

mock.module("site:routes", () => ({ site: SITE, rows: [] }));
mock.module("apps:scenes", () => ({ scenes: { one: async () => ({ make }), two: async () => ({ SAVERS, make }) } }));

const { lock } = await import("./lock.js");

const { run } = await import("../lib/app/run.js");

const canvas = () => ({ clientWidth: 4, clientHeight: 4 });

async function staged(fn) {
  Object.assign(globalThis, {
    window: new EventTarget(),
    document: Object.assign(new EventTarget(), { hidden: true, documentElement: { dataset: {} } }),
  });
  try {
    return await fn();
  } finally {
    for (const name of ["window", "document"]) delete globalThis[name];
  }
}

const locked = (pick, how) =>
  staged(async () => {
    const stop = await lock(canvas(), pick, how);
    stop();
    return stop;
  });

test("an empty pick runs the desk saver of the site with its picked value", async () => {
  const stop = await locked("");
  expect([stop.pick, stop.id, stop.scene.value.tone]).toEqual(["", "two/still", "red"]);
});

test("random runs a saver of the pool", async () => {
  const pool = ["one/one", "two/drift"];
  const seen = [];
  for (let i = 0; i < 8; i++) seen.push(await locked("random", { pool }));
  expect([[...new Set(seen.map((one) => one.pick))], seen.every((one) => pool.includes(one.id))]).toEqual([
    ["random"],
    true,
  ]);
});

test("a saver id with a query runs that exact value", async () => {
  const stop = await locked("two/drift?seed=7&tone=green");
  expect([stop.id, stop.scene.value]).toEqual(["two/drift", { seed: 7, tone: "green" }]);
});

test("an unknown saver becomes random", async () => {
  const stop = await locked("gone?seed=3");
  expect([stop.pick, SITE.savers.some((one) => one.id === stop.id)]).toEqual(["random", true]);
});

test("the lock names the saver it ran and the value it rolled", async () => {
  const stop = await locked("two/drift");
  const { seed, at } = stop.scene.value;
  expect([Number.isInteger(seed), stop.as]).toEqual([true, `two/drift?seed=${seed}&tone=blue&at=${at}`]);
});

test("view.lock is true when the lock runs a scene and false on the page", async () => {
  const seen = await staged(async () => {
    const shut = await lock(canvas(), "one/one");
    const paged = run(canvas(), make, {});
    const both = [shut.scene.lock, paged.scene.lock];
    shut();
    paged();
    return both;
  });
  expect(seen).toEqual([true, false]);
});
