import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { build, bytes, forget, type Spec } from "../kit/ssg/build.ts";
import { ensureFigures, type Get, type Want } from "./figs.ts";

/* WORLD */

const PIN = { key: "kp", files: ["pin.png", "pin.webp"] };

function world(over?: Get) {
  const home = mkdtempSync(join(tmpdir(), "figs-"));
  const at = { root: join(home, "site"), press: join(home, "press"), cache: join(home, "cache") };
  mkdirSync(at.root, { recursive: true });
  mkdirSync(at.press, { recursive: true });
  const calls: string[] = [];
  const get: Get = async (want: Want) => {
    calls.push(`${want.key}/${want.from}`);
    return new TextEncoder().encode(`s3 ${want.key}/${want.from}`);
  };
  const lock = (rows: unknown) => writeFileSync(join(at.root, "figures.lock"), JSON.stringify(rows));
  const desk = (rows: unknown) => writeFileSync(join(at.press, "figures.lock"), JSON.stringify(rows));
  const store = (base: string, key: string, files: string[]) => {
    mkdirSync(join(at.press, base, key), { recursive: true });
    for (const file of files) writeFileSync(join(at.press, base, key, file), `desk ${key}/${file}`);
  };
  const names = () => readdirSync(join(at.root, "data", "figures")).filter((name) => !name.startsWith(".")).sort();
  const text = (name: string) => readFileSync(join(at.root, "data", "figures", name), "utf8");
  const run = () => ensureFigures({ ...at, get: over ?? get });
  return { home, at, calls, lock, desk, store, names, text, run };
}

const trio = (name: string) => [`${name}-dark.webp`, `${name}-light.webp`, `${name}.png`];

/* PLACE */

test("a row lands under today's names, from the desk store first and from s3 once, a pinned row under its own", async () => {
  const w = world();
  w.lock({ a: "ka", b: "kb", pin: PIN });
  w.store("store", "ka", trio("a"));
  w.store("pins", "kp", PIN.files);
  const made = await w.run();
  expect(w.names()).toEqual(["a-dark.png", "a-dark.webp", "a-light.webp", "b-dark.png", "b-dark.webp", "b-light.webp", "pin.png", "pin.webp"]);
  expect(w.text("a-dark.png")).toBe("desk ka/a.png");
  expect(w.text("pin.webp")).toBe("desk kp/pin.webp");
  expect(w.text("b-dark.png")).toBe("s3 kb/b.png");
  expect([made.files, made.placed, made.fetched]).toEqual([8, 8, 3]);
  expect(w.calls.sort()).toEqual(["kb/b-dark.webp", "kb/b-light.webp", "kb/b.png"]);
  rmSync(join(w.at.root, "data"), { recursive: true });
  expect((await w.run()).fetched).toBe(0);
  expect(w.calls.length).toBe(3);
  rmSync(w.home, { recursive: true });
});

test("a desk lock row wins when its store folder exists, loses when it has none, and never unpins", async () => {
  const w = world();
  w.lock({ a: "ka", pin: PIN });
  w.desk({ a: "ka2", c: "kc", pin: "kz" });
  w.store("store", "ka2", trio("a"));
  w.store("pins", "kp", PIN.files);
  await w.run();
  expect(w.text("a-light.webp")).toBe("desk ka2/a-light.webp");
  expect(w.names()).toEqual(["a-dark.png", "a-dark.webp", "a-light.webp", "pin.png", "pin.webp"]);
  expect(w.text("pin.png")).toBe("desk kp/pin.png");
  rmSync(w.home, { recursive: true });
});

test("a second run places nothing, a moved key replaces its files and a name that left the lock loses them", async () => {
  const w = world();
  w.lock({ a: "ka", b: "kb" });
  w.store("store", "ka", trio("a"));
  w.store("store", "kb", trio("b"));
  expect((await w.run()).placed).toBe(6);
  expect((await w.run()).placed).toBe(0);
  w.store("store", "ka2", trio("a"));
  w.lock({ a: "ka2" });
  const made = await w.run();
  expect([made.placed, made.removed]).toEqual([3, 3]);
  expect(w.names()).toEqual(["a-dark.png", "a-dark.webp", "a-light.webp"]);
  expect(w.text("a-dark.webp")).toBe("desk ka2/a-dark.webp");
  rmSync(w.home, { recursive: true });
});

