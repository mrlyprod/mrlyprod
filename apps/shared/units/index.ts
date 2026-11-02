import { LOAD } from "./load.ts";

type Mods = { [N in keyof typeof LOAD]: Awaited<ReturnType<(typeof LOAD)[N]>> };

export type Name = keyof Mods;

export type Unit<N extends Name> = N extends Name ? (Mods[N] extends Record<N, infer U> ? U : never) : never;

type Ready<N extends Name> = Mods[N]["ready"];

export type Units<N extends Name> = { [K in N]: Unit<K> } & { ready: Promise<Awaited<Ready<N>>[]> };

export const wasm =
  <N extends Name = never>(...names: N[]) =>
  (): Promise<Units<N>> =>
    Promise.all(names.map((name) => LOAD[name]().then((mod) => [name, mod] as const))).then(
      (pairs) =>
        ({
          ...Object.fromEntries(pairs.map(([name, mod]) => [name, (mod as Record<N, unknown>)[name]])),
          ready: Promise.all(pairs.map(([, mod]) => mod.ready)),
        }) as Units<N>,
    );
