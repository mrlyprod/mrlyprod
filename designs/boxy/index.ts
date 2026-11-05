import type { Design } from "@mrlyprod/ui/shell/design.ts";
import { App } from "./app.tsx";
import { Bar } from "./bar.tsx";
import { hued } from "./cell.ts";
import { Control } from "./control.tsx";
import { Dialog } from "./dialog.tsx";
import { Group } from "./group.tsx";
import { Launcher } from "./launcher.tsx";
import { Lock } from "./lock.tsx";
import { MEASURE } from "./measure.ts";
import { Menu } from "./menu.tsx";
import { Preview } from "./preview.tsx";
import { Prose } from "./prose.tsx";
import { Saver } from "./saver.tsx";
import { SKIN } from "./skin.ts";
import { Tile } from "./tile.tsx";
import { Widgets } from "./widgets.tsx";
import { Window } from "./window.tsx";

export const boxy: Design = {
  id: "boxy",
  title: "Boxy",
  skin: SKIN,
  bar: { light: "#ffffff", dark: "#000000" },
  css: ["boxy.css"],
  measure: MEASURE,
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
    widgets: Widgets,
    lock: Lock,
    saver: Saver,
    preview: Preview,
  },
};
