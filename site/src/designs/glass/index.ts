import type { Design, Parts, World } from "../../shell/design.ts";
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

/* START */

function start(_world: World) {
  let stop: (() => void) | null = null;
  let gone = false;
  import("./engine/index.ts")
    .then((one) => {
      if (!gone) stop = one.start();
    })
    .catch((error) => console.error(error));
  return () => {
    gone = true;
    stop?.();
  };
}

/* GLASS */

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

export const glass = {
  id: "glass",
  title: "Glass",
  skin: { light: {}, dark: {} },
  css: ["tokens.css", "chrome.css", "desk.css", "appicons.css", "saver.css"],
  measure: MEASURE,
  pulldown: "pulldown.ts",
  parts,
  start,
} satisfies Design;
