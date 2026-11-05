import type { ComponentType } from "react";

export type Row = {
  route: string;
  kind: "app" | "prose";
  title: string;
  source: string | null;
  date: string | null;
  lead: string;
  figure: string | null;
  hidden: boolean;
  meta: {
    id?: string;
    scene?: true;
    deep?: true;
    prose?: "source" | "page" | "post";
    date?: string;
    aliases?: string[];
  };
};

export type App = {
  id: string;
  dir: string;
  title: string;
  line: string;
  aliases?: string[];
  saver?: true;
  hidden?: true;
  deep?: true;
  source?: string;
  route?: string;
  page: string | null;
  icon: boolean;
};

export type Topic = {
  slug: string;
  title: string;
  date: string;
  lead: string;
  figure: string | null;
  figures: string[];
};

export type Saver = { id: string; app: string; name: string; title: string };

export type Host = { id: string; title: string; saver?: true; file: string };

export type Shelf = { name: string; apps: string[] };

export type Pad = { sections: Shelf[]; dock: string[][] };

export type Desk = { name: string; size: "wide" | "small"; Body: ComponentType };
