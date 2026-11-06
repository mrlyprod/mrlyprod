import type { ReactNode } from "react";
import { Emoji, Icon } from "../../kit/controls.tsx";
import type { Row } from "../../kit/rows.ts";
import type { EmojiName } from "../../shell/design.ts";
import { EMOJIS, ICONS } from "../../shell/design.ts";
import { Lede } from "../../shell/parts.tsx";
import { Glass, Icons, Theme } from "./look.tsx";
import { Controls, Library } from "./parts.tsx";
import { PIECES } from "./pieces.tsx";

export type KitProps = { row: Pick<Row, "title" | "lead"> };

const TYPE = [1, 2, 3, 4, 5, 6, 7];

const CORNERS = ["--r-window", "--r-panel", "--r-control", "--r-inset", "--round"];

const FACES = [
  ["--sans", "Sans"],
  ["--serif", "Serif"],
  ["--mono", "Mono"],
];

const MARKS = ["proved", "verified", "conjecture", "refuted"];

function Scale() {
  return (
    <div className="lanes">
      <div className="type">
        {TYPE.map((n) => (
          <p key={n} style={{ fontSize: `var(--t${n})` }}>
            <code>{`--t${n}`}</code>
            The quick brown fox
          </p>
        ))}
        {FACES.map(([face, name]) => (
          <p key={face} style={{ fontFamily: `var(${face})` }}>
            <code>{face}</code>
            {name}
          </p>
        ))}
      </div>
      <div className="line">
        {CORNERS.map((name) => (
          <div key={name} className="corner" style={{ borderRadius: `var(${name})` }}>
            <code>{name}</code>
          </div>
        ))}
      </div>
      <div className="line">
        <kbd>R</kbd>
        <kbd>Esc</kbd>
        <kbd>Shift</kbd>
        <kbd>Z</kbd>
        <span className="chip">chip</span>
        {MARKS.map((mark) => (
          <span key={mark} className={`chip ${mark}`}>
            {mark}
          </span>
        ))}
      </div>
    </div>
  );
}

const drawn = (name: string, art: ReactNode) => (
  <figure key={name}>
    {art}
    <figcaption>{name}</figcaption>
  </figure>
);

const SECTIONS = [
  { id: "theme", title: "Theme and tint", body: <Theme /> },
  { id: "glass", title: "Glass", body: <Glass /> },
  { id: "icons", title: "Icon styles", body: <Icons /> },
  {
    id: "glyphs",
    title: "Icons",
    body: <div className="drawn">{ICONS.map((name) => drawn(name, <Icon name={name} size={24} />))}</div>,
  },
  {
    id: "emojis",
    title: "Emojis",
    body: (
      <div className="drawn">
        {(Object.keys(EMOJIS) as EmojiName[]).map((name) => drawn(name, <Emoji name={name} size={32} />))}
      </div>
    ),
  },
  { id: "controls", title: "Controls", body: <Controls /> },
  { id: "library", title: "Library", body: <Library /> },
  { id: "type", title: "Type and shape", body: <Scale /> },
];

const ALL = [...SECTIONS, ...PIECES];

export function Kit({ row }: KitProps) {
  return (
    <div className="kit">
      <Lede id="kit" title={row.title} lead={row.lead}>
        <nav className="jump" aria-label="Sections">
          {ALL.map(({ id, title }) => (
            <a key={id} className="chip" href={`#${id}`}>
              {title}
            </a>
          ))}
        </nav>
      </Lede>
      {ALL.map(({ id, title, body }) => (
        <section key={id} aria-labelledby={id}>
          <h2 id={id}>{title}</h2>
          {body}
        </section>
      ))}
    </div>
  );
}
