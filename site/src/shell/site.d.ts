interface WindowEventMap {
  wire: CustomEvent<import("./route.ts").Wired>;
  go: CustomEvent<string>;
  menus: CustomEvent<import("../lib/app/menus.ts").Model | null>;
}

declare module "site:routes" {
  export type Site = {
    title: string;
    root: string;
    tagline: string;
    prefix: string;
    tint: string;
    menu: string;
    company: string;
    since: number;
    desk: string;
    savers: import("../../scripts/build.ts").Saver[];
    launchpad: import("../../scripts/build.ts").Pad;
    redirects: Record<string, { to: string; since: string }>;
    version: string;
    units: string[];
    licences: { name: string; href: string }[];
    socials: { name: string; href: string }[];
    contact: string;
    tree: { name: string; nodes: { name: string; href: string }[] }[];
    git: { slug: string; branch: string };
    doors?: undefined;
  };

  export const site: Site;
  export const rows: import("../../scripts/build.ts").Row[];
}

declare module "site:icons" {
  export const defs: string;
  export const arts: Record<string, string>;
}

declare module "site:css" {
  export const css: Record<string, string>;
}

declare module "site:apps" {
  export const apps: Record<
    string,
    () => Promise<import("../lib/app/island.tsx").Island<import("./router.ts").Mounting> | import("./router.ts").Own>
  >;
  export const own: Set<string>;
  export const lock: () => Promise<typeof import("./lock.ts")>;
}

declare module "site:figures" {
  export const units: Record<string, string>;
  export const figures: Record<string, () => Promise<import("../lib/figure.ts").Figure>>;
  export const plates: string[];
}

declare module "apps:scenes" {
  export const scenes: Record<string, () => Promise<import("./savers.ts").Scene>>;
}

declare module "*?gate" {
  export const Gate: typeof import("./app.tsx").Gate;
  export const gated: typeof import("./app.tsx").gated;
  export const page: typeof import("./app.tsx").page;
}

declare module "*?post" {
  export const Post: typeof import("./prose.tsx").Post;
}
