import type { Design } from "../../shell/design.ts";
import { App } from "./app.tsx";
import { Bar } from "./bar.tsx";
import { Control } from "./control.tsx";
import { Desk } from "./desk.tsx";
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

/* BLOCKS */

export const blocks: Design = {
  id: "blocks",
  title: "Blocks",
  skin: SKIN,
  css: ["blocks.css"],
  measure: MEASURE,
  pulldown: "pulldown.ts",
  parts: {
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
    library: Library,
    widgets: Widgets,
    lock: Lock,
    saver: Saver,
  },
};
