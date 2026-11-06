import type { ReactNode } from "react";
import { useRef, useState } from "react";
import { HUES } from "../../kit/color.ts";
import type { IconName, Tab } from "../../kit/controls.tsx";
import {
  ask,
  Btn,
  DateField,
  Empty,
  Group,
  Icon,
  Progress,
  Search,
  Segment,
  Sheet,
  Slider,
  Table,
  Tabs,
  Text,
  Toggle,
  Tooltip,
  toast,
} from "../../kit/controls.tsx";
import type { Part } from "../../shell/design.ts";

type Piece = { id: Part; title: string; body: ReactNode };

const TABS: Tab[] = [
  { id: "photos", label: "Photos", icon: "image" },
  { id: "files", label: "Files", icon: "folder" },
  { id: "code", label: "Code", icon: "code" },
];

const PANELS: Record<string, string> = {
  photos: "Arrow keys move between tabs. The panel follows the tab that has focus.",
  files: "Files saved from apps land here.",
  code: "Every page links to its source.",
};

const STORE: [string, number, number][] = [
  ["Patterns", 214, 1.2],
  ["Screen savers", 18, 0.34],
  ["Widgets", 6, 0.02],
  ["History", 1873, 4.75],
];

const ROUND: [IconName, string][] = [
  ["share", "Share"],
  ["download", "Download"],
  ["star", "Add to favourites"],
  ["trash", "Move to Trash"],
];

const SIZES: [string, string][] = [
  ["s", "Small"],
  ["m", "Medium"],
  ["l", "Large"],
];

const count = (n: number) => n.toLocaleString("en");

function SheetDemo() {
  const at = useRef<HTMLDialogElement>(null);
  const [name, setName] = useState("Moire");
  const [size, setSize] = useState("m");
  const [depth, setDepth] = useState(40);
  const [glow, setGlow] = useState<0 | 1>(1);
  const make = () => {
    at.current?.close();
    toast(`Made ${name}`, { kind: "ok" });
  };
  return (
    <div className="line">
      <Btn primary data-demo="sheet" onClick={() => at.current?.showModal()}>
        New pattern…
      </Btn>
      <Sheet ref={at} title="New pattern">
        <div className="controls">
          <Group name="Pattern">
            <Text label="Name" value={name} onChange={setName} />
            <Segment label="Size" value={size} options={SIZES} onChange={setSize} />
            <Slider label="Depth" value={depth} min={0} max={100} unit="%" onChange={setDepth} />
            <Toggle label="Glow" value={glow} onChange={setGlow} />
          </Group>
        </div>
        <Btn primary onClick={make}>
          Create
        </Btn>
      </Sheet>
    </div>
  );
}

function AlertDemo() {
  const [said, say] = useState("");
  const erase = async () => {
    const pick = await ask({
      title: "Erase all data?",
      text: "Settings and layouts kept in this browser go. This cannot be undone.",
      actions: [{ label: "Cancel" }, { label: "Erase", danger: true }],
    });
    say(pick ?? "Dismissed");
  };
  const close = async () => {
    const pick = await ask({
      title: "Save changes to Moire?",
      text: "Your changes are lost if you close without saving.",
      actions: [{ label: "Save", primary: true }, { label: "Don't Save", danger: true }, { label: "Cancel" }],
    });
    say(pick ?? "Dismissed");
  };
  return (
    <div className="line">
      <Btn data-demo="alert" onClick={erase}>
        Erase all data…
      </Btn>
      <Btn data-demo="alert-three" onClick={close}>
        Close document…
      </Btn>
      <span className="fine" aria-live="polite">
        {said}
      </span>
    </div>
  );
}

function TabsDemo() {
  const [tab, setTab] = useState("photos");
  return (
    <Tabs label="Library" items={TABS} value={tab} onChange={setTab}>
      <p className="fine">{PANELS[tab]}</p>
    </Tabs>
  );
}

function ToastDemo() {
  const undo = () => toast("Restored", { kind: "ok" });
  return (
    <div className="line">
      <Btn data-demo="toast" onClick={() => toast("Moved to Trash", { action: ["Undo", undo] })}>
        Delete
      </Btn>
      <Btn data-demo="toast-ok" onClick={() => toast("Link copied", { kind: "ok" })}>
        Copy link
      </Btn>
      <Btn data-demo="toast-warn" onClick={() => toast("Offline. Changes stay in this browser.", { kind: "warn" })}>
        Go offline
      </Btn>
    </div>
  );
}

function TooltipDemo() {
  return (
    <div className="line" data-demo="tooltip">
      {ROUND.map(([name, label]) => (
        <Tooltip key={name} text={label}>
          <Btn className="icon" aria-label={label}>
            <Icon name={name} />
          </Btn>
        </Tooltip>
      ))}
    </div>
  );
}

function SearchDemo() {
  const [find, setFind] = useState("");
  const want = find.trim().toLowerCase();
  const hits = HUES.filter((one) => one.includes(want));
  return (
    <div className="lanes">
      <div className="box stack">
        <Search
          label="Search colours"
          placeholder="Search colours"
          value={find}
          onChange={setFind}
          data-demo="search"
        />
        {hits.length ? (
          <p className="fine">{hits.join(", ")}</p>
        ) : (
          <Empty icon="search" title="No Results" text={`No colour is called ${find.trim()}.`} />
        )}
      </div>
    </div>
  );
}

function ProgressDemo() {
  const [done, setDone] = useState(60);
  return (
    <div className="lanes">
      <div className="stack">
        <Progress value={done} max={100} label={`Exporting, ${done}%`} />
        <div className="controls">
          <Slider label="Done" value={done} min={0} max={100} unit="%" onChange={setDone} />
        </div>
      </div>
      <Progress label="Loading" />
    </div>
  );
}

function EmptyDemo() {
  return (
    <div className="lanes">
      <div className="box stack">
        <Empty
          icon="folder"
          title="No Files"
          text="Files you save from apps show up here."
          action={["New File", () => toast("New file made", { kind: "ok" })]}
        />
      </div>
    </div>
  );
}

function DateDemo() {
  const [start, setStart] = useState("2026-11-06");
  const [end, setEnd] = useState("");
  return (
    <div className="stack" data-demo="date">
      <div className="line">
        <DateField label="Starts" value={start} onChange={setStart} min="2026-01-01" max="2027-12-31" />
      </div>
      <div className="line">
        <DateField label="Ends" value={end} onChange={setEnd} min={start} />
      </div>
    </div>
  );
}

export const PIECES: Piece[] = [
  { id: "sheet", title: "Sheet", body: <SheetDemo /> },
  { id: "alert", title: "Alert", body: <AlertDemo /> },
  { id: "tabs", title: "Tabs", body: <TabsDemo /> },
  {
    id: "table",
    title: "Table",
    body: (
      <Table
        caption="Kept in this browser"
        head={["Item", "Count", "Size"]}
        align={["start", "end", "end"]}
        rows={STORE.map(([name, n, mb]) => [name, count(n), `${mb.toFixed(2)} MB`])}
      />
    ),
  },
  { id: "toast", title: "Toast", body: <ToastDemo /> },
  { id: "tooltip", title: "Tooltip", body: <TooltipDemo /> },
  { id: "search", title: "Search", body: <SearchDemo /> },
  { id: "progress", title: "Progress", body: <ProgressDemo /> },
  { id: "empty", title: "Empty", body: <EmptyDemo /> },
  { id: "calendar", title: "Date", body: <DateDemo /> },
];
