import { Figure } from "@mrlyprod/ui/kit/host.tsx";
import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

type Card = NonNullable<Props["prose"]["cards"]>[number];

function Listing({ cards }: { cards: Card[] }) {
  return (
    <div className="listing">
      {cards.map((card) => (
        <a key={card.href} className="icon" href={card.href}>
          {card.figure ? (
            <span className="art">
              <Figure name={card.figure} />
            </span>
          ) : (
            <span className="art sheet" aria-hidden="true" />
          )}
          <span className="name">{card.name}</span>
          {card.text && <span className="text">{card.text}</span>}
        </a>
      ))}
    </div>
  );
}

export function Prose(props: Props["prose"]) {
  const { kind, id, title, level, lead, by, dates = [], figure, html, contents, cards, children, after } = props;
  const Head = level === 2 ? "h2" : "h1";
  if (kind === "lost" || kind === "error")
    return (
      <div className={classes("terminal", "prose", kind)}>
        <div className="alert frame" role="alert">
          <span className="sigil" aria-hidden="true">
            {kind === "lost" ? "?" : "!"}
          </span>
          <div>
            <Head id={id}>{title}</Head>
            {lead && <p className="lead">{lead}</p>}
            {children}
          </div>
        </div>
        {after}
      </div>
    );
  return (
    <article className={classes("terminal", "prose", kind)}>
      <Head id={id}>{title}</Head>
      {(by || dates.length > 0) && <p className="by">{[by, ...dates].filter(Boolean).join(" - ")}</p>}
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
      {figure && <Figure className="opener" name={figure} label={title} />}
      {(lead || children) && (
        <div className="intro">
          {lead && <p className="lead">{lead}</p>}
          {children}
        </div>
      )}
      {html !== undefined && <div className="body" dangerouslySetInnerHTML={{ __html: html }} />}
      {cards && cards.length > 0 && <Listing cards={cards} />}
      {after}
    </article>
  );
}
