import { useRef, useState } from "react";
import {
  Btn,
  Export,
  Fold,
  Group,
  Icon,
  Menu,
  Modal,
  Pick,
  Segment,
  Slider,
  Text,
  Toggle,
} from "../../lib/app/controls.jsx";
import { useUnit } from "../../lib/app/hooks.js";
import { flip } from "../../lib/app/keys.jsx";
import { Knobs } from "../../lib/app/knobs.jsx";
import { Art, Item, Shelf } from "../../lib/app/library.jsx";
import { defaults } from "../../lib/app/spec.js";
import { blend, HUES } from "../../lib/color.js";
import { Browser } from "../../lib/designs/browse.jsx";
import { Chip } from "../../lib/designs/chip.jsx";
import { place } from "../../lib/designs/store.js";
import { palette } from "../../lib/theme/palette.gen.js";

/* KNOBS */

const MODES = [
  ["soft", "Soft"],
  ["hard", "Hard"],
  ["even", "Even"],
];

const EDGES = [
  ["sharp", "Sharp"],
  ["round", "Round"],
];

const PARTS = [
  ["a", "Alpha"],
  ["b", "Beta"],
  ["c", "Gamma"],
];

export const SPEC = [
  { key: "size", label: "Size", kind: "slider", def: 40, min: 0, max: 100, step: 1, unit: "%", group: "Numbers" },
  { key: "angle", label: "Angle", kind: "number", def: 15, min: -180, max: 180, step: 5, group: "Numbers" },
  { key: "glow", label: "Glow", kind: "toggle", def: 1, group: "Numbers" },
  { key: "mode", label: "Mode", kind: "pick", def: "soft", options: MODES, group: "Choices" },
  { key: "edge", label: "Edge", kind: "segment", def: "round", options: EDGES, group: "Choices" },
  { key: "parts", label: "Parts", kind: "list", def: "a.b", options: PARTS, group: "Choices" },
  { key: "name", label: "Name", kind: "text", def: "mrly", group: "Choices" },
];

/* CONTROLS */

const START = { slider: 40, toggle: 1, segment: "b", pick: "two", text: "Hello" };

const COUNT = [
  ["a", "One"],
  ["b", "Two"],
  ["c", "Three"],
];

const NAMES = [
  ["one", "One"],
  ["two", "Two"],
  ["three", "Three"],
];

export function Controls() {
  const [v, set] = useState(START);
  const [knobs, setKnobs] = useState(() => defaults(SPEC));
  const [said, say] = useState("");
  const modal = useRef(null);
  const edit = (key) => (next) => set((was) => ({ ...was, [key]: next }));
  const rows = ["Copy", "Paste", "Reset"].map((text) => [
    text,
    () => say(`Menu: ${text}`),
    text === "Copy" ? "link" : undefined,
  ]);
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
        <Btn className="icon" aria-label="Info" title="Icon">
          <Icon name="info" />
        </Btn>
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
        canvas={() => document.querySelector(".kit .backdrop > canvas")}
        name="kit"
        more={[["Say hello", () => say("Export: hello")]]}
      />
    </>
  );
}

/* LIBRARY */

const WARM = ["red", "orange", "yellow", "pink", "brown"];

const FILTERS = [
  ["all", "All"],
  ["warm", "Warm"],
  ["cool", "Cool"],
];

const SHADES = HUES.map((id) => ({
  id,
  name: id[0].toUpperCase() + id.slice(1),
  hex: palette[id],
  warm: WARM.includes(id),
}));

const paint = (hex) => (ctx, w, h) => {
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
  const item = (one) => (
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
        <label className="find">
          <Icon name="search" />
          <input
            type="search"
            value={find}
            placeholder="Search"
            aria-label="Search Library"
            onChange={(e) => setFind(e.target.value)}
          />
        </label>
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
      </div>
    </div>
  );
}

/* DESIGNS */

const SLOT = { dim: "dim", base: "base", code: "code" };

export function Designs() {
  const math = useUnit("math");
  const [design, setDesign] = useState({ dim: 2, base: 2, code: "" });
  const pick = (patch) => setDesign((was) => ({ ...was, ...patch }));
  return (
    <div className="lanes">
      <div className="line">
        <div className="frost chipbar">
          <Chip slot={SLOT} value={design} onChange={pick} math={math} />
        </div>
        <span className="fine">{`${design.dim}D, base ${design.base}, ${design.code ? `code ${design.code}` : "home design"}`}</span>
      </div>
      <div className="box tall">
        <Browser compact slot={SLOT} value={design} math={math} onPick={(one) => pick(place(SLOT, one))} />
      </div>
    </div>
  );
}
