import { useReducer } from 'react';
import { AppIcon } from '../../shell/appicon.jsx';
import { desk } from '../../shell/desk.js';
import { KEY, magnify, root, screen, write } from '../../shell/prefs.js';
import { kept, WIDGETS } from '../../shell/widgets.jsx';
import { Action, Nav, Pane as Page, Section, Switch, Tiles } from './ui.jsx';
import { usePref } from './use.js';

export const FIND = ['Wallpaper', 'Desktop picture', 'Random', 'Widgets', 'Dock', 'Magnify', 'Reset dock', 'Launchpad', 'Window width', 'Reset window width'];

/* MARK */

const DICE = '#...# ..... ..#.. ..... #...#';
const TILE = '#.#.# .#.#. #.#.# .#.#. #.#.#';

function Mark({ art }) {
  const rows = art.split(' ');
  return (
    <span className="desk-mark">
      <svg className="glyphs" viewBox={`0 0 ${rows[0].length} ${rows.length}`} style={{ '--rows': rows.length, '--cols': rows[0].length }} aria-hidden="true">
        {rows.flatMap((line, y) => [...line].map((c, x) => (c === '#' ? <rect key={`${x}:${y}`} x={x} y={y} width="1" height="1" /> : null)))}
      </svg>
    </span>
  );
}

const Art = ({ children }) => <span className="desk-art">{children}</span>;

/* PICTURE */

const caption = (one) => (one.title && one.title !== one.name ? `${one.name} · ${one.title}` : one.name);

function choices(list) {
  return [
    { value: 'random', text: 'Random', art: <Art><Mark art={DICE} /></Art> },
    ...list.map((one) => ({
      value: one.id,
      text: caption(one),
      art: <Art>{one.app === 'mrly' ? <Mark art={TILE} /> : <AppIcon id={one.app} />}</Art>,
    })),
  ];
}

function chosen(site, stored) {
  const list = site.savers ?? [];
  const head = stored.split('?')[0];
  if (!head) return site.desk ?? 'random';
  if (head === 'random' || list.some((one) => one.id === head)) return head;
  return list.find((one) => one.app === head)?.id ?? 'random';
}

function pick(id) {
  screen(id);
  desk();
}

function Picture({ site }) {
  const stored = usePref(KEY.saver);
  return (
    <Section title="Desktop picture" foot="The desktop is a still frame of a screen saver. Random picks a new one each visit.">
      <Tiles label="Desktop picture" value={chosen(site, stored)} options={choices(site.savers ?? [])} onChange={pick} />
    </Section>
  );
}

/* WIDGETS */

function Widgets() {
  const list = kept(usePref(KEY.widgets));
  const flip = (id, on) => write(KEY.widgets, JSON.stringify(on ? [...list, id] : list.filter((one) => one !== id)));
  return (
    <Section title="Widgets">
      {WIDGETS.map((one) => (
        <Switch key={one.id} label={one.name} value={list.includes(one.id)} onChange={(on) => flip(one.id, on)} />
      ))}
      <Nav label="Edit widgets on the desktop" href="/" />
    </Section>
  );
}

/* DOCK */

function Dock() {
  const on = usePref(KEY.magnify) === '1';
  return (
    <Section title="Dock">
      <Switch label="Magnify" hint="Icons grow under the pointer." value={on} onChange={magnify} />
      <Action label="Reset dock" hint="Put the default apps back in the dock." onClick={() => write(KEY.dock, '')}>
        Reset
      </Action>
      <Nav label="Arrange apps in the Launchpad" href="/menu/" />
    </Section>
  );
}

/* WINDOWS */

function Windows() {
  const page = usePref(KEY.page);
  const wide = usePref(KEY.wide);
  const [, bump] = useReducer((n) => n + 1, 0);
  const style = root().style;
  if (!(page || wide || style.getPropertyValue('--win') || style.getPropertyValue('--wide'))) return null;
  const reset = () => {
    write(KEY.page, '');
    write(KEY.wide, '');
    style.removeProperty('--win');
    style.removeProperty('--wide');
    bump();
  };
  return (
    <Section title="Windows" foot="Drag the edge of a window to resize it.">
      <Action label="Reset window width" hint="Go back to the default width." onClick={reset}>
        Reset
      </Action>
    </Section>
  );
}

/* PANE */

export default function Pane({ site }) {
  return (
    <Page title="Desktop & Dock">
      <Picture site={site} />
      <Widgets />
      <Dock />
      <Windows />
    </Page>
  );
}