/* READS */

test("a figure pressed again repaints the routes that ship it and no others, by the files it left under the same names", async () => {
  const w = world();
  w.lock({ a: "ka", b: "kb" });
  w.store("store", "ka", trio("a"));
  w.store("store", "kb", trio("b"));
  await w.run();
  const dir = join(w.at.root, "data", "figures");
  writeFileSync(join(w.at.root, "site.json"), "{}");
  const drawn: string[] = [];
  const spec: Spec = {
    root: w.at.root,
    out: join(w.home, "dist"),
    collect: () => ({ routes: [{ route: "/a/" }, { route: "/b/" }] }),
    render: (_site, route) => {
      drawn.push(route.route);
      const name = route.route === "/a/" ? "a" : "b";
      return [
        { path: `${route.route.slice(1)}index.html`, bytes: "<p>x</p>" },
        { path: `figures/${name}-dark.webp`, bytes: bytes(join(dir, `${name}-dark.webp`)) },
      ];
    },
  };
  const round = async () => {
    drawn.length = 0;
    forget();
    await ensureFigures({ ...w.at, get: async () => new Uint8Array() });
    await build(spec, { manifest: "manifest.json" });
    return drawn.join(" ");
  };
  expect(await round()).toBe("/a/ /b/");
  expect(await round()).toBe("");
  w.store("store", "ka2", trio("a"));
  w.lock({ a: "ka2", b: "kb" });
  expect(await round()).toBe("/a/");
  expect(readFileSync(join(w.home, "dist", "figures", "a-dark.webp"), "utf8")).toBe("desk ka2/a-dark.webp");
  expect(readFileSync(join(w.home, "dist", "figures", "b-dark.webp"), "utf8")).toBe("desk kb/b-dark.webp");
  rmSync(w.home, { recursive: true });
});

/* DIES */

test("a run that dies half way forgets the files it was replacing, so reverting the lock puts every old byte back", async () => {
  const dead: Get = async () => {
    throw new Error("s3 is out");
  };
  const w = world(dead);
  const names = Array.from({ length: 12 }, (_, n) => `n${String(n).padStart(2, "0")}`);
  const rows = (tag: string) => Object.fromEntries(names.map((name) => [name, `${name}-${tag}`]));
  w.lock(rows("old"));
  for (const name of names) w.store("store", `${name}-old`, trio(name));
  expect((await w.run()).placed).toBe(36);
  w.lock(rows("new"));
  for (const name of names.slice(0, 11)) w.store("store", `${name}-new`, trio(name));
  await expect(w.run()).rejects.toThrow(/n11/);
  expect(w.text("n00-dark.webp")).toBe("desk n00-new/n00-dark.webp");
  w.lock(rows("old"));
  expect((await w.run()).placed).toBe(36);
  expect(w.text("n00-dark.webp")).toBe("desk n00-old/n00-dark.webp");
  expect(w.text("n10-dark.png")).toBe("desk n10-old/n10.png");
  rmSync(w.home, { recursive: true });
});

/* AT ONCE */

test("two runs on one folder at once both finish, each writing through its own temp names", async () => {
  const w = world();
  const names = Array.from({ length: 40 }, (_, n) => `n${String(n).padStart(2, "0")}`);
  w.lock(Object.fromEntries(names.map((name) => [name, `${name}-k`])));
  for (const name of names) w.store("store", `${name}-k`, trio(name));
  const code = `import { ensureFigures } from ${JSON.stringify(join(import.meta.dir, "figs.ts"))}; await ensureFigures(JSON.parse(process.env.OPTIONS!));`;
  const start = () => Bun.spawn([process.execPath, "-e", code], { env: { ...process.env, OPTIONS: JSON.stringify(w.at) }, stdout: "ignore", stderr: "pipe" });
  const pair = [start(), start()];
  expect(await Promise.all(pair.map((one) => one.exited))).toEqual([0, 0]);
  expect(w.names().length).toBe(120);
  expect((await w.run()).placed).toBe(0);
  rmSync(w.home, { recursive: true });
});
