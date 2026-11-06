import type { Site } from "site:routes";
import { site as SITE } from "site:routes";
import type { ChangeEvent } from "react";
import { useEffect, useRef, useState } from "react";
import { ask, sure } from "../../kit/controls.tsx";
import { design } from "../active.ts";
import { STORE } from "../password.ts";
import { dial, KNOBBED, PREFIX, stored, subscribe, write } from "../prefs.ts";
import { Password } from "./password.tsx";
import { Action, Item, Note, Section } from "./ui.tsx";

export const FIND = [
  "privacy",
  "password",
  "lock screen",
  "data",
  "storage",
  "export",
  "import",
  "reset",
  "erase",
  "cookies",
  "analytics",
  "tracking",
  "kept in this browser",
  "what the site sends",
];

const NAMES: Partial<Record<string, string>> = {
  design: "Design",
  theme: "Appearance",
  tint: "Accent colour",
  icons: "Icon style",
  "icons-always": "Icons always dark",
  magnify: "Dock magnification",
  text: "Text size",
  motion: "Reduce motion",
  clear: "Reduce transparency",
  contrast: "Increase contrast",
  targets: "Larger targets",
  saver: "Desktop picture",
  "savers-off": "Screen saver picks",
  idle: "Screen saver delay",
  clock: "Screen saver clock",
  cart: "Cart",
  welcome: "Lock screen seen",
  password: "Lock screen password",
  launchpad: "Launchpad layout",
  recent: "Recent apps",
  dock: "Dock",
  widgets: "Widgets",
  sprite: "Current pattern",
  "designs-recent": "Recent patterns",
  "designs-saved": "Saved patterns",
  panes: "Open side panels",
  world: "World",
  journal: "History",
  history: "Acts kept",
};

const KNOBS = new Map((design.knobs?.own ?? []).map((one) => [dial(one.id), one.name]));

const TITLES = new Map(SITE.designs.map((one) => [one.id, one.title]));

const name = (key: string) => {
  const id = key.slice(PREFIX.length);
  const [, owner, knob] = KNOBBED.exec(id) ?? [];
  if (knob) return KNOBS.get(key) ?? `${TITLES.get(owner) ?? owner} design: ${knob}`;
  return NAMES[id] ?? id;
};

const bytes = (text: string) => new TextEncoder().encode(text).length;

const size = (n: number) => (n < 1024 ? `${n} B` : `${(n / 1024).toFixed(1)} KB`);

const count = (n: number) => `${n} ${n === 1 ? "item" : "items"}`;

function save() {
  const text = JSON.stringify(Object.fromEntries(stored().filter(([key]) => key !== STORE)), null, 2);
  const url = URL.createObjectURL(new Blob([text], { type: "application/json" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = `${PREFIX}data.json`;
  document.body.append(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

function parse(text: string): [string, string][] | null {
  try {
    const data: unknown = JSON.parse(text);
    if (!data || typeof data !== "object" || Array.isArray(data)) return null;
    const rows: [string, unknown][] = Object.entries(data).filter(([key]) => key !== STORE);
    return rows.length &&
      rows.every((row): row is [string, string] => row[0].startsWith(PREFIX) && typeof row[1] === "string")
      ? rows
      : null;
  } catch {
    return null;
  }
}

function wipe() {
  for (const area of ["localStorage", "sessionStorage"] as const) {
    try {
      const store = window[area];
      for (const key of Object.keys(store))
        if (key.startsWith(PREFIX) || key.startsWith("knobs:")) store.removeItem(key);
    } catch {}
  }
  location.replace("/settings/privacy/");
}

function useKept() {
  const [list, setList] = useState(stored);
  useEffect(() => {
    setList(stored());
    return subscribe(() => setList(stored()));
  }, []);
  return list;
}

async function erase() {
  if (await sure({ title: "Erase all data?", text: "This cannot be undone." }, { label: "Erase", danger: true }))
    wipe();
}

export default function Pane({ site }: { site: Site }) {
  const host = new URL(site.root).host;
  const list = useKept();
  const input = useRef<HTMLInputElement>(null);
  const total = list.reduce((sum, [, value]) => sum + bytes(value), 0);

  async function choose(event: ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    event.target.value = "";
    if (!file) return;
    const rows = parse(await file.text().catch(() => ""));
    if (!rows) return void ask({ title: `That file is not a ${site.title} data export.` });
    const asked = {
      title: `Import ${count(rows.length)}?`,
      text: "They replace matching items here. The page reloads.",
    };
    if (!(await sure(asked, { label: "Import" }))) return;
    for (const [key, value] of rows) write(key, value);
    location.reload();
  }

  return (
    <>
      <Password />
      <Section title="Kept in this browser" foot={list.length ? `${count(list.length)}, ${size(total)}` : undefined}>
        <Note>Settings and layouts stay in this browser. There is no account.</Note>
        {list.length === 0 && <Note>Nothing is kept yet.</Note>}
        {list.map(([key, value]) => (
          <Item key={key} label={name(key)}>
            <span className="num dim">{size(bytes(value))}</span>
          </Item>
        ))}
        <Note>Scroll positions and open Knobs sections last only for this tab.</Note>
      </Section>
      <Section title="Export and import">
        <Action
          label="Export all data"
          hint={`Saves everything above, except the password, as ${PREFIX}data.json.`}
          onClick={save}
        >
          Export
        </Action>
        <Action label="Import data" hint={`Choose a ${PREFIX}data.json file.`} onClick={() => input.current?.click()}>
          Import…
        </Action>
      </Section>
      <Section title="Reset">
        <Action
          danger
          label="Erase all data"
          hint="Removes everything kept in this browser. The lock screen plays again on your next visit home."
          onClick={erase}
        >
          Erase…
        </Action>
      </Section>
      <Section title="What the site sends">
        <Item label="Cookies">
          <span className="dim">None</span>
        </Item>
        <Item label="Analytics and trackers">
          <span className="dim">None</span>
        </Item>
        <Item label="Other sites" hint={`Pages, fonts, code and data all load from ${host}.`}>
          <span className="dim">None</span>
        </Item>
        <Item label="Your settings" hint="Except inside a settings link you copy and share.">
          <span className="dim">Stay here</span>
        </Item>
        <Note>
          Like any web host, the servers behind {host} see your IP address and the pages you request, in order to
          deliver them. The Stats page shows totals per hour, not visitors.
        </Note>
        <Note>
          Links to other sites, such as GitHub, donations and social profiles, open on those sites, which have their own
          rules.
        </Note>
      </Section>
      <input ref={input} type="file" accept="application/json,.json" hidden onChange={choose} />
    </>
  );
}
