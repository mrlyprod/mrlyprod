import { useRef, useState } from "react";
import { blend, HUES } from "../../kit/color.ts";
import type { MenuRow } from "../../kit/controls.tsx";
import {
  Btn,
  Empty,
  Export,
  Fold,
  Group,
  Icon,
  Menu,
  Modal,
  Pick,
  Search,
  Segment,
  Slider,
  Text,
  Toggle,
  Tooltip,
} from "../../kit/controls.tsx";
import { flip } from "../../kit/keys.tsx";
import { Knobs } from "../../kit/knobs.tsx";
import { Art, Item, Shelf } from "../../kit/library.tsx";
import type { Scalar, Spec } from "../../kit/spec.ts";
import { defaults } from "../../kit/spec.ts";
import { palette } from "../../kit/theme/palette.gen.js";
import type { Name } from "../../kit/theme/theme.ts";

export type Inputs = Record<"slider" | "toggle" | "segment" | "pick" | "text", Scalar>;

export type Shade = { id: string; name: string; hex: string; warm: boolean };

const MODES: [string, string][] = [
  ["soft", "Soft"],
  ["hard", "Hard"],
  ["even", "Even"],
];

const EDGES: [string, string][] = [
  ["sharp", "Sharp"],
  ["round", "Round"],
];

const PARTS: [string, string][] = [
  ["a", "Alpha"],
  ["b", "Beta"],
  ["c", "Gamma"],
];

export const SPEC: Spec = [
  { key: "size", label: "Size", kind: "slider", def: 40, min: 0, max: 100, step: 1, unit: "%", group: "Numbers" },
  { key: "angle", label: "Angle", kind: "number", def: 15, min: -180, max: 180, step: 5, group: "Numbers" },
  { key: "glow", label: "Glow", kind: "toggle", def: 1, group: "Numbers" },
  { key: "mode", label: "Mode", kind: "pick", def: "soft", options: MODES, group: "Choices" },
  { key: "edge", label: "Edge", kind: "segment", def: "round", options: EDGES, group: "Choices" },
  { key: "parts", label: "Parts", kind: "list", def: "a.b", options: PARTS, group: "Choices" },
  { key: "name", label: "Name", kind: "text", def: "kit", group: "Choices" },
];

const START: Inputs = { slider: 40, toggle: 1, segment: "b", pick: "two", text: "Hello" };

const COUNT: [string, string][] = [
  ["a", "One"],
  ["b", "Two"],
  ["c", "Three"],
];

const NAMES: [string, string][] = [
  ["one", "One"],
  ["two", "Two"],
  ["three", "Three"],
];

