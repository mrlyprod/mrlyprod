import { Figure } from "@mrlyprod/ui/kit/host.tsx";
import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Glyph } from "../own/glyphs.tsx";

type Card = NonNullable<Props["prose"]["cards"]>[number];

function Icons({ cards }: { cards: Card[] }) {
  return (
    <div className="sunken icons">
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
  const { kind, id, title, level, lead, figure, html, cards, children, after } = props;
  const Head = level === 2 ? "h2" : "h1";
  if (kind === "lost" || kind === "error")
    return (
      <div className={classes("prose", kind)}>
        <div className="raised alert" role="alert">
          <Glyph art="warn" size={32} />
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
    <article className={classes("prose", kind)}>
      <Head id={id}>{title}</Head>
      {figure && <Figure className="opener" name={figure} label={title} />}
      {(lead || children) && (
        <div className="intro">
          {lead && <p className="lead">{lead}</p>}
          {children}
        </div>
      )}
      {html !== undefined && <div className="body" dangerouslySetInnerHTML={{ __html: html }} />}
      {cards && cards.length > 0 && <Icons cards={cards} />}
      {after}
    </article>
  );
}
