import type { Site } from "site:routes";
import type { Saver } from "../../kit/rows.ts";
import { AppIcon } from "../appicon.tsx";
import { app as routed } from "../catalog.ts";
import { KEY, offs, savers } from "../prefs.ts";
import { PLAIN } from "../tiles.ts";
import { Action, Glyph, Item, Nav, Note, Section } from "./ui.tsx";
import { usePref } from "./use.ts";

type RowProps = { one: Saver; on: boolean; flip: (id: string, on: boolean) => void };

export const FIND = ["Screen saver", "Screensaver", "Idle", "Start after", "Show clock", "Shuffle", "Savers"];

const play = (only?: string) => import("../idle.ts").then(({ start }) => start({ only }));

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

export default function Shuffle({ site }: { site: Site }) {
  const off = new Set(usePref(KEY.off) ? offs() : []);
  const list = site.savers ?? [];
  const rows = [...new Set(list.map((one) => one.app))].flatMap((app) => list.filter((one) => one.app === app));
  const count = list.filter((one) => !off.has(one.id)).length;
  const many = list.length > 1;
  const flip = (id: string, on: boolean) => savers(on ? [...off].filter((one) => one !== id) : [...off, id]);
  return (
    <>
      {count > 0 && (
        <Section>
          <Action label="Show the screen saver now" onClick={() => play()}>
            Preview
          </Action>
        </Section>
      )}
      <Section title="Savers">
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
    </>
  );
}
