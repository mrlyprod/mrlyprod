import { expect, test } from "bun:test";
import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { Page } from "../net/sdk/page.tsx";
import { values } from "../net/sdk/page.tsx";
import type { Patch, Scalar, Units, Value } from "../net/sdk/spec.ts";
import { tidy } from "../net/sdk/spec.ts";
import type { Model, Schema } from "../net/sdk/verbs.ts";
import { call, list, read } from "../net/sdk/verbs.ts";

type Loaded = { ready: unknown } & Record<string, unknown>;

const APPS = readdirSync(import.meta.dir, { withFileTypes: true })
  .filter((one) => one.isDirectory() && existsSync(join(import.meta.dir, one.name, "scene.ts")))
  .map((one) => one.name);

function turned(args: Schema, now: Scalar): Scalar | undefined {
  if (args.enum) return args.enum.find((one) => one !== now);
  if (args.type !== "integer" && args.type !== "number") return undefined;
  if (args.minimum !== undefined && args.minimum !== now) return args.minimum;
  return args.maximum ?? Number(now) + (args.multipleOf ?? 1);
}

async function staged(id: string) {
  const { PAGE }: { PAGE: Page } = await import(`./${id}/scene.ts`);
  const { spec, keys = [] } = PAGE;
  const loaded = PAGE.units ? ((await PAGE.units()) as unknown as Loaded) : null;
  await loaded?.ready;
  const units = loaded
    ? (Object.fromEntries(Object.entries(loaded).filter(([key]) => key !== "ready")) as Units)
    : undefined;
  let raw: Value = values({ seed: 0, ...PAGE.defaults }, spec);
  const model = (): Model => ({
    id,
    spec,
    keys,
    units,
    value: spec ? { ...raw, ...tidy(spec, raw, units) } : raw,
  });
  const ports = {
    change: (patch: Patch) => {
      raw = { ...raw, ...patch } as Value;
    },
    run: () => {},
  };
  return { model, ports };
}

for (const id of APPS)
  test(`${id}: an agent lists its verbs, sets a knob and reads the new value`, async () => {
    const { model, ports } = await staged(id);
    const verbs = list(model());
    expect(verbs.slice(0, 3).map((one) => one.name)).toEqual(
      ["set", "reroll", "pause"].map((verb) => `mrlyprod.${id}.${verb}`),
    );
    const now = model().value;
    const props = verbs[0].args.properties!;
    const row = (model().spec ?? []).find(
      (one) => (!one.when || one.when(now)) && turned(props[one.key], now[one.key]) !== undefined,
    );
    const key = row?.key ?? "seed";
    const want = row ? turned(props[key], now[key])! : 7;
    call(model(), ports, `mrlyprod.${id}.set`, { [key]: want });
    expect([key, read(model()).value[key]]).toEqual([key, want]);
  });
