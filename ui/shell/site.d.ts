interface WindowEventMap {
  wire: CustomEvent<import("./route.ts").Wired>;
  go: CustomEvent<string>;
  menus: CustomEvent<import("../kit/menus.ts").Model | null>;
}

declare module "site:routes" {
  export type Site = {
    title: string;
    wordmark?: string;
    root: string;
    mark: string;
    prefix: string;
    tint: string;
    menu: string;
    company: string;
    since: number;
    desk: string;
    savers: import("../kit/rows.ts").Saver[];
    launchpad: import("../kit/rows.ts").Pad;
    redirects: Record<string, { to: string; since: string }>;
    version: string | null;
    units: string[];
    licences: { name: string; href: string }[];
    socials: { name: string; href: string }[];
    contact: string;
    tree: { name: string; nodes: { name: string; href: string }[] }[];
    git: { slug: string; branch: string; folder: string };
    designs: { id: string; title: string }[];
    min?: [number, number];
    doors?: undefined;
  };

  export const site: Site;
  export const rows: import("../kit/rows.ts").Row[];
}

declare module "site:icons" {
  export const defs: string;
  export const arts: Record<string, string>;
}

declare module "site:widgets" {
  export const order: string[];
  export const widgets: Record<string, import("../kit/rows.ts").Desk>;
}

declare module "site:frame" {
  export const frames: Record<string, string>;
}

declare module "site:design" {
  export const names: string[];
  export const designs: Record<string, () => Promise<import("./design.ts").Design>>;
}

declare module "site:css" {
  export const css: Record<string, string>;
}

declare module "site:apps" {
  export const apps: Record<
    string,
    () => Promise<import("../kit/island.tsx").Island<import("./router.ts").Mounting> | import("./router.ts").Own>
  >;
  export const own: Set<string>;
  export const lock: () => Promise<typeof import("./lock.ts")>;
}

declare module "site:figures" {
  export const units: Record<string, string>;
  export const figures: Record<string, () => Promise<import("@mrlyprod/draw/figure.ts").Figure>>;
  export const plates: string[];
}

declare module "site:world" {
  export const apps: import("@mrlyprod/os/kernel/index.ts").App[];
  export const config: import("@mrlyprod/os/kernel/index.ts").Config;
}

declare module "apps:scenes" {
  export const scenes: Record<string, () => Promise<import("./savers.ts").Scene>>;
}

declare module "*?page" {
  export const page: typeof import("./app.tsx").page;
}
