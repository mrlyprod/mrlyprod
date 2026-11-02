import type { ComponentType, ReactNode } from "react";
import type { Item as Row } from "../lib/app/menus.ts";

/* PARTS */

export const REQUIRED = [
  "bar",
  "window",
  "app",
  "menu",
  "control",
  "group",
  "dialog",
  "prose",
  "tile",
  "launcher",
] as const;

export const OPTIONAL = ["desk", "dock", "widgets", "library", "lock", "saver"] as const;

export type Required = (typeof REQUIRED)[number];

export type Optional = (typeof OPTIONAL)[number];

export type Part = Required | Optional;

/* PROPS */

export type Bind = () => Record<string, unknown>;

export type Side = "left" | "right";

export type Item = {
  label: string;
  icon?: string;
  href?: string;
  act?: string;
  on?: boolean;
  radio?: boolean;
  off?: boolean;
  keys?: string[];
};

export type Tile = { id: string; title: string; href: string; art: ReactNode; current?: boolean; soon?: boolean };

export type Scalar = string | number;

export type Clock = { now: Date; hours: 12 | 24 | null };

export type Props = {
  desk: { canvas: Bind; home: boolean };
  bar: {
    site: { name: string; logo: string; word: string };
    app: string | null;
    menus: { name: string; bind: Bind }[];
    cart: number;
    clock: Clock;
    bind: { launch: Bind; mark: Bind; cart: Bind; find: Bind };
  };
  menu: {
    name: string;
    items: (Item | null)[];
    anchor?: DOMRect;
    mac: boolean;
    bind: { list: Bind; item: (i: number) => Bind };
  };
  window: {
    kind: "page" | "wide";
    title: string;
    zoomed: boolean;
    tools: boolean;
    drawer: boolean;
    width: number;
    menus?: { name: string; bind: Bind }[];
    bind: {
      close: Bind;
      zoom: Bind;
      tools: Bind;
      edge: (side: Side) => Bind;
      scroll: Bind;
      main: Bind;
      pane: Bind;
      scrim: Bind;
    };
    children: ReactNode;
    side?: ReactNode;
  };
  app: {
    title: string;
    slots: Partial<Record<"stage" | "left" | "right" | "tail" | "actions" | "status" | "hints", Bind>>;
    open: Record<Side, boolean>;
    widths: Record<Side, number>;
    float: boolean;
    onFlip: (side: Side) => void;
    bind: { root: Bind; pane: (side: Side) => Bind; grip: (side: Side) => Bind; opener: (side: Side) => Bind };
    children?: ReactNode;
  };
  control: {
    kind: "slider" | "number" | "toggle" | "pick" | "segment" | "text" | "button";
    label: ReactNode;
    value: Scalar;
    text?: ReactNode;
    options?: [Scalar, Scalar][];
    min?: number;
    max?: number;
    step?: number;
    unit?: string;
    zero?: string;
    primary?: boolean;
    on?: boolean;
    wide?: boolean;
    tag?: string;
    onChange?: (value: Scalar) => void;
    onCommit?: () => void;
    bind?: Bind;
    children?: ReactNode;
  };
  group: {
    name?: string;
    open?: boolean | null;
    live?: ReactNode;
    onToggle?: (on: boolean) => void;
    bind?: Bind;
    children: ReactNode;
  };
  dialog: { title: string; light: boolean; tag?: string; bind: Bind; children: ReactNode };
  prose: {
    kind: "page" | "post" | "hub" | "lost" | "error";
    id?: string;
    title: string;
    lead?: ReactNode;
    by?: string;
    dates?: string[];
    figure?: string;
    html?: string;
    contents: { id: string; level: number; text: string; on: boolean }[];
    cards?: { href: string; name: string; text?: string; figure?: string }[];
    children?: ReactNode;
    after?: ReactNode;
  };
  tile: Tile & { named: boolean; bind: Bind };
  launcher: {
    word: string;
    query: string;
    onQuery: (query: string) => void;
    chips: string[];
    chip: string;
    onChip: (chip: string) => void;
    naming: string | null;
    sections: {
      name: string;
      tiles: Tile[];
      bind: { shelf: Bind; head: Bind; list: Bind; tile: (id: string) => Bind; drop?: Bind };
    }[];
    results: { name: string; tiles: Tile[]; hits: Item[] }[] | null;
    at: number;
    said: string | null;
    more: Item[];
    menu: { label: string; items: Item[]; bind: { list: Bind; item: (i: number) => Bind } } | null;
    bind: {
      root: Bind;
      veil: Bind;
      find: Bind;
      body: Bind;
      results: Bind;
      hit: (i: number) => Bind;
      chips: Bind;
      chip: (name: string) => Bind;
      rename: Bind;
      add: Bind;
      more: { button: Bind; list: Bind; item: (i: number) => Bind };
      said: Bind;
      undo: Bind | null;
    };
  };
  library: {
    label: string;
    chips: [string, string][];
    chip: string;
    onChip: (chip: string) => void;
    shelves: {
      name: string;
      grid: boolean;
      items: {
        label: string;
        sub?: string;
        on: boolean;
        href?: string;
        paint?: (pen: CanvasRenderingContext2D, side: number) => void;
        bind: Bind;
      }[];
    }[];
  };
  dock: {
    launch: Tile;
    groups: Tile[][];
    magnify: boolean;
    bind: {
      root: Bind;
      group: (g: number) => Bind;
      tile: (id: string) => Bind;
      lens: { item: (id: string) => Bind; gap: (g: number) => Bind };
    };
  };
  widgets: {
    cards: { id: string; name: string; size: "wide" | "small"; body: ReactNode; held: boolean }[];
    editing: boolean;
    gallery: { id: string; name: string; size: "wide" | "small"; body: ReactNode; added: boolean }[];
    menu: { list: Bind; edit: Bind; wallpaper: Bind } | null;
    onEdit: (on: boolean) => void;
    onAdd: (id: string) => void;
    onRemove: (id: string) => void;
    bind: {
      root: Bind;
      column: Bind;
      card: (id: string) => Bind;
      edit: Bind;
      gallery: Bind;
      shelf: Bind;
      pick: (id: string) => Bind;
      done: Bind;
    };
  };
  lock: { word: string; state: "wake" | "rest" | "leaving"; bind: Bind };
  saver: { clock: Clock | null; state?: "on" | "out"; stage: Bind };
};

/* DESIGN */

export type Tokens = Record<`--${string}`, string>;

export type Measure = {
  page: number;
  wide: number;
  least: number;
  step: number;
  panes: { width: number; least: number; most: number; gap: number; stage: number };
  edge?: number;
  dock?: number;
  thumb?: number;
};

export type World = { root: HTMLElement; desk: () => HTMLCanvasElement | null; calm: () => boolean };

export type Pulldown = {
  drop: number;
  button: (name: string) => HTMLButtonElement;
  sheet: (name: string) => HTMLElement;
  item: (row: Row | null, mac: boolean) => HTMLElement;
};

export type Parts = { [K in Required]: ComponentType<Props[K]> } & { [K in Optional]?: ComponentType<Props[K]> };

export type Design = {
  id: string;
  title: string;
  skin: { light: Tokens; dark: Tokens };
  css: string[];
  measure: Measure;
  parts: Parts;
  pulldown?: string;
  start?: (world: World) => () => void;
};

export const lacks = (design: { parts: Partial<Record<Part, unknown>> }): Required[] =>
  REQUIRED.filter((part) => typeof design.parts[part] !== "function");
