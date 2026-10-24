import { AppIcon } from "../../shell/appicon.jsx";
import { clock, idle, KEY, offs, savers } from "../../shell/prefs.js";
import { Action, Item, Nav, Note, Pane as Page, Section, Select, Switch } from "./ui.jsx";
import { usePref } from "./use.js";

export const FIND = ["Screen saver", "Screensaver", "Idle", "Start after", "Show clock", "Shuffle", "Savers"];

/* START */

const WAITS = [
  ["2", "2 minutes"],
  ["", "5 minutes"],
  ["10", "10 minutes"],
  ["never", "Never"],
];

const known = (wait) => (WAITS.some(([value]) => value === wait) ? wait : "");

const play = (only) => import("../../shell/idle.js").then(({ start }) => start({ only }));

/* FACE */

const TILE = "#.#.# .#.#. #.#.# .#.#. #.#.#".split(" ");

function Face({ app }) {
  if (app !== "mrly") return <AppIcon id={app} className="screensaver-face" />;
  return (
    <span className="screensaver-face screensaver-plain" aria-hidden="true">
      <svg
        className="glyphs"
        viewBox={`0 0 ${TILE[0].length} ${TILE.length}`}
        style={{ "--rows": TILE.length, "--cols": TILE[0].length }}
        aria-hidden="true"
      >
        {TILE.flatMap((line, y) => [...line].flatMap((c, x) => (c === "#" ? [[x, y]] : []))).map(([x, y]) => (
          <rect key={`${x}:${y}`} x={x} y={y} width="1" height="1" />
        ))}
      </svg>
    </span>
  );
}

/* SAVER */

function Row({ one, on, flip }) {
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

/* PANE */

export default function Pane({ site }) {
  const wait = known(usePref(KEY.idle));
  const shown = usePref(KEY.clock) !== "off";
  const off = new Set(usePref(KEY.off) ? offs() : []);
  const list = site.savers ?? [];
  const rows = [...new Set(list.map((one) => one.app))].flatMap((app) => list.filter((one) => one.app === app));
  const count = list.filter((one) => !off.has(one.id)).length;
  const many = list.length > 1;
  const flip = (id, on) => savers(on ? [...off].filter((one) => one !== id) : [...off, id]);
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
