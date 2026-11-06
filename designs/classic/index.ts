import type { Design } from "@mrlyprod/ui/shell/design.ts";
import { emojis } from "./emojis/index.ts";
import { icons } from "./icons/index.ts";
import { Alert } from "./parts/alert.tsx";
import { App } from "./parts/app.tsx";
import { Bar } from "./parts/bar.tsx";
import { Calendar } from "./parts/calendar.tsx";
import { Control } from "./parts/control.tsx";
import { Desk } from "./parts/desk.tsx";
import { Dialog } from "./parts/dialog.tsx";
import { Dock } from "./parts/dock.tsx";
import { Empty } from "./parts/empty.tsx";
import { Group } from "./parts/group.tsx";
import { Launcher } from "./parts/launcher.tsx";
import { Lock } from "./parts/lock.tsx";
import { Menu } from "./parts/menu.tsx";
import { Preview } from "./parts/preview.tsx";
import { Progress } from "./parts/progress.tsx";
import { Prose } from "./parts/prose.tsx";
import { Saver } from "./parts/saver.tsx";
import { Search } from "./parts/search.tsx";
import { Sheet } from "./parts/sheet.tsx";
import { Table } from "./parts/table.tsx";
import { Tabs } from "./parts/tabs.tsx";
import { Tile } from "./parts/tile.tsx";
import { Toast } from "./parts/toast.tsx";
import { Tooltip } from "./parts/tooltip.tsx";
import { Widgets } from "./parts/widgets.tsx";
import { Window } from "./parts/window.tsx";
import { pulldown } from "./pulldown.ts";
import { BAR, SKIN } from "./skin.ts";

export const design: Design = {
  id: "classic",
  title: "Classic",
  skin: SKIN,
  bar: BAR,
  icons,
  emojis,
  pulldown,
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
    sheet: Sheet,
    alert: Alert,
    tabs: Tabs,
    table: Table,
    toast: Toast,
    tooltip: Tooltip,
    search: Search,
    progress: Progress,
    empty: Empty,
    calendar: Calendar,
    desk: Desk,
    dock: Dock,
    widgets: Widgets,
    lock: Lock,
    saver: Saver,
    preview: Preview,
  },
};
