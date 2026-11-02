import type { Design } from "../../net/shell/design.ts";
import { App } from "./app.tsx";
import { Bar } from "./bar.tsx";
import { Control } from "./control.tsx";
import { Desk } from "./desk.tsx";
import { Dialog } from "./dialog.tsx";
import { Dock } from "./dock.tsx";
import { Group } from "./group.tsx";
import { Launcher } from "./launcher.tsx";
import { Library } from "./library.tsx";
import { Lock } from "./lock.tsx";
import { MEASURE } from "./measure.ts";
import { Menu } from "./menu.tsx";
import { Prose } from "./prose.tsx";
import { Saver } from "./saver.tsx";
import { DARK, LIGHT } from "./skin.ts";
import { Tile } from "./tile.tsx";
import { Widgets } from "./widgets.tsx";
import { Window } from "./window.tsx";

/* CLASSIC */

export const terminal: Design = {
  id: "terminal",
  title: "Terminal",
  skin: { light: LIGHT, dark: DARK },
  bar: { light: "#1f1d18", dark: "#123a22" },
  css: ["terminal.css"],
  measure: MEASURE,
  pulldown: "pulldown.ts",
  parts: {
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
    desk: Desk,
    dock: Dock,
    widgets: Widgets,
    library: Library,
    lock: Lock,
    saver: Saver,
  },
};
