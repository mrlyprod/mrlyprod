import type { Design, Knobs, Parts, World } from "@mrlyprod/ui/shell/design.ts";
import { App } from "./app.tsx";
import { Bar } from "./bar.tsx";
import { Control } from "./control.tsx";
import { Desk } from "./desk.tsx";
import { Dialog } from "./dialog.tsx";
import { Dock } from "./dock.tsx";
import { Group } from "./group.tsx";
import { Launcher } from "./launcher.tsx";
import { Lock } from "./lock.tsx";
import { MEASURE } from "./measure.ts";
import { Menu } from "./menu.tsx";
import { Prose } from "./prose.tsx";
import { Saver } from "./saver.tsx";
import { Tile } from "./tile.tsx";
import { Widgets } from "./widgets.tsx";
import { Window } from "./window.tsx";

const READ = new Set(["page", "wide"]);

function lens(root: HTMLElement) {
  const box = root.querySelector<HTMLElement>(".window");
  const set = () => {
    if (!box) return;
    if (READ.has(root.dataset.window ?? "")) box.dataset.lens = "window";
    else delete box.dataset.lens;
  };
  set();
  const eye = new MutationObserver(set);
  eye.observe(root, { attributeFilter: ["data-window"] });
  return () => eye.disconnect();
}

function start({ root }: World) {
  let stop: (() => void) | null = null;
  let gone = false;
  const unlens = lens(root);
  import("./engine/index.ts")
    .then((one) => {
      if (!gone) stop = one.start();
    })
    .catch((error) => console.error(error));
  return () => {
    gone = true;
    unlens();
    stop?.();
  };
}

const knobs: Knobs = {
  icons: true,
  own: [
    {
      id: "glass",
      name: "Glass",
      hint: "Glass bends and blurs the picture under the bar, dock, widgets, windows and panels. Reduce transparency keeps it solid.",
      options: [
        ["clear", "Clear"],
        ["", "Tinted"],
        ["frosted", "Frosted"],
        ["off", "Off"],
      ],
    },
    {
      id: "font",
      name: "Font",
      options: [
        ["", "Default"],
        ["sans", "Noto Sans"],
        ["serif", "Noto Serif"],
        ["mono", "Noto Sans Mono"],
        ["mrly", "MrlyFont"],
      ],
    },
  ],
};

const parts = {
  desk: Desk,
  bar: Bar,
  window: Window,
  app: App,
  menu: Menu,
  control: Control,
  group: Group,
  dialog: Dialog,
  prose: Prose,
  tile: Tile,
  launcher: Launcher,
  dock: Dock,
  widgets: Widgets,
  lock: Lock,
  saver: Saver,
} satisfies Parts;

export const liquid = {
  id: "liquid",
  title: "Liquid",
  skin: { light: {}, dark: {} },
  bar: { light: "#ffffff", dark: "#070707" },
  css: ["tokens.css", "chrome.css", "desk.css", "appicons.css", "saver.css"],
  measure: MEASURE,
  pulldown: "pulldown.ts",
  parts,
  knobs,
  start,
} satisfies Design;