export function Controls() {
  const [v, set] = useState(START);
  const [knobs, setKnobs] = useState(() => defaults(SPEC));
  const [said, say] = useState("");
  const modal = useRef<HTMLDialogElement>(null);
  const edit = (key: keyof Inputs) => (next: Scalar) => set((was) => ({ ...was, [key]: next }));
  const rows = ["Copy", "Paste", "Reset"].map(
    (text): MenuRow => [text, () => say(`Menu: ${text}`), text === "Copy" ? "link" : undefined],
  );
  return (
    <>
      <div className="lanes">
        <div className="controls">
          <Group name="Inputs">
            <Slider label="Slider" value={v.slider} min={0} max={100} unit="%" onChange={edit("slider")} />
            <Toggle label="Toggle" value={v.toggle} onChange={edit("toggle")} />
            <Segment label="Segment" value={v.segment} options={COUNT} onChange={edit("segment")} />
            <Pick label="Pick" value={v.pick} options={NAMES} onChange={edit("pick")} />
            <Text label="Text" value={v.text} onChange={edit("text")} />
          </Group>
          <Group name="Buttons">
            <Btn>Plain</Btn>
            <Btn on>On</Btn>
            <Btn primary>Primary</Btn>
            <Btn>
              <Icon name="download" />
              With icon
            </Btn>
          </Group>
          <Fold name="Fold" live={`${v.slider}%`}>
            <Toggle label="Inside" value={v.toggle} onChange={edit("toggle")} />
            <Slider label="Also inside" value={v.slider} min={0} max={100} onChange={edit("slider")} />
          </Fold>
        </div>
        <div className="controls">
          <Knobs spec={SPEC} value={knobs} onChange={(patch) => setKnobs((was) => ({ ...was, ...patch }))} id="kit" />
        </div>
      </div>
      <div className="line">
        <Btn primary>Primary</Btn>
        <Btn on>On</Btn>
        <Btn>Plain</Btn>
        <Tooltip text="Info">
          <Btn className="icon" aria-label="Info">
            <Icon name="info" />
          </Btn>
        </Tooltip>
        <Btn disabled>Disabled</Btn>
        <Menu rows={rows}>Menu</Menu>
        <Btn onClick={() => modal.current?.showModal()}>Modal</Btn>
        <Btn onClick={() => flip("dialog.export")}>Export</Btn>
        <span className="fine" aria-live="polite">
          {said}
        </span>
      </div>
      <Modal ref={modal} title="Modal">
        <p className="fine">A dialog with a close button. Escape closes it.</p>
      </Modal>
      <Export
        canvas={() => document.querySelector<HTMLCanvasElement>(".kit .backdrop > canvas")}
        name="kit"
        more={[["Say hello", () => say("Export: hello")]]}
      />
    </>
  );
}

const WARM = ["red", "orange", "yellow", "pink", "brown"];

const FILTERS = [
  ["all", "All"],
  ["warm", "Warm"],
  ["cool", "Cool"],
];

const SHADES: Shade[] = HUES.map((id) => ({
  id,
  name: id[0].toUpperCase() + id.slice(1),
  hex: palette[id as Name],
  warm: WARM.includes(id),
}));

const paint = (hex: string) => (ctx: CanvasRenderingContext2D, w: number, h: number) => {
  const fill = ctx.createLinearGradient(0, 0, w, h);
  fill.addColorStop(0, hex);
  fill.addColorStop(1, blend(hex, "#000000", 0.45));
  ctx.fillStyle = fill;
  ctx.fillRect(0, 0, w, h);
};

export function Library() {
  const [find, setFind] = useState("");
  const [kind, setKind] = useState("all");
  const [on, setOn] = useState("blue");
  const want = find.trim().toLowerCase();
  const shown = SHADES.filter((one) => !want || one.name.toLowerCase().includes(want) || one.hex.includes(want));
  const item = (one: Shade) => (
    <Item
      key={one.id}
      label={one.name}
      sub={one.hex}
      on={one.id === on}
      onPick={() => setOn(one.id)}
      art={<Art paint={paint(one.hex)} />}
    />
  );
  const warm = kind === "cool" ? [] : shown.filter((one) => one.warm);
  const cool = kind === "warm" ? [] : shown.filter((one) => !one.warm);
  return (
    <div className="lanes">
      <div className="library box">
        <h2>Library</h2>
        <Search label="Search Library" value={find} placeholder="Search" onChange={setFind} />
        <div className="chips" role="radiogroup" aria-label="Library filter">
          {FILTERS.map(([v, text]) => (
            <button
              type="button"
              key={v}
              role="radio"
              aria-checked={v === kind}
              className={v === kind ? "on" : undefined}
              onClick={() => setKind(v)}
            >
              {text}
            </button>
          ))}
        </div>
        {warm.length > 0 && <Shelf name="Warm">{warm.map(item)}</Shelf>}
        {cool.length > 0 && (
          <Shelf name="Cool" grid>
            {cool.map(item)}
          </Shelf>
        )}
        {warm.length + cool.length === 0 && <Empty icon="search" title="No Results" />}
      </div>
    </div>
  );
}
