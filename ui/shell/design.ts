import type { Color, Pen } from "@mrlyprod/draw";
import type { ComponentType, ReactNode } from "react";
import type { Item as Row } from "../kit/menus.ts";

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
  "sheet",
  "alert",
  "tabs",
  "table",
  "toast",
  "tooltip",
  "search",
  "progress",
  "empty",
  "calendar",
] as const;

export const OPTIONAL = ["desk", "dock", "widgets", "lock", "saver", "preview"] as const;

export const ICONS = [
  "add",
  "minus",
  "check",
  "close",
  "left",
  "right",
  "up",
  "down",
  "search",
  "more",
  "grid",
  "pane-left",
  "pane-right",
  "shrink",
  "share",
  "download",
  "link",
  "info",
  "warn",
  "lock",
  "cart",
  "star",
  "star-fill",
  "dice",
  "tune",
  "levels",
  "note",
  "volume",
  "keyboard",
  "command",
  "shift",
  "calendar",
  "clock",
  "trash",
  "edit",
  "folder",
  "file",
  "code",
  "image",
  "data",
] as const;

export type IconName = (typeof ICONS)[number];

export const EMOJIS = {
  smile: "\u{1F642}",
  grin: "\u{1F600}",
  joy: "\u{1F602}",
  wink: "\u{1F609}",
  love: "\u{1F60D}",
  cool: "\u{1F60E}",
  sad: "\u{1F622}",
  heart: "\u{2764}\u{FE0F}",
  thumb: "\u{1F44D}",
  fire: "\u{1F525}",
  spark: "\u{2728}",
  party: "\u{1F389}",
} as const;

export type EmojiName = keyof typeof EMOJIS;

export type Glyph = (pen: Pen, ink: Color) => void;

export type Glyphs<Name extends string> = { box: number; crisp?: boolean; draw: Record<Name, Glyph> };

type Required = (typeof REQUIRED)[number];

type Optional = (typeof OPTIONAL)[number];

export type Part = Required | Optional;

export type Bind = () => Record<string, unknown>;

export type Side = "left" | "right";

export type Item = {
  label: string;
  icon?: IconName;
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

export type Ask = { wrong: boolean; busy: boolean; bind: { form: Bind; field: Bind } };

export type Act = { label: string; primary?: boolean; danger?: boolean; bind: Bind };

export type Day = { iso: string; n: number; out: boolean; bind: Bind };

export type Props = {
  desk: { canvas: Bind; home: boolean };
  bar: {
    site: { name: string; logo: string; word: string; cart?: boolean };
    app: string | null;
    cart: number;
    clock: Clock | null;
    bind: { launch: Bind; mark: Bind; cart: Bind; find: Bind; menus: Bind };
  };
  menu: {
    name: string;
    items: (Item | null)[];
    anchor?: DOMRect;
    mac: boolean;
    bind: { list: Bind; item: (i: number) => Bind };
  };
  window: { bind: { close: Bind; title: Bind; menus: Bind; main: Bind; pane: Bind } };
  app: {
    title: string;
    slots: Partial<Record<"stage" | "left" | "right" | "tail" | "actions" | "status" | "hints", Bind>>;
    open: Record<Side, boolean>;
    bind: { pane: (side: Side) => Bind; opener: (side: Side) => Bind };
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
    kind: "page" | "hub" | "lost" | "error";
    id?: string;
    title: string;
    level?: 1 | 2;
    lead?: ReactNode;
    figure?: string;
    html?: string;
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
  dock: {
    launch: Tile;
    groups: Tile[][];
    magnify: boolean;
    bind: {
      root: Bind;
      launch: Bind;
      group: (g: number) => Bind;
      tile: (id: string) => Bind;
      lens: { item: (id: string) => Bind; gap: (g: number) => Bind };
    };
  };
  widgets: {
    cards: { id: string; name: string; size: "wide" | "small"; body: ReactNode; held: boolean }[];
    editing: boolean;
    gallery: { id: string; name: string; size: "wide" | "small"; body: ReactNode; added: boolean }[];
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
  lock: { word: string; state: "wake" | "rest" | "leaving"; bind: Bind; ask?: Ask };
  saver: { clock: Clock | null; state?: "on" | "out"; stage: Bind };
  preview: { title: string; theme: "light" | "dark" };
  sheet: { title: string; tag?: string; bind: { root: Bind; pull: Bind }; children: ReactNode };
  alert: { title: string; text?: string; actions: Act[]; bind: { root: Bind; title: Bind; text: Bind } };
  tabs: {
    items: { id: string; label: string; icon?: IconName }[];
    bind: { list: Bind; tab: (id: string) => Bind; panel: Bind };
    children?: ReactNode;
  };
  table: { head: string[]; rows: ReactNode[][]; align?: ("start" | "end")[]; caption?: string };
  toast: { text: string; kind: "info" | "ok" | "warn"; icon: IconName; action?: Act };
  tooltip: { text: string; bind: { root: Bind; tip: Bind }; children: ReactNode };
  search: {
    label: string;
    value: string;
    placeholder?: string;
    onChange: (value: string) => void;
    bind: { field: Bind; clear: Bind };
  };
  progress: { value: number | null; max: number; label: string };
  empty: { icon?: IconName; title: string; text?: string; action?: Act };
  calendar: {
    label: string;
    text: string;
    month: string;
    week: string[];
    days: Day[];
    bind: { field: Bind; pop: Bind; prev: Bind; next: Bind; grid: Bind };
  };
};

export type Tokens = Record<`--${string}`, string>;

export type World = { root: HTMLElement; desk: () => HTMLCanvasElement | null; calm: () => boolean };

export type Pulldown = {
  drop: number;
  button: (name: string) => HTMLButtonElement;
  sheet: (name: string) => HTMLElement;
  item: (row: Row | null, mac: boolean) => HTMLElement;
};

export type Parts = { [K in Required]: ComponentType<Props[K]> } & { [K in Optional]?: ComponentType<Props[K]> };

export type Knob = { id: string; name: string; options: [string, string][]; hint?: string };

export type Knobs = { icons?: boolean; own?: Knob[] };

export type Design = {
  id: string;
  title: string;
  skin: { light: Tokens; dark: Tokens };
  bar: { light: string; dark: string };
  parts: Parts;
  knobs?: Knobs;
  lens?: { big: number; reach: number };
  icons: Glyphs<IconName>;
  emojis: Glyphs<EmojiName>;
  pulldown?: Pulldown;
  start?: (world: World) => () => void;
};

type Gives = Record<string, Partial<Record<keyof Design, string>>>;

export const MAP: {
  index: string;
  required: Gives;
  optional: Gives;
  each: Record<"parts" | "icons" | "emojis", string>;
  free: string[];
} = {
  index: "index.ts",
  required: {
    "skin.ts": { skin: "SKIN", bar: "BAR" },
    "icons/index.ts": { icons: "icons" },
    "emojis/index.ts": { emojis: "emojis" },
    "chrome.css": {},
  },
  optional: {
    "knobs.ts": { knobs: "KNOBS" },
    "lens.ts": { lens: "LENS" },
    "pulldown.ts": { pulldown: "pulldown" },
    "start.ts": { start: "start" },
    "tokens.css": {},
    "saver.css": {},
    "fonts/fonts.css": {},
  },
  each: { parts: "parts/*.tsx", icons: "icons/*.ts", emojis: "emojis/*.ts" },
  free: ["fonts", "own"],
};

export const lacks = (design: { parts: Partial<Record<Part, unknown>> }): Required[] =>
  REQUIRED.filter((part) => typeof design.parts[part] !== "function");
