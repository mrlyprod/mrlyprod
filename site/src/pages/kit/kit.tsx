import type { Row } from "../../../scripts/build.ts";
import { Lede } from "../../shell/parts.tsx";
import { Glass, Icons, Theme } from "./look.tsx";
import { Controls, Designs, Library } from "./parts.tsx";

/* TYPES */

export type KitProps = { row: Pick<Row, "title" | "lead"> };

/* SCALE */

const TYPE = [1, 2, 3, 4, 5, 6, 7];

const CORNERS = ["--r-window", "--r-panel", "--r-control", "--r-inset", "--round"];

const FACES = [
  ["--sans", "Noto Sans"],
  ["--serif", "Noto Serif"],
  ["--mono", "Noto Sans Mono"],
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

/* KIT */

const SECTIONS = [
  { id: "theme", title: "Theme and tint", body: <Theme /> },
  { id: "glass", title: "Glass", body: <Glass /> },
  { id: "icons", title: "Icon styles", body: <Icons /> },
  { id: "controls", title: "Controls", body: <Controls /> },
  { id: "library", title: "Library", body: <Library /> },
  { id: "designs", title: "Design picker", body: <Designs /> },
  { id: "type", title: "Type and shape", body: <Scale /> },
];

export function Kit({ row }: KitProps) {
  return (
    <div className="kit">
      <Lede id="kit" title={row.title} lead={row.lead}>
        <nav className="jump" aria-label="Sections">
          {SECTIONS.map(({ id, title }) => (
            <a key={id} className="chip" href={`#${id}`}>
              {title}
            </a>
          ))}
        </nav>
      </Lede>
      {SECTIONS.map(({ id, title, body }) => (
        <section key={id} aria-labelledby={id}>
          <h2 id={id}>{title}</h2>
          {body}
        </section>
      ))}
    </div>
  );
}
