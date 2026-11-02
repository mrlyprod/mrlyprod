import type { Design } from "../../net/shell/design.ts";
import { App } from "./app.tsx";
import { Bar } from "./bar.tsx";
import { hued } from "./cell.ts";
import { Control } from "./control.tsx";
import { Dialog } from "./dialog.tsx";
import { Group } from "./group.tsx";
import { Launcher } from "./launcher.tsx";
import { Library } from "./library.tsx";
import { Lock } from "./lock.tsx";
import { MEASURE } from "./measure.ts";
import { Menu } from "./menu.tsx";
import { Prose } from "./prose.tsx";
import { Saver } from "./saver.tsx";
import { SKIN } from "./skin.ts";
import { Tile } from "./tile.tsx";
import { Widgets } from "./widgets.tsx";
import { Window } from "./window.tsx";

/* MRLY */

export const mrly: Design = {
  id: "mrly",
  title: "Mrly",
  skin: SKIN,
  bar: { light: "#ffffff", dark: "#000000" },
  css: ["mrly.css"],
  measure: MEASURE,
  pulldown: "pulldown.ts",
  start: hued,
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
    library: Library,
    widgets: Widgets,
    lock: Lock,
    saver: Saver,
  },
};
