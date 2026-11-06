import type { Site } from "site:routes";
import type { ReactNode } from "react";
import type { Saver } from "../../kit/rows.ts";
import { design, parts } from "../active.ts";
import { AppIcon } from "../appicon.tsx";
import { app, KNOWN, PAD, redock } from "../catalog.ts";
import { desk } from "../desk.ts";
import { docked } from "../launch.ts";
import { KEY, magnify, screen, write } from "../prefs.ts";
import { PLAIN } from "../tiles.ts";
import { kept, WIDGETS } from "../widgets.tsx";
import type { Choice } from "./ui.tsx";
import { Action, Glyph, Nav, Note, Section, Select, Switch, Tiles } from "./ui.tsx";
import { usePref } from "./use.ts";

const DOCK = Boolean(parts.dock);

const LENS = DOCK && Boolean(design.lens);

export const FIND = [
  ...(parts.desk ? ["Wallpaper", "Desktop picture", "Random"] : []),
  ...(parts.widgets ? ["Widgets"] : []),
  ...(DOCK ? ["Dock", "Dock apps", "Add to Dock", "Remove from Dock", "Reset dock"] : []),
  ...(LENS ? ["Magnify"] : []),
  "Launchpad",
];

const DICE = "#...# ..... ..#.. ..... #...#";

function Mark({ art }: { art: string }) {
  return (
    <span className="desk-mark">
      <Glyph art={art} />
    </span>
  );
}

const Art = ({ children }: { children: ReactNode }) => <span className="desk-art">{children}</span>;

const caption = (one: Saver) => (one.title && one.title !== one.name ? `${one.name} · ${one.title}` : one.name);

function choices(list: Saver[]) {
  return [
    {
      value: "random",
      text: "Random",
      art: (
        <Art>
          <Mark art={DICE} />
        </Art>
      ),
    },
    ...list.map((one) => ({
      value: one.id,
      text: caption(one),
      art: <Art>{app(one.app) ? <AppIcon id={one.app} /> : <Mark art={PLAIN} />}</Art>,
    })),
  ];
}

function chosen(site: Site, stored: string) {
  const list = site.savers ?? [];
  const head = stored.split("?")[0];
  if (!head) return site.desk ?? "random";
  if (head === "random" || list.some((one) => one.id === head)) return head;
  return list.find((one) => one.app === head)?.id ?? "random";
}

function pick(id: string) {
  screen(id);
  desk();
}

function Picture({ site }: { site: Site }) {
  const stored = usePref(KEY.saver);
  return (
    <Section
      title="Desktop picture"
      foot="The desktop is a still frame of a screen saver. Random picks a new one each visit."
    >
      <Tiles
        label="Desktop picture"
        value={chosen(site, stored)}
        options={choices(site.savers ?? [])}
        onChange={pick}
      />
    </Section>
  );
}

function Widgets() {
  const list = kept(usePref(KEY.widgets));
  const flip = (id: string, on: boolean) =>
    write(KEY.widgets, JSON.stringify(on ? [...list, id] : list.filter((one) => one !== id)));
  return (
    <Section title="Widgets">
      {WIDGETS.map((one) => (
        <Switch key={one.id} label={one.name} value={list.includes(one.id)} onChange={(on) => flip(one.id, on)} />
      ))}
      <Nav label="Edit widgets on the desktop" href="/" />
    </Section>
  );
}

const title = (id: string) => app(id)?.title ?? id;

function Apps() {
  const groups = docked(PAD.dock, usePref(KEY.dock), KNOWN);
  const ids = groups.flat();
  const spare = [...KNOWN].filter((id) => !ids.includes(id)).sort((a, b) => title(a).localeCompare(title(b)));
  const drop = (id: string) => redock(groups.map((one) => one.filter((each) => each !== id)));
  const add = (id: string) => id && redock(groups.length ? groups.map((one, g) => (g ? one : [...one, id])) : [[id]]);
  return (
    <Section
      title="Dock apps"
      foot="An added app joins the end of the first group. Drag apps in the dock to reorder them."
    >
      {ids.map((id) => (
        <Action key={id} label={title(id)} onClick={() => drop(id)}>
          Remove
        </Action>
      ))}
      {!ids.length && <Note>The dock holds only the Launchpad.</Note>}
      {spare.length > 0 && (
        <Select
          label="Add to Dock"
          value=""
          options={[["", "Choose an app"], ...spare.map((id): Choice => [id, title(id)])]}
          onChange={add}
        />
      )}
    </Section>
  );
}

function Dock() {
  const on = usePref(KEY.magnify) === "1";
  return (
    <Section title="Dock">
      {LENS && <Switch label="Magnify" hint="Icons grow under the pointer." value={on} onChange={magnify} />}
      <Action label="Reset dock" hint="Put the default apps back in the dock." onClick={() => write(KEY.dock, "")}>
        Reset
      </Action>
      <Nav label="Arrange apps in the Launchpad" href="/menu/" />
    </Section>
  );
}

export default function Pane({ site }: { site: Site }) {
  return (
    <>
      {parts.desk && <Picture site={site} />}
      {parts.widgets && <Widgets />}
      {DOCK && <Dock />}
      {DOCK && <Apps />}
    </>
  );
}
