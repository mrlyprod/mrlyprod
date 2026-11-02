import { classes } from "../../lib/app/room.ts";
import { Figure } from "../../lib/host.tsx";
import type { Props } from "../../shell/design.ts";
import { Glyph } from "./glyphs.tsx";

/* CARDS */

type Card = NonNullable<Props["prose"]["cards"]>[number];

const pairs = (cards: Card[]) =>
  Array.from({ length: Math.ceil(cards.length / 2) }, (_, n) => cards.slice(2 * n, 2 * n + 2));

function Cards({ cards }: { cards: Card[] }) {
  return (
    <table className="cards">
      <tbody>
        {pairs(cards).map((row) => (
          <tr key={row[0]!.href}>
            {row.map((card) => (
              <td key={card.href}>
                {card.figure && <Figure className="thumb" name={card.figure} />}
                <a href={card.href}>{card.name}</a>
                {card.text && <p>{card.text}</p>}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}

/* PROSE */

export function Prose(props: Props["prose"]) {
  const { kind, id, title, lead, by, dates = [], figure, html, contents, cards, children, after } = props;
  if (kind === "lost" || kind === "error")
    return (
      <div className={classes("classic", "prose", kind)}>
        <div className="raised alert" role="alert">
          <Glyph art="warn" size={32} />
          <div>
            <h1 id={id}>{title}</h1>
            {lead && <p className="lead">{lead}</p>}
            {children}
          </div>
        </div>
        {after}
      </div>
    );
  return (
    <article className={classes("classic", "prose", kind)}>
      {contents.length > 0 && (
        <ul className="contents">
          {contents.map((one) => (
            <li key={one.id} className={`level${one.level}`}>
              <a href={`#${one.id}`} aria-current={one.on ? "location" : undefined}>
                {one.text}
              </a>
            </li>
          ))}
        </ul>
      )}
      <h1 id={id}>{title}</h1>
      {(by || dates.length > 0) && <p className="by">{[by, ...dates].filter(Boolean).join(" - ")}</p>}
      {figure && <Figure className="opener" name={figure} label={title} />}
      {lead && <p className="lead">{lead}</p>}
      {children}
      {html !== undefined && <div className="body" dangerouslySetInnerHTML={{ __html: html }} />}
      {cards && cards.length > 0 && <Cards cards={cards} />}
      {after}
    </article>
  );
}
