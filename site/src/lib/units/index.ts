type Mods = {
  font: typeof import("./font.ts");
  life: typeof import("./life.ts");
  math: typeof import("./math.ts");
  num: typeof import("./num.ts");
};

export type Name = keyof Mods;

export type Unit<N extends Name> = N extends Name ? (Mods[N] extends Record<N, infer U> ? U : never) : never;

type Ready<N extends Name> = Mods[N]["ready"];

type Loaded = { [N in Name]: Record<N, Unit<N>> & { ready: Ready<N> } };

export type Units<N extends Name> = { [K in N]: Unit<K> } & { ready: Promise<Awaited<Ready<N>>[]> };

const LOAD: { [N in Name]: () => Promise<Loaded[N]> } = {
  font: () => import("./font.ts"),
  life: () => import("./life.ts"),
  math: () => import("./math.ts"),
  num: () => import("./num.ts"),
};

export const wasm =
  <N extends Name = never>(...names: N[]) =>
  (): Promise<Units<N>> =>
    Promise.all(names.map((name) => LOAD[name]().then((mod) => [name, mod] as const))).then(
      (pairs) =>
        ({
          ...Object.fromEntries(pairs.map(([name, mod]) => [name, mod[name]])),
          ready: Promise.all(pairs.map(([, mod]) => mod.ready)),
        }) as Units<N>,
    );
