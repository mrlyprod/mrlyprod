import type { Design, Knob } from "@mrlyprod/ui/shell/design.ts";
import { App } from "./app.tsx";
import { Bar } from "./bar.tsx";
import { Control } from "./control.tsx";
import { Desk } from "./desk.tsx";
import { Dialog } from "./dialog.tsx";
import { Dock } from "./dock.tsx";
import { wear } from "./face.ts";
import { Group } from "./group.tsx";
import { Launcher } from "./launcher.tsx";
import { Lock } from "./lock.tsx";
import { MEASURE } from "./measure.ts";
import { Menu } from "./menu.tsx";
import { Preview } from "./preview.tsx";
import { Prose } from "./prose.tsx";
import { Saver } from "./saver.tsx";
import { DARK, LIGHT } from "./skin.ts";
import { start } from "./start.ts";
import { Tile } from "./tile.tsx";
import { Window } from "./window.tsx";

wear();

export const KNOBS: Knob[] = [
  {
    id: "sparks",
    name: "Sparks",
    options: [
      ["", "High"],
      ["low", "Low"],
      ["off", "Off"],
    ],
    hint: "Pixels fly on every press, swipe and key; off when motion is reduced.",
  },
  {
    id: "palette",
    name: "Palette",
    options: [
      ["", "Arcade"],
      ["pocket", "Pocket"],
      ["neon", "Neon"],
      ["fire", "Fire"],
    ],
  },
  {
    id: "scanlines",
    name: "Scanlines",
    options: [
      ["", "On"],
      ["off", "Off"],
    ],
  },
  {
    id: "sound",
    name: "Sound",
    options: [
      ["", "On"],
      ["off", "Off"],
    ],
    hint: "Blips start after the first press.",
  },
  {
    id: "attract",
    name: "Screen saver",
    options: [
      ["", "Maze"],
      ["stars", "Stars"],
      ["apps", "App savers"],
    ],
  },
];

export const pixel: Design = {
  id: "pixel",
  title: "Pixel",
  skin: { light: LIGHT, dark: DARK },
  bar: { light: "#16112e", dark: "#000000" },
  css: ["pixel.css"],
  measure: MEASURE,
  pulldown: "pulldown.ts",
  knobs: { own: KNOBS },
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
    lock: Lock,
    saver: Saver,
    preview: Preview,
  },
  start,
};
