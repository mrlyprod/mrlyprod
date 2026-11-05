import type { Site } from "site:routes";
import type { Saver } from "../../kit/rows.ts";
import { AppIcon } from "../../shell/appicon.tsx";
import { app as routed } from "../../shell/catalog.ts";
import { clock, idle, KEY, offs, savers } from "../../shell/prefs.ts";
import { PLAIN } from "../../shell/tiles.ts";
import type { Choice } from "./ui.tsx";
import { Action, Glyph, Item, Nav, Note, Pane as Page, Section, Select, Switch } from "./ui.tsx";
import { usePref } from "./use.ts";

type RowProps = { one: Saver; on: boolean; flip: (id: string, on: boolean) => void };

export const FIND = ["Screen saver", "Screensaver", "Idle", "Start after", "Show clock", "Shuffle", "Savers"];

const WAITS: Choice[] = [
  ["2", "2 minutes"],
  ["", "5 minutes"],
  ["10", "10 minutes"],
  ["never", "Never"],
];

const known = (wait: string) => (WAITS.some(([value]) => value === wait) ? wait : "");

const play = (only?: string) => import("../../shell/idle.ts").then(({ start }) => start({ only }));

function Face({ app }: { app: string }) {
  if (routed(app)) return <AppIcon id={app} className="screensaver-face" />;
  return (
    <span className="screensaver-face screensaver-plain" aria-hidden="true">
      <Glyph art={PLAIN} />
    </span>
  );
}

function Row({ one, on, flip }: RowProps) {
  const name = (
    <span className="screensaver-name">
      <Face app={one.app} />
      <span>
        {one.title}
        {one.title !== one.name && <small>{one.name}</small>}
      </span>
    </span>
  );
  return (
    <Item label={name}>
      <span className="screensaver-do">
        <button
          type="button"
          className="screensaver-peek"
          aria-label={`Preview ${one.title}`}
          onClick={() => play(one.id)}
        >
          Preview
        </button>
        <input
          type="checkbox"
          role="switch"
          title="In shuffle"
          aria-label={`${one.title} in shuffle`}
          checked={on}
          aria-checked={on}
          onChange={(e) => flip(one.id, e.target.checked)}
        />
      </span>
    </Item>
  );
}

export default function Pane({ site }: { site: Site }) {
  const wait = known(usePref(KEY.idle));
  const shown = usePref(KEY.clock) !== "off";
  const off = new Set(usePref(KEY.off) ? offs() : []);
  const list = site.savers ?? [];
  const rows = [...new Set(list.map((one) => one.app))].flatMap((app) => list.filter((one) => one.app === app));
  const count = list.filter((one) => !off.has(one.id)).length;
  const many = list.length > 1;
  const flip = (id: string, on: boolean) => savers(on ? [...off].filter((one) => one !== id) : [...off, id]);
  return (
    <Page title="Screen Saver">
      <Section>
        <Select label="Start after" value={wait} options={WAITS} onChange={idle} />
        <Switch label="Show clock" value={shown} onChange={(on) => clock(on ? "" : "off")} />
        {count > 0 && (
          <Action label="Show the screen saver now" onClick={() => play()}>
            Preview
          </Action>
        )}
      </Section>
      <Section
        title="Savers"
        foot="Savers take turns at random, a new one every 90 seconds. Any key or click wakes the screen."
      >
        {count === 0 && <Note>No saver is on, so the screen saver never starts.</Note>}
        {rows.map((one) => (
          <Row key={one.id} one={one} on={!off.has(one.id)} flip={flip} />
        ))}
        {many && count < list.length && <Action onClick={() => savers([])}>Turn all on</Action>}
        {many && count > 0 && <Action onClick={() => savers(list.map((one) => one.id))}>Turn all off</Action>}
      </Section>
      <Section>
        <Nav label="Screensavers app" hint="Preview and reroll each saver" href="/screensavers/" />
      </Section>
    </Page>
  );
}
